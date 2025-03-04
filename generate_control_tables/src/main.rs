use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use dynamixel_ct::models::Model as DModel;
use dynamixel_ct::Register;
use itertools::Itertools;
use num_traits::FromPrimitive;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeSet, };
use std::fs;
use std::io::Write;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

struct ModelGroup {
    name: String,
    model: Vec<DModel>,
    table: BTreeSet<ControlTableRow>,
}

fn main() -> Result<()> {
    if std::path::Path::new("emanual").exists().not() {
        clone_emanual()?
    }

    let dirs = vec![
        "emanual/docs/en/dxl/x",
        "emanual/docs/en/dxl/y",
        "emanual/docs/en/dxl/p",
    ];

    let all_model_files: Vec<_> = dirs.iter().flat_map(collect_model_files).try_collect()?;

    let models: Vec<_> = all_model_files
        .iter()
        .filter(|f| filter_files(f))
        .map(|file| {
            println!("parsing table {}", file.display());
            parse_table(&file).with_context(|| anyhow!("error parsing {:?}", file))
        })
        .try_collect()?;

    let mut all_models: Vec<ModelGroup> = Vec::new();

    for m in &models {
        if let Some(mg) = all_models.iter_mut().find(|mg| mg.table == m.table) {
            mg.model.push(m.model)
        } else {
            all_models.push(ModelGroup {
                name: m.name.clone(),
                model: vec![m.model],
                table: m.table.clone(),
            })
        }
    }

    for mg in &all_models {
        println!("model_group: {:?}", mg.model);
    }

    println!("total model groups {}", all_models.len());

    let generate_path: PathBuf = "src/models/generated/".into();
    let mod_path = generate_path.join("mod.rs");

    if fs::exists(&mod_path)? {
        fs::remove_file(&mod_path)?;
    }

    all_models.into_iter().try_for_each(|model| {
        let path = generate_path.join(format!("{}.rs", model.name));

        write_file_model_group(&mod_path, &path, model)?;
        anyhow::Ok(())
    })?;

    Ok(())
}

fn filter_files(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref().to_str().unwrap();
    let filter = ["test", "xl320", "2x", "xw430", "x.md", "dxl_p.md", "y.md"];
    filter.iter().any(|f| path.contains(f)).not()
}

fn clone_emanual() -> Result<()> {
    let clone = Command::new("git")
        .args(&[
            "clone",
            "https://github.com/ROBOTIS-GIT/emanual.git",
            "--depth",
            "1",
        ])
        .spawn()
        .context("failed to spawn git clone")?
        .wait()
        .context("failed to wait on git clone")?;
    if clone.success().not() {
        Err(anyhow!("failed to clone repo"))
    } else {
        Ok(())
    }
}

fn collect_model_files(
    dir: impl AsRef<str>,
) -> impl Iterator<Item = Result<PathBuf, walkdir::Error>> {
    let r = walkdir::WalkDir::new(dir.as_ref())
        .min_depth(1)
        .into_iter()
        .map_ok(|d| d.into_path());
    r
}

#[derive(Debug, Clone, Eq, Ord)]
struct ControlTableRow {
    address: u16,
    size: u16,
    data_name: Register,
    access: String,
    initial_value: Option<i32>,
    range: String,
    unit: String,
    area: String,
}

impl PartialEq for ControlTableRow {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
            && self.size == other.size
            && self.data_name == other.data_name
            && self.access == other.access
    }
}

impl PartialOrd for ControlTableRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.address.cmp(&other.address))
    }
}

impl ControlTableRow {
    fn parse(header: &str, row: &str, area: Option<&str>) -> Result<Option<ControlTableRow>> {
        let mut cells = header
            .split("|")
            .zip(row.split("|"))
            .skip(1) // remove first empty cell
            .collect_vec();
        _ = cells.pop(); //remove last empty cell

        let find = |pattern| -> Option<String> {
            cells.iter().find_map(|(header, cell)| {
                header
                    .to_lowercase()
                    .contains(pattern)
                    .then_some(cell.trim().to_string())
            })
        };

        let address = find("address").unwrap();
        let size = find("size").unwrap();
        let data_name = find("data").unwrap();
        let access = find("access").unwrap();
        let initial_value = find("initial").unwrap();
        let range = find("range").unwrap();
        let unit = find("unit").unwrap();
        let area = find("area")
            .or_else(|| area.map(|s| s.to_string()))
            .ok_or(anyhow!("missing area"))?;

        let mut data_name = Regex::new(r"\[(.+)]")
            .unwrap()
            .captures(&data_name)
            .context(anyhow!("failed to parse data name: {}", &data_name))?
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        if data_name.contains("(") {
            let re = Regex::new(r"\(.*\)").expect("tested");
            data_name = re.replace(data_name.as_str(), "").to_string();
        }

        let data_name = match data_name.to_string().to_case(Case::Pascal).parse() {
            Ok(data_name) => data_name,
            Err(e) => {
                println!("error parsing {}: {}", data_name, e);
                return Ok(None);
            }
        };
        let initial_value = (initial_value.is_empty()
            || initial_value.contains("-")
            || initial_value.contains("br"))
        .not()
        .then(|| {
            let initial_value = initial_value.replace(",", "");
            initial_value
                .parse()
                .with_context(|| anyhow!("failed to parse initial value: {}", initial_value))
        })
        .transpose()?;
        // println!("parsed {}", data_name);
        Ok(Some(Self {
            address: address
                .parse()
                .with_context(|| anyhow!("failed to parse address {}", address))?,
            size: size
                .parse()
                .with_context(|| anyhow!("failed to parse size {}", size))?,
            data_name,
            access,
            initial_value,
            range,
            unit,
            area,
        }))
    }

