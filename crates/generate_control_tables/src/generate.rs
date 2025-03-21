use std::fs::File;
use std::path::{Path, PathBuf};
use std::fs;
use itertools::Itertools;
use std::io::Write;
use crate::parse::ModelGroup;

pub fn create_match(mod_path: &PathBuf, all_models: Vec<ModelGroup>) -> anyhow::Result<()> {
    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path)?;

    writeln!(mod_file)?;
    writeln!(mod_file, "pub(crate) fn control_table_from_model(model: &dynamixel_registers::models::Model) -> &'static std::collections::HashMap<dynamixel_registers::Register, dynamixel_registers::RegisterData> {{")?;
    writeln!(mod_file, "    use dynamixel_registers::models::Model::*;")?;
    writeln!(mod_file, "    match model {{")?;
    for group in &all_models {
        for (alias, models) in group.alias() {
            writeln!(
                mod_file,
                "        {} => {}::table(),",
                models.iter().map(|m| m.to_string()).join(" | "),
                alias,
            )?;
        }
    }
    writeln!(mod_file, "    }}")?;
    writeln!(mod_file, "}}")?;
    writeln!(mod_file)?;
    writeln!(mod_file, "pub(crate) fn control_table_from_model_group(model_group: &dynamixel_registers::models::ModelGroup) -> &'static std::collections::HashMap<dynamixel_registers::Register, dynamixel_registers::RegisterData> {{")?;
    writeln!(mod_file, "    use dynamixel_registers::models::ModelGroup;")?;
    writeln!(mod_file, "    match model_group {{")?;
    for group in &all_models {
        for alias in group.alias().keys() {
            writeln!(
                mod_file,
                "        ModelGroup::{} => {}::table(),",
                alias,
                alias,
            )?;
        }
    }
    writeln!(mod_file, "    }}")?;
    writeln!(mod_file, "}}")?;
    Ok(())
}

fn to_model_macro_from_group(file: &mut File, model_group: &ModelGroup) -> anyhow::Result<()> {
    writeln!(file, "//! Dynamixel XM430 model definitions.")?;
    writeln!(file)?;
    writeln!(file, "use crate::model;")?;
    writeln!(file)?;

    writeln!(file)?;
    writeln!(file, "model![{} => {{", model_group.alias().keys().join(" "))?;

    for row in model_group.table().values() {
        writeln!(
            file,
            "   {}: {}, {},",
            row.data_name, row.address, row.size,
        )?;
    }

    writeln!(file, "}}];")?;
    Ok(())
}

pub fn write_file_model_group(
    mod_path: impl AsRef<Path>,
    file_path: impl AsRef<Path>,
    model: ModelGroup,
) -> anyhow::Result<()> {
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
        model.table_name(),
        file_path.display()
    );
    to_model_macro_from_group(&mut file, &model)?;

    let mod_path = mod_path.as_ref();

    let folder = mod_path.parent().unwrap();
    fs::create_dir_all(folder)?;
    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path)?;
    writeln!(mod_file, "mod {};", model.file_name())?;
    writeln!(mod_file, "pub use {}::*;", model.file_name())?;

    Ok(())
}