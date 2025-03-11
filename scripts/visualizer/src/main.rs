#![allow(dead_code)]
mod dot;

use anyhow::{Context as _, Result, ensure};
use rayon::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

mod json;
use json::*;

mod attack_fsm;

const DUMP_DIR: &str = "C:/Users/Jakob/Documents/dev/nine-sols/StateDump/Attacks";

fn main() -> Result<()> {
    let path = Path::new(DUMP_DIR);
    println!("{}", path.display());

    let mut graphs = Vec::new();
    for level in path.read_dir()? {
        let level = level?;
        if !level.file_type()?.is_dir() {
            continue;
        }

        for monster in level.path().read_dir()? {
            let monster = monster?.path();
            let path = monster.join("data.json");
            ensure!(path.exists(), "{} does not exist", path.display());

            /*if !path.display().to_string().contains("Eigong") {
                continue;
            }*/

            println!(
                "graphing {}",
                monster.file_name().unwrap().to_str().unwrap()
            );

            let json = std::fs::read_to_string(path)?;
            let data: Data = serde_json::from_str(&json)?;

            let cx = attack_fsm::Context { data };
            let dot = cx.dump_graphviz().context(monster.display().to_string())?;
            std::fs::write(monster.join("data.dot"), &dot)?;
            graphs.push((monster, dot));
        }
    }

    graphs
        .par_iter()
        .try_for_each(|(monster, dot)| -> Result<_> {
            println!(
                "exporting {}",
                monster.file_name().unwrap().to_str().unwrap()
            );
            let svg = dot_export(dot, "svg").context(monster.display().to_string())?;
            std::fs::write(monster.join("data.svg"), &svg)?;
            Ok(())
        })?;

    Ok(())
}

fn dot_export(input: &str, format: &str) -> Result<Vec<u8>> {
    let mut cmd = Command::new("dot")
        .arg("-T")
        .arg(format)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    cmd.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
    let out = cmd.wait_with_output()?;
    ensure!(out.status.success());

    Ok(out.stdout)
}
