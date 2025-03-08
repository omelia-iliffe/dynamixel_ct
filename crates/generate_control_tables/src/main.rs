mod parse;
mod generate;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::process::Command;
use parse::ModelGroup;

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

    for m in &models {
        if let Some(mg) = all_models.iter_mut().find(|mg| {
            println!("comparing {} with {}", mg.name, m.model);
            mg.table == m.table
        }) {
            mg.model.insert(m.model);
        } else {
            let mut model = BTreeSet::new();
            model.insert(m.model);
            all_models.push(ModelGroup {
                name: m.name.clone(),
                model,
                table: m.table.clone(),
                ..Default::default()
            })
        }
    }

    for mg in &all_models {
        println!("model_group: {:?}", mg.model);
    }

    println!("total model groups {}", all_models.len());

    all_models.iter_mut().for_each(|mg| {
        mg.name = match mg.file_name().as_str() {
            "xh540_w270" => "xdh540".to_string(),
            "xc430_w150" => "xcl430".to_string(),
            "xw540_t140" => "xw".to_string(),
            "xl330_m288" => "xcl330".to_string(),
            "xd430_t350" => "xdhm430".to_string(),
            "ym080_230_a051_rh" => "ym".to_string(),
            "pm54_060_s250_r" => "p".to_string(),
            a => panic!("file name order has changed. {a} not recognised"),
        };
        mg.calc_alias()
    });
    let generate_path: PathBuf = "crates/dynamixel_ct/src/models/generated/".into();
    fs::remove_dir_all(&generate_path).ok();
    let mod_path = generate_path.join("mod.rs");

    if fs::exists(&mod_path)? {
        fs::remove_file(&mod_path)?;
    }

    all_models.clone().into_iter().try_for_each(|model| {
        let path = generate_path.join(format!("{}.rs", model.file_name()));

        generate::write_file_model_group(&mod_path, &path, model)?;
        anyhow::Ok(())
    })?;

    generate::create_match(&mod_path, all_models)?;

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

