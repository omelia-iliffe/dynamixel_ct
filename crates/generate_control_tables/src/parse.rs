use anyhow::{anyhow, Context};
use convert_case::{Case, Casing};
use dynamixel_registers::models::Model as DModel;
use dynamixel_registers::models::ModelGroup as DModelGroup;
use dynamixel_registers::{Access, Area, Register, Unit, UnitScale};
use itertools::Itertools;
use num_traits::FromPrimitive;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::ops::Not;
use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;

/// Extracts the text of a markdown link: `[Name](#anchor)` -> `Name`.
static LINK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(.+)]").unwrap());
/// Matches a parenthesised qualifier to strip, e.g. the `(Shadow)` in `Secondary(Shadow) ID`.
static PARENS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\(.*\)").unwrap());
/// Splits a unit cell like `0.229 [rev/min]` into the leading scale and the bracketed unit.
static UNIT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*([0-9.]+)?\s*\[\s*([^\]]+?)\s*\]").unwrap());

/// Parse a unit-column cell into a normalised base [`UnitScale`].
///
/// The docs give the unit as `<scale> [<unit>]`, e.g. `2.69 [mA]`. Metric prefixes
/// are folded into the scale so each physical quantity has one base unit
/// (`2.69 [mA]` -> `Ampere`, scale `0.00269`). Cells with no bracketed unit and
/// scale (`-`, `R`, `1/R`) yield `None`; dual-mode cells keep the primary unit.
fn parse_unit(cell: &str) -> Option<UnitScale> {
    let cell = cell.split("<br").next().unwrap_or(cell);
    let cell = cell
        .replace('\\', "")
        .replace("<sup>", "")
        .replace("</sup>", "");
    let caps = UNIT_RE.captures(&cell)?;
    // Fold in f64 and cast once, so e.g. 2.69 mA yields a clean 0.00269 rather than
    // the f32 round-off (0.0026900002) that repeated f32 arithmetic would produce.
    let number: Option<f64> = caps.get(1).and_then(|m| m.as_str().parse().ok());
    let raw = caps.get(2).unwrap().as_str();

    let tok = raw
        .to_lowercase()
        .replace(' ', "")
        .replace(['\u{00b5}', '\u{03bc}'], "u") // µ micro sign / μ greek mu
        .replace('²', "2");

    let (unit, mult) = if raw.contains("deg") || raw.contains('°') || raw.contains('℃') {
        (Unit::DegreesCelsius, 1.0)
    } else {
        match tok.as_str() {
            "pulse" => (Unit::Pulse, 1.0),
            "rev/min" => (Unit::RevPerMinute, 1.0),
            "rev/min2" => (Unit::RevPerMinuteSquared, 1.0),
            "pulse/ms" => (Unit::PulsePerMillisecond, 1.0),
            "pulse/s" => (Unit::PulsePerSecond, 1.0),
            "v" => (Unit::Volt, 1.0),
            "%" => (Unit::Percent, 1.0),
            "ma" => (Unit::Ampere, 1e-3),
            "a" => (Unit::Ampere, 1.0),
            "hz" => (Unit::Hertz, 1.0),
            "usec" | "us" => (Unit::Second, 1e-6),
            "ms" | "msec" => (Unit::Second, 1e-3),
            "sec" | "s" => (Unit::Second, 1.0),
            "mv/msec" | "mv/ms" => (Unit::VoltPerSecond, 1.0),
            other => {
                println!("unknown unit: {:?}", other);
                return None;
            }
        }
    };
    let scale = (number? * mult) as f32;
    Some(UnitScale::new(unit, scale))
}

/// Parse the Access column (`R` / `RW`) into an [`Access`].
fn parse_access(cell: &str) -> Access {
    if cell.to_lowercase().contains('w') {
        Access::Rw
    } else {
        Access::R
    }
}

