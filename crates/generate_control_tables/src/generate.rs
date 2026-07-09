use crate::parse::{ControlTableRow, IndirectBlock, IndirectKind, ModelGroup, SeparatedTable};
use dynamixel_registers::Register;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
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

    // Dispatch the per-group indirect block layouts, backing the dynamic
    // `ControlTable::indirect_address_blocks` / `indirect_data_blocks` methods.
    for (fn_name, method, kind) in [
        (
            "indirect_address_blocks_from_model_group",
            "indirect_address_blocks",
            IndirectKind::Address,
        ),
        (
            "indirect_data_blocks_from_model_group",
            "indirect_data_blocks",
            IndirectKind::Data,
        ),
    ] {
        writeln!(mod_file)?;
        writeln!(mod_file, r#"#[cfg(feature = "std")]"#)?;
        writeln!(mod_file, "pub(crate) fn {fn_name}(model_group: &dynamixel_registers::models::ModelGroup) -> &'static [dynamixel_registers::IndirectRange] {{")?;
        writeln!(mod_file, "    use dynamixel_registers::models::ModelGroup;")?;
        writeln!(mod_file, "    match model_group {{")?;
        for group in all_models {
            let has = group.indirect().iter().any(|b| b.kind == kind);
            for alias in group.alias().keys() {
                if has {
                    writeln!(
                        mod_file,
                        "        ModelGroup::{alias} => {alias}::{method}(),"
                    )?;
                } else {
                    writeln!(mod_file, "        ModelGroup::{alias} => &[],")?;
                }
            }
        }
        writeln!(mod_file, "        _ => &[],")?;
        writeln!(mod_file, "    }}")?;
        writeln!(mod_file, "}}")?;
    }
    Ok(())
}

/// Emit the resolution lookups into `mod.rs`: `position_resolution(model)` for the exact
/// pulses-per-revolution of a model, and `position_resolution_from_model_group(group)`
/// returning `Some` only when every model in the group agrees (the Y series does not).
pub fn create_resolution(
    mod_path: &PathBuf,
    resolutions: &BTreeMap<dynamixel_registers::models::Model, u32>,
) -> anyhow::Result<()> {
    use dynamixel_registers::models::ModelGroup;

    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path)?;

    writeln!(mod_file)?;
    writeln!(
        mod_file,
        "/// The position encoder resolution in pulses per revolution for a model, from the ROBOTIS docs."
    )?;
    writeln!(
        mod_file,
        "pub fn position_resolution(model: dynamixel_registers::models::Model) -> u32 {{"
    )?;
    writeln!(mod_file, "    use dynamixel_registers::models::Model;")?;
    writeln!(mod_file, "    match model {{")?;
    for (model, res) in resolutions {
        writeln!(mod_file, "        Model::{model} => {res},")?;
    }
    writeln!(
        mod_file,
        r#"        _ => panic!("no resolution for model {{model}}"),"#
    )?;
    writeln!(mod_file, "    }}")?;
    writeln!(mod_file, "}}")?;

    // A group has one shared resolution only if all its members agree.
    let mut by_group: BTreeMap<ModelGroup, BTreeSet<u32>> = BTreeMap::new();
    for (model, res) in resolutions {
        by_group
            .entry(model.model_group())
            .or_default()
            .insert(*res);
    }

    writeln!(mod_file)?;
    writeln!(
        mod_file,
        "/// The shared position resolution of a model group, or `None` if its models disagree (the Y series)."
    )?;
    writeln!(mod_file, r#"#[cfg(feature = "std")]"#)?;
    writeln!(mod_file, "pub(crate) fn position_resolution_from_model_group(model_group: &dynamixel_registers::models::ModelGroup) -> Option<u32> {{")?;
    writeln!(mod_file, "    use dynamixel_registers::models::ModelGroup;")?;
    writeln!(mod_file, "    match model_group {{")?;
    for (group, values) in &by_group {
        match values.iter().exactly_one() {
            Ok(res) => writeln!(mod_file, "        ModelGroup::{group} => Some({res}),")?,
            Err(_) => writeln!(mod_file, "        ModelGroup::{group} => None,")?,
        }
    }
    writeln!(mod_file, "        _ => None,")?;
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
    let macro_names = struct_names
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(file, "model![{macro_names} => {{")?;
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

/// Emit `impl <Name> { indirect_address_blocks / indirect_data_blocks }` returning the
/// contiguous (address, byte length) runs of each indirect family.
fn emit_indirect_impls(
    file: &mut File,
    struct_names: &str,
    indirect: &[IndirectBlock],
) -> anyhow::Result<()> {
    if indirect.is_empty() {
        return Ok(());
    }
    for name in struct_names.split_whitespace() {
        writeln!(file)?;
        writeln!(file, "impl {name} {{")?;
        for block in indirect {
            let (method, human) = match block.kind {
                IndirectKind::Address => ("indirect_address", "Indirect Address"),
                IndirectKind::Data => ("indirect_data", "Indirect Data"),
            };
            writeln!(
                file,
                "    /// Contiguous {human} runs as `(address, byte length)` — sync-usable windows."
            )?;
            writeln!(
                file,
                "    pub const fn {method}_blocks() -> &'static [crate::IndirectRange] {{"
            )?;
            writeln!(file, "        const BLOCKS: &[crate::IndirectRange] = &[")?;
            for seg in &block.segments {
                let bytes = (seg.last_index - seg.first_index + 1) * block.size;
                writeln!(
                    file,
                    "            crate::IndirectRange::new({}, {bytes}),",
                    seg.base,
                )?;
            }
            writeln!(file, "        ];")?;
            writeln!(file, "        BLOCKS")?;
            writeln!(file, "    }}")?;
        }
        writeln!(file, "}}")?;
    }
    Ok(())
}

/// Create `<dir>/<stem>.rs` containing the model table, and register it in `mod.rs`.
fn write_table_file(
    mod_path: impl AsRef<Path>,
    dir: impl AsRef<Path>,
    stem: &str,
    struct_names: &str,
    table: &BTreeMap<Register, ControlTableRow>,
    indirect: &[IndirectBlock],
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
    emit_indirect_impls(&mut file, struct_names, indirect)?;

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
        model.indirect(),
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
        &separated.indirect,
    )
}
