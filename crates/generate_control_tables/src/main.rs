mod generate;
mod parse;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use parse::ModelGroup;
use std::fs;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    if Path::new("emanual").exists().not() {
        clone_emanual()?
    }

    let dirs = [
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
            parse::parse_table(file).with_context(|| anyhow!("error parsing {:?}", file))
        })
        .try_collect()?;

    let mut all_models: Vec<ModelGroup> = Vec::new();

    for m in models {
        if let Some(mg) = all_models.iter_mut().find(|mg| {
            println!("comparing {} with {}", mg.name(), m.model);
            mg.table() == &m.table
        }) {
            mg.insert_model(m.model);
        } else {
            let mut mg = ModelGroup::new(m.table);
            mg.insert_model(m.model);
            all_models.push(mg);
        }
    }

    for mg in &all_models {
        println!("model_group: {:?}", mg.name());
    }

    println!("total model groups {}", all_models.len());

    let generate_path: PathBuf = "crates/dynamixel_ct/src/models/".into();
    fs::remove_dir_all(&generate_path).ok();
    let mod_path = generate_path.join("mod.rs");

    fs::create_dir_all(&generate_path)?;
    generate::mod_path_header(&mod_path)?;

    all_models.clone().into_iter().try_for_each(|model| {
        let path = generate_path.join(format!("{}.rs", model.file_name()));

        generate::write_file_model_group(&mod_path, &path, model)?;
        anyhow::Ok(())
    })?;

    generate::create_match(&mod_path, all_models)?;

    let mut fmt = Command::new("cargo").arg("fmt").spawn()?;
    if !fmt.wait()?.success() {
        panic!("cargo fmt failed")
    }

    Ok(())
}

fn filter_files(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref().to_str().unwrap();
    let filter = ["test", "xl320", "2x", "xw430", "x.md", "dxl_p.md", "y.md"];
    filter.iter().any(|f| path.contains(f)).not()
}

fn clone_emanual() -> Result<()> {
    let clone = Command::new("git")
        .args([
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