/// Parse the Area column (`EEPROM` / `RAM`) into an [`Area`], if present.
fn parse_area(cell: &str) -> Option<Area> {
    match cell.to_uppercase().as_str() {
        "EEPROM" => Some(Area::Eeprom),
        "RAM" => Some(Area::Ram),
        "HYBRID" => Some(Area::Hybrid),
        _ => None,
    }
}

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

    /// Whether `other` describes the same control table as this group (same registers,
    /// each row [compatible](ControlTableRow::compatible)).
    pub(crate) fn table_compatible(&self, other: &BTreeMap<Register, ControlTableRow>) -> bool {
        self.table.len() == other.len()
            && self
                .table
                .iter()
                .all(|(reg, row)| other.get(reg).is_some_and(|o| row.compatible(o)))
    }

    /// Fold another (compatible) table in, reconciling each row's unit.
    pub(crate) fn merge(&mut self, other: BTreeMap<Register, ControlTableRow>) {
        for (reg, o) in other {
            if let Some(row) = self.table.get_mut(&reg) {
                row.merge(&o);
            }
        }
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

#[derive(Debug, Clone)]
pub(crate) struct ControlTableRow {
    pub(crate) address: u16,
    pub(crate) size: u16,
    pub(crate) data_name: Register,
    pub(crate) access: Access,
    pub(crate) area: Area,
    initial_value: Option<i32>,
    pub(crate) unit: Option<UnitScale>,
    /// Set once two grouped models disagree on this register's unit; keeps [`Self::unit`]
    /// dropped so a later matching model can't resurrect a conflicted value.
    conflicted: bool,
}

impl ControlTableRow {
    /// Whether two rows describe the same register in a shareable table: identical
    /// address, size, name, access and area. Units are reconciled separately by [`merge`].
    fn compatible(&self, other: &Self) -> bool {
        self.address == other.address
            && self.size == other.size
            && self.data_name == other.data_name
            && self.access == other.access
            && self.area == other.area
    }

    /// Fold another model's row for the same register into this one, keeping a unit only
    /// while every model agrees on it. Order-independent: once conflicted it stays dropped.
    fn merge(&mut self, other: &Self) {
        if self.conflicted {
            return;
        }
        match (self.unit, other.unit) {
            (Some(a), Some(b)) if a != b => {
                self.unit = None;
                self.conflicted = true;
            }
            (None, Some(_)) => self.unit = other.unit,
            _ => {}
        }
    }
}

impl ControlTableRow {
    fn parse(
        header: &str,
        row: &str,
        area: Option<Area>,
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
        fn text_before_markup(cell: String) -> String {
            match cell.split_once('<') {
                Some((before, _)) => before.trim().to_string(),
                None => cell,
            }
        }

        let address = find("address").unwrap();
        let size = find("size").unwrap();
        let data_name = find("data").unwrap();
        let access = parse_access(&find("access").unwrap_or_default());
        // Area comes from the table's own column when present (single-table models),
        // otherwise from the EEPROM/RAM section the row was found in.
        let area = find("area")
            .and_then(|a| parse_area(&a))
            .or(area)
            .unwrap_or(Area::Ram);
        let initial_value = text_before_markup(find("initial").unwrap());
        let unit = parse_unit(&find("unit").unwrap_or_default());

        // Data names are usually markdown links `[Name](#anchor)`, but some rows
        // (e.g. `Model Information`) are plain text, so fall back to the raw cell.
        let mut data_name = LINK_RE
            .captures(&data_name)
            .map(|caps| caps[1].to_string())
            .unwrap_or(data_name);

        if data_name.contains('(') {
            data_name = PARENS_RE.replace(&data_name, "").to_string();
        }

        let data_name = match data_name.to_case(Case::Pascal).parse() {
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
            area,
            initial_value,
            unit,
            conflicted: false,
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
                       area: Option<Area>|
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
            let eeprom = parse_table("Control Table of EEPROM Area", Some(Area::Eeprom))?;

            let ram = parse_table("Control Table of RAM Area", Some(Area::Ram))?;
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

/// A per-variant control table exposed as a standalone struct (e.g. `XH430V`), for
/// models whose exact units are dropped from the shared model-group table.
pub struct SeparatedTable {
    pub name: String,
    pub table: BTreeMap<Register, ControlTableRow>,
}

/// Build the separated (exact-unit) tables: for any model group whose members genuinely
/// disagree on a register's unit, split that group's models by their exact table and emit
/// one entry per partition, named from the models' common prefix (`XH430_V*` -> `XH430V`).
///
/// Groups with no such conflict are skipped — their shared table already carries the
/// correct units, so no extra struct is needed.
pub fn separated_tables(models: &[Model]) -> Vec<SeparatedTable> {
    let mut by_alias: BTreeMap<DModelGroup, Vec<&Model>> = BTreeMap::new();
    for m in models {
        by_alias.entry(m.model.model_group()).or_default().push(m);
    }

    let mut out = Vec::new();
    for alias_models in by_alias.into_values() {
        if !has_unit_conflict(&alias_models) {
            continue;
        }
        // Partition by exact-unit compatibility (equal, or blank on either side).
        let mut subs: Vec<(Vec<DModel>, BTreeMap<Register, ControlTableRow>)> = Vec::new();
        for m in alias_models {
            match subs.iter_mut().find(|(_, t)| unit_compatible(t, &m.table)) {
                Some((members, table)) => {
                    members.push(m.model);
                    for (reg, row) in &m.table {
                        if let Some(existing) = table.get_mut(reg) {
                            existing.merge(row);
                        }
                    }
                }
                None => subs.push((vec![m.model], m.table.clone())),
            }
        }
        for (members, table) in subs {
            out.push(SeparatedTable {
                name: common_prefix_name(&members),
                table,
            });
        }
    }
    out
}

/// Whether any register carries two different (present) units across these models.
fn has_unit_conflict(models: &[&Model]) -> bool {
    let mut seen: BTreeMap<Register, UnitScale> = BTreeMap::new();
    for m in models {
        for (reg, row) in &m.table {
            if let Some(us) = row.unit {
                match seen.get(reg) {
                    Some(prev) if *prev != us => return true,
                    Some(_) => {}
                    None => {
                        seen.insert(*reg, us);
                    }
                }
            }
        }
    }
    false
}

/// Two tables are unit-compatible when they cover the same registers and no register has
/// a *conflicting* unit (a blank on either side is fine); addresses etc. must match.
fn unit_compatible(
    a: &BTreeMap<Register, ControlTableRow>,
    b: &BTreeMap<Register, ControlTableRow>,
) -> bool {
    a.len() == b.len()
        && a.iter().all(|(reg, ra)| {
            b.get(reg).is_some_and(|rb| {
                ra.compatible(rb) && !matches!((ra.unit, rb.unit), (Some(x), Some(y)) if x != y)
            })
        })
}

/// The uppercase common prefix of the model names, underscores removed
/// (`XH430_V210` + `XH430_V350` -> `XH430V`).
fn common_prefix_name(models: &[DModel]) -> String {
    let names: Vec<String> = models.iter().map(|m| m.to_string()).collect();
    let first = &names[0];
    let end = names[1..].iter().fold(first.len(), |end, n| {
        first
            .chars()
            .zip(n.chars())
            .take_while(|(a, b)| a == b)
            .count()
            .min(end)
    });
    first[..end].replace('_', "")
}