    fn to_macro_line(self) -> String {
        let comment = format!(
            "access: {} initial_value: {:?} area: {}",
            self.access, self.initial_value, self.area
        );
        format!(
            "{}: {}, {}, \"{}\",\n",
            self.data_name, self.address, self.size, comment
        )
    }
}

#[derive(Debug, Clone)]
struct Model {
    name: String,
    model: dynamixel_ct::models::Model,
    model_number: u16,
    table: BTreeSet<ControlTableRow>,
}

fn to_model_macro(name: String, table: BTreeSet<ControlTableRow>) -> String {
    let mut output = format!(
        "//! Dynamixel XM430 model definitions.\n\
            \n\
            use crate::model;\n\
            \n\
            model![{} {{\n",
        name.to_case(Case::Pascal),
    );

    for eeprom in table {
        output.push_str(&format!("\t{}", &eeprom.to_macro_line()));
    }

    output.push_str(
        "}\
        ];\
        ",
    );
    output
}

fn to_model_macro_from_group(
    name: String,
    models: Vec<DModel>,
    table: BTreeSet<ControlTableRow>,
) -> String {
    let model_string = models.iter().map(|m| format!("Model::{}", m)).join(", ");

    let mut output = format!(
        "//! Dynamixel XM430 model definitions.\n\
            \n\
            use crate::model;\n\
            use crate::models::Model;\n\

            const MODELS: [Model; {}] = [{}];\n\

            model![{} {{\n",
        models.len(),
        model_string,
        name.to_case(Case::Pascal),
    );

    for eeprom in table {
        output.push_str(&format!("\t{}", &eeprom.to_macro_line()));
    }

    output.push_str(
        "}\
        ];\
        ",
    );
    output
}

fn parse_table(model_file: impl AsRef<Path>) -> Result<Model> {
    let model_file = model_file.as_ref();
    let file_name = model_file
        .file_name()
        .ok_or(anyhow!("no file name"))?
        .to_str()
        .ok_or(anyhow!("error parsing file name"))?;
    let file = fs::read_to_string(model_file)?;

    let parse_table = |start: &str,
                       area: Option<&str>|
     -> anyhow::Result<BTreeSet<ControlTableRow>> {
        let (start, _) = file
            .lines()
            .find_position(|p| p.to_lowercase().contains(&start.to_lowercase()))
            .ok_or(anyhow!("cannot find {} table", start))?;
        let mut lines = file.lines();
        let table = lines.by_ref().skip(start).skip_while(|l| !l.contains("|"));

        let mut table = table.take_while(|l| l.contains("|"));
        let header = table.next().unwrap();
        // println!("header: {}", header);
        table
            .skip(1)
            .filter(|r| {
                !r.contains("…") && !r.contains("···") && !r.contains("...") && !r.contains("N/A")
            })
            .flat_map(|r| {
                ControlTableRow::parse(header, r, area)
                    .with_context(|| anyhow!("failed to parse row {}", r))
                    .transpose()
            })
            .try_collect()
    };

    let try_double_table =
        || -> anyhow::Result<(BTreeSet<ControlTableRow>, BTreeSet<ControlTableRow>)> {
            let eeprom = parse_table("Control Table of EEPROM Area", Some("EEPROM"))?;

            let ram = parse_table("Control Table of RAM Area", Some("RAM"))?;
            Ok((eeprom, ram))
        };

    let table = match try_double_table() {
        Err(e) => parse_table("Control Table", None)
            .with_context(|| e)
            .with_context(|| anyhow!("failed to parse double table and single table"))?,
        Ok((mut eeprom, mut ram)) => {
            eeprom.append(&mut ram);
            eeprom
        }
    };

    let model_number = table
        .iter()
        .find(|t| t.data_name == Register::ModelNumber)
        .expect("can't find modelNumber");
    let model_number = model_number
        .initial_value
        // .ok_or_else(|| anyhow!("no initial model number {:?}", model_number))?
        .unwrap_or_default() as u16;
    let name = file_name
        .split(".")
        .nth(0)
        .unwrap()
        .to_string()
        .to_uppercase()
        .replace("-", "_");
    let model = DModel::from_str(&name).unwrap_or(
        DModel::from_u16(model_number)
            .ok_or_else(|| anyhow!("cannot find model for {} = {},", name, model_number))?,
    );
    let model = Model {
        name,
        model,
        model_number,
        table,
    };

    Ok(model)
}

fn write_file(mod_path: impl AsRef<Path>, file_path: impl AsRef<Path>, model: Model) -> Result<()> {
    let file_path = file_path.as_ref();
    let folder = file_path.parent().unwrap();
    fs::create_dir_all(folder)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    println!(
        "writing model {} to file {}",
        model.name,
        file_path.display()
    );
    file.write_all(to_model_macro(model.name, model.table).as_bytes())?;

    let mod_path = mod_path.as_ref();

    let folder = mod_path.parent().unwrap();
    fs::create_dir_all(folder)?;
    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path)?;
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    mod_file.write_all(format!("mod {};\n", file_name).as_bytes())?;

    Ok(())
}

fn write_file_model_group(
    mod_path: impl AsRef<Path>,
    file_path: impl AsRef<Path>,
    model: ModelGroup,
) -> Result<()> {
    let file_path = file_path.as_ref();
    let folder = file_path.parent().unwrap();
    fs::create_dir_all(folder)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    println!(
        "writing model {} to file {}",
        model.name,
        file_path.display()
    );
    file.write_all(to_model_macro_from_group(model.name, model.model, model.table).as_bytes())?;

    let mod_path = mod_path.as_ref();

    let folder = mod_path.parent().unwrap();
    fs::create_dir_all(folder)?;
    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path)?;
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    mod_file.write_all(format!("mod {};\n", file_name).as_bytes())?;

    Ok(())
}
