use anyhow::{anyhow, Context};
use convert_case::{Case, Casing};
use dynamixel_registers::models::Model as DModel;
use dynamixel_registers::models::ModelGroup as DModelGroup;
use dynamixel_registers::Register;
use itertools::Itertools;
use num_traits::FromPrimitive;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::ops::Not;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct ModelGroup {
    model: BTreeSet<DModel>,
    table: BTreeMap<Register, ControlTableRow>,
}

impl ModelGroup {
    pub(crate) fn new(table: BTreeMap<Register, ControlTableRow>) -> Self {
        Self {
            table,
            ..Default::default()
        }
    }

    pub(crate) fn insert_model(&mut self, model: DModel) {
        self.model.insert(model);
    }

    pub(crate) fn name(&self) -> String {
        self.alias().keys().join("_")
    }

    pub(crate) fn table(&self) -> &BTreeMap<Register, ControlTableRow> {
        &self.table
    }
    pub(crate) fn table_name(&self) -> String {
        self.name().to_uppercase()
    }

    pub(crate) fn file_name(&self) -> String {
        self.name().to_lowercase()
    }

    pub(crate) fn alias(&self) -> BTreeMap<DModelGroup, Vec<DModel>> {
        self.model.iter().fold(BTreeMap::new(), |mut acc, model| {
            let alias = model.model_group();
            acc.entry(alias).or_default().push(*model);
            acc
        })
    }
}

#[derive(Debug, Clone, Eq)]
#[expect(dead_code)]
pub(crate) struct ControlTableRow {
    pub(crate) address: u16,
    pub(crate) size: u16,
    pub(crate) data_name: Register,
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

impl Ord for ControlTableRow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.address.cmp(&other.address)
    }
}

impl PartialOrd for ControlTableRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.address.cmp(&other.address))
    }
}

impl ControlTableRow {
    fn parse(
        header: &str,
        row: &str,
        area: Option<&str>,
    ) -> anyhow::Result<Option<ControlTableRow>> {
        let mut cells = header
            .split("|")
            .zip(row.split("|"))
            .skip(1) // remove first empty cell
            .collect_vec();
        _ = cells.pop(); //remove last empty cell

        // Match the column by header. Prefer an exact header match, falling back to a
        // substring match. The exact pass is needed because the new docs add a
        // `_Modbus_ Address` column that would otherwise be caught by the `address` lookup.
        let find = |pattern: &str| -> Option<String> {
            cells
                .iter()
                .find(|(header, _)| header.trim().to_lowercase() == pattern)
                .or_else(|| {
                    cells
                        .iter()
                        .find(|(header, _)| header.to_lowercase().contains(pattern))
                })
                .map(|(_, cell)| cell.trim().to_string())
        };

        /// Strip trailing markup from a cell, keeping only the text before the first tag.
        /// Handles weird multi-value cells like `4,030<br />4,031<sup>1)</sup>`.
        fn handle_double_model(cell: String) -> String {
            match cell.split_once('<') {
                Some((before, _)) => before.trim().to_string(),
                None => cell,
            }
        }

        let address = find("address").unwrap();
        let size = find("size").unwrap();
        let data_name = find("data").unwrap();
        let access = find("access").unwrap();
        let initial_value = handle_double_model(find("initial").unwrap());
        let range = find("range").unwrap().replace("<br>", " ").replace(",", "");
        let unit = find("unit").unwrap();
        let area = find("area")
            .or_else(|| area.map(|s| s.to_string()))
            .ok_or(anyhow!("missing area"))?;

        // Data names are usually markdown links `[Name](#anchor)`, but some rows
        // (e.g. `Model Information`) are plain text, so fall back to the raw cell.
        let mut data_name = match Regex::new(r"\[(.+)]").unwrap().captures(&data_name) {
            Some(caps) => caps.get(1).unwrap().as_str().to_string(),
            None => data_name.clone(),
        };

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
}

#[derive(Debug, Clone)]
pub(crate) struct Model {
    pub(crate) model: dynamixel_registers::models::Model,
    pub(crate) table: BTreeMap<Register, ControlTableRow>,
}

pub fn parse_table(model_file: impl AsRef<Path>) -> anyhow::Result<Model> {
    let model_file = model_file.as_ref();
    let file_name = model_file
        .file_name()
        .ok_or(anyhow!("no file name"))?
        .to_str()
        .ok_or(anyhow!("error parsing file name"))?;
    let file = fs::read_to_string(model_file)?;

    let parse_table = |start: &str,
                       area: Option<&str>|
     -> anyhow::Result<BTreeMap<Register, ControlTableRow>> {
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
            .map(|table| table.map(|t| (t.data_name, t)))
            .try_collect()
    };

    let try_double_table =
        || -> anyhow::Result<(BTreeMap<Register, ControlTableRow>, BTreeMap<Register, ControlTableRow>)> {
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

    let (_, model_number) = table
        .iter()
        .find(|(r, _)| r == &&Register::ModelNumber)
        .expect("can't find modelNumber");
    let model_number = model_number
        .initial_value
        // .ok_or_else(|| anyhow!("no initial model number {:?}", model_number))?
        .unwrap_or_default() as u16;
    let name = file_name
        .split(".")
        .next()
        .unwrap()
        .to_string()
        .to_uppercase()
        .replace("-", "_");
    // Resolve by name first (the filename maps directly to a variant), falling back to
    // the model number from the table. Done lazily so a name match still wins if the
    // table's model number ever disagrees with the enum.
    let model = DModel::from_str(&name)
        .ok()
        .or_else(|| DModel::from_u16(model_number))
        .ok_or_else(|| anyhow!("cannot find model for {} = {},", name, model_number))?;
    let model = Model { model, table };

    Ok(model)
}
