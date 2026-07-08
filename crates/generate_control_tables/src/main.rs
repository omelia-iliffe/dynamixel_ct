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
    if Path::new("docs").exists().not() {
        clone_docs()?
    }

    let dirs = [
        "docs/docusaurus/docs/dxl/model_reference/x_series",
        "docs/docusaurus/docs/dxl/model_reference/y_series",
        "docs/docusaurus/docs/dxl/model_reference/p_series",
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

    // Per-variant tables with exact units, for model groups whose members disagree on a
    // unit (the shared group table drops those to `None`). Computed before grouping
    // consumes `models`.
    let separated = parse::separated_tables(&models);

    let mut all_models: Vec<ModelGroup> = Vec::new();

    for m in models {
        if let Some(mg) = all_models.iter_mut().find(|mg| {
            println!("comparing {} with {}", mg.name(), m.model);
            mg.table_compatible(&m.table)
        }) {
            mg.insert_model(m.model);
            mg.merge(m.table);
        } else {
            let mut mg = ModelGroup::new(m.table, m.indirect);
            mg.insert_model(m.model);
            all_models.push(mg);
        }
    }

    // Sort groups by name so the generated `mod` declarations and match arms are
    // deterministic and independent of filesystem traversal order.
    all_models.sort_by_key(|mg| mg.name());

    for mg in &all_models {
        println!("model_group: {:?}", mg.name());
    }

    println!("total model groups {}", all_models.len());

    let generate_path: PathBuf = "crates/dynamixel_ct/src/models/".into();
    fs::remove_dir_all(&generate_path).ok();
    let mod_path = generate_path.join("mod.rs");

    fs::create_dir_all(&generate_path)?;
    generate::mod_path_header(&mod_path)?;

    all_models.iter().try_for_each(|model| {
        generate::write_file_model_group(&mod_path, &generate_path, model)?;
        anyhow::Ok(())
    })?;

    for sep in &separated {
        generate::write_separated_table(&mod_path, &generate_path, sep)?;
    }

    generate::create_match(&mod_path, &all_models)?;

    let mut fmt = Command::new("cargo").arg("fmt").spawn()?;
    if !fmt.wait()?.success() {
        panic!("cargo fmt failed")
    }

    Ok(())
}

fn filter_files(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    // only model pages
    if path.extension().and_then(|e| e.to_str()) != Some("mdx") {
        return false;
    }
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    // skip series index pages (`*_series.mdx`) and models this crate doesn't support:
    // the `2x` dual servos, `xl320` (protocol 1.0), `xw430`, and the `xc430-*bb` variants.
    let filter = ["_series", "xl320", "2x", "xw430", "bb"];
    filter.iter().any(|f| name.contains(f)).not()
}

fn clone_docs() -> Result<()> {
    // The docs repo carries large image assets, so do a blobless, sparse clone that
    // only materialises the control-table model pages.
    let run = |args: &[&str]| -> Result<()> {
        let status = Command::new("git")
            .args(args)
            .spawn()
            .with_context(|| anyhow!("failed to spawn git {:?}", args))?
            .wait()
            .with_context(|| anyhow!("failed to wait on git {:?}", args))?;
        status
            .success()
            .then_some(())
            .ok_or_else(|| anyhow!("git {:?} failed", args))
    };

    run(&[
        "clone",
        "--filter=blob:none",
        "--no-checkout",
        "--depth",
        "1",
        "https://github.com/ROBOTIS-GIT/docs.git",
    ])?;
    run(&[
        "-C",
        "docs",
        "sparse-checkout",
        "set",
        "docusaurus/docs/dxl/model_reference",
    ])?;
    run(&["-C", "docs", "checkout"])?;
    Ok(())
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
