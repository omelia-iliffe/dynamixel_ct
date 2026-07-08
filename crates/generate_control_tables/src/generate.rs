use crate::parse::{ControlTableRow, ModelGroup, SeparatedTable};
use dynamixel_registers::Register;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn mod_path_header(mod_path: &PathBuf) -> anyhow::Result<()> {
    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path)?;
    writeln!(
        mod_file,
        "

//! The Control Tables for all supported models.
//! These structs can be used with `no_std`

            
        "
    )?;

    Ok(())
}

pub fn create_match(mod_path: &PathBuf, all_models: &[ModelGroup]) -> anyhow::Result<()> {
    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path)?;

    writeln!(mod_file)?;
    writeln!(mod_file, r#"#[cfg(feature = "std")]"#)?;
    writeln!(mod_file, "pub(crate) fn control_table_from_model_group(model_group: &dynamixel_registers::models::ModelGroup) -> &'static std::collections::HashMap<dynamixel_registers::Register, dynamixel_registers::RegisterData> {{")?;
    writeln!(mod_file, "    use dynamixel_registers::models::ModelGroup;")?;
    writeln!(mod_file, "    match model_group {{")?;
    for group in all_models {
        for alias in group.alias().keys() {
            writeln!(
                mod_file,
                "        ModelGroup::{} => {}::table(),",
                alias, alias,
            )?;
        }
    }
    writeln!(mod_file, r#"        _ => panic!("unknown model group")"#)?;
    writeln!(mod_file, "    }}")?;
    writeln!(mod_file, "}}")?;
    Ok(())
}

/// Write a `model![<struct_names> => { .. }]` file for the given control table.
/// `struct_names` is the space-separated list of structs to define (one for a per-variant
/// table, several for a shared model-group table).
fn emit_model_file(
    file: &mut File,
    struct_names: &str,
    table: &BTreeMap<Register, ControlTableRow>,
) -> anyhow::Result<()> {
    writeln!(file, "//! Dynamixel {struct_names} model definitions.")?;
    writeln!(file)?;
    writeln!(file, "use crate::model;")?;
    writeln!(file)?;
    writeln!(file)?;
    writeln!(file, "model![{struct_names} => {{")?;
    for row in table.values() {
        let unit = match row.unit {
            Some(u) => format!("Some(UnitScale::new(Unit::{:?}, {}f32))", u.unit, u.scale),
            None => "None".to_string(),
        };
        writeln!(
            file,
            "    {}: {}, {}, Access::{:?}, Area::{:?}, {},",
            row.data_name, row.address, row.size, row.access, row.area, unit,
        )?;
    }
    writeln!(file, "}}];")?;
    Ok(())
}

/// Create `<dir>/<stem>.rs` containing the model table, and register it in `mod.rs`.
fn write_table_file(
    mod_path: impl AsRef<Path>,
    dir: impl AsRef<Path>,
    stem: &str,
    struct_names: &str,
    table: &BTreeMap<Register, ControlTableRow>,
) -> anyhow::Result<()> {
    let dir = dir.as_ref();
    fs::create_dir_all(dir)?;
    let file_path = dir.join(format!("{stem}.rs"));
    println!("writing {struct_names} to file {}", file_path.display());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)?;
    emit_model_file(&mut file, struct_names, table)?;

    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path.as_ref())?;
    writeln!(mod_file, "mod {stem};")?;
    writeln!(mod_file, "pub use {stem}::*;")?;
    Ok(())
}

pub fn write_file_model_group(
    mod_path: impl AsRef<Path>,
    dir: impl AsRef<Path>,
    model: &ModelGroup,
) -> anyhow::Result<()> {
    let struct_names = model.alias().keys().join(" ");
    write_table_file(
        mod_path,
        dir,
        &model.file_name(),
        &struct_names,
        model.table(),
    )
}

/// Write a standalone per-variant table (e.g. `XH430V`) with its exact units.
pub fn write_separated_table(
    mod_path: impl AsRef<Path>,
    dir: impl AsRef<Path>,
    separated: &SeparatedTable,
) -> anyhow::Result<()> {
    write_table_file(
        mod_path,
        dir,
        &separated.name.to_lowercase(),
        &separated.name,
        &separated.table,
    )
}
