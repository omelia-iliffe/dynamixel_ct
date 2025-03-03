use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::io::{Read, Write};
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::process::Command;

const VALID_SERIES: [&str; 3] = ["x", "y", "p"];

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
    // let mut all_model_files = collect_model_files("emanual/docs/en/dxl/x")
    // all_model_files.append(&mut collect_model_files("emanual/docs/en/dxl/y")?);
    // all_model_files.append(&mut collect_model_files("emanual/docs/en/dxl/y")?);

    let models: Vec<_> =
        all_model_files
        .iter()
        .filter(|f| filter_files(f))
        .map(|file| {
            println!("parsing table {}", file.display());
            parse_table(&file).with_context(|| anyhow!("error parsing {:?}", file))
        }).try_collect()?;



    let generate_path: PathBuf = "src/models/generated/".into();
    let mod_path = generate_path.join("mod.rs");

    if fs::exists(&mod_path)? {
        fs::remove_file(&mod_path)?;
    }

    models.into_iter().try_for_each(|model| {
        let path = generate_path.join(format!("{}.rs", model.name));

        write_file(&mod_path, &path, model)?;
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

#[derive(Debug, Clone)]
struct ControlTableRow {
    address: u16,
    size: u16,
    data_name: String,
    access: String,
    initial_value: Option<i32>,
    range: String,
    unit: String,
    area: String,
}

impl ControlTableRow {
    const CELL_NUM: usize = 7;
    fn parse(header: &str, row: &str, area: Option<&str>) -> Result<ControlTableRow> {
        let mut cells = header
            .split("|")
            .zip(row.split("|"))
            .skip(1)
            // .filter(|c| !c.is_empty())
            // .map(|(header, cell)| (header.to_lowercase(), cell.to_string()))
            .collect_vec();
        _ = cells.pop();
        // if cells.len() != Self::CELL_NUM {
        //     return Err(anyhow!("cells in {:?} is not {}", cells, Self::CELL_NUM));
        // }
        // let [address, size, data_name, access, initial_value, range, unit]: [String; Self::CELL_NUM] =
        //     cells.try_into().unwrap();

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
            .unwrap();

        let mut data_name = Regex::new(r"\[(.+)]")
            .unwrap()
            .captures(&data_name)
            .context(anyhow!("failed to parse data name: {}", &data_name))?
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        if data_name.contains("(") {
            let re = Regex::new(r"\(.*\)").unwrap();
            data_name = re.replace(data_name.as_str(), "").to_string();
        }

        let data_name = data_name.to_string().to_case(Case::Pascal);
        let initial_value = (initial_value.is_empty() || initial_value.contains("-") || initial_value.contains("br"))
            .not()
            .then(|| {
                let initial_value = initial_value.replace(",", "");
                initial_value
                    .parse()
                    .with_context(|| anyhow!("failed to parse initial value: {}", initial_value))
            })
            .transpose()?;
        // println!("parsed {}", data_name);
        Ok(Self {
            address: address.parse().with_context(|| anyhow!("failed to parse address {}", address))?,
            size: size.parse().with_context(|| anyhow!("failed to parse size {}", size))?,
            data_name,
            access,
            initial_value,
            range,
            unit,
            area,
        })
    }

    fn to_macro_line(self) -> String {
        format!("{}: {}, {},\n", self.data_name, self.address, self.size)
    }
}

#[derive(Debug, Clone)]
struct Model {
    name: String,
    model_number: u16,
    table: Vec<ControlTableRow>,
}

impl Model {
    fn to_model_macro(self) -> String {
        let mut output = format!(
            "//! Dynamixel XM430 model definitions.\n\
            \n\
            use crate::model;\n\
            \n\
            model![{} {{\n",
            self.name,
        );

        for eeprom in self.table {
            output.push_str(&format!("\t{}", &eeprom.to_macro_line()));
        }

        output.push_str(
            "}\
        ];\
        ",
        );
        output
    }
}

fn parse_table(model_file: impl AsRef<Path>) -> Result<Model> {
    let model_file = model_file.as_ref();
    let file_name = model_file
        .file_name()
        .ok_or(anyhow!("no file name"))?
        .to_str()
        .ok_or(anyhow!("error parsing file name"))?;
    let file = fs::read_to_string(model_file)?;

    let mut parse_table =
        |start: &str, area: Option<&str>| -> anyhow::Result<Vec<ControlTableRow>> {
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
                .filter(|r| !r.contains("…") && !r.contains("···") && !r.contains("...") && !r.contains("N/A"))
                .map(|r| ControlTableRow::parse(header, r, area).with_context(|| anyhow!("failed to parse row {}", r)))
                .try_collect()
        };

    let mut try_double_table =
        || -> anyhow::Result<(Vec<ControlTableRow>, Vec<ControlTableRow>)> {
            let eeprom = parse_table("Control Table of EEPROM Area", Some("EEPROM"))?;

            let ram = parse_table("Control Table of RAM Area", Some("RAM"))?;
            Ok((eeprom, ram))
        };

    let table = match try_double_table() {
        Err(e) => parse_table("Control Table", None)
            .with_context(|| anyhow!("failed to parse double table and single table: {e}"))?,
        Ok((mut eeprom, mut ram)) => {
            eeprom.append(&mut ram);
            eeprom
        }
    };

    let model_number = table
        .iter()
        .find(|t| t.data_name == "ModelNumber")
        .expect("can't find modelNumber");
    let model_number = model_number
        .initial_value
        // .ok_or_else(|| anyhow!("no initial model number {:?}", model_number))?
        .unwrap_or_default()
        as u16;
    let model = Model {
        name: file_name.split(".").nth(0).unwrap().to_string().to_lowercase().replace("-", "_"),
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
    println!("writing model {} to file {}", model.name, file_path.display());
    file.write_all(model.to_model_macro().as_bytes())?;

    let mod_path = mod_path.as_ref();

    let folder = mod_path.parent().unwrap();
    fs::create_dir_all(folder)?;
    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true).open(mod_path)?;
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    mod_file.write_all(format!("mod {};\n", file_name).as_bytes())?;

    Ok(())
}
