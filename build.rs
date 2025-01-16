// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn glob_io(pattern: &str) -> Result<Vec<PathBuf>> {
    glob::glob(pattern)
        .map_err(|err| Error::new(ErrorKind::Other, err))?
        .map(|item| item.map_err(|err| Error::new(ErrorKind::Other, err)))
        .collect::<Result<Vec<PathBuf>>>()
}

trait CommandExt {
    fn checked(&mut self);
}

impl CommandExt for Command {
    fn checked(&mut self) {
        let status = self.status().unwrap();
        if !status.success() {
            panic!("Command {:?} failed with status {status}", self);
        }
    }
}

/// Compile all blueprint files.
fn compile_blueprint() -> Vec<PathBuf> {
    let blueprint_files = glob_io("resources/**/*.blp").unwrap();
    if let Some("1") | Some("true") = std::env::var("SKIP_BLUEPRINT").ok().as_deref() {
        println!("cargo::warning=Skipping blueprint compilation, falling back to committed files.");
    } else {
        Command::new("blueprint-compiler")
            .args(["batch-compile", "resources", "resources"])
            .args(&blueprint_files)
            .checked();
    }
    blueprint_files
}

pub fn compile_resources<P: AsRef<Path>>(
    source_dirs: &[P],
    gresource: &str,
    target: &str,
) -> Vec<PathBuf> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let mut command = Command::new("glib-compile-resources");

    for source_dir in source_dirs {
        command.arg("--sourcedir").arg(source_dir.as_ref());
    }

    command
        .arg("--target")
        .arg(out_dir.join(target))
        .arg(gresource)
        .checked();

    let mut command = Command::new("glib-compile-resources");
    for source_dir in source_dirs {
        command.arg("--sourcedir").arg(source_dir.as_ref());
    }

    let output = command
        .arg("--generate-dependencies")
        .arg(gresource)
        .stderr(Stdio::inherit())
        .output()
        .unwrap()
        .stdout;

    let mut sources = vec![Path::new(gresource).into()];

    for line in String::from_utf8(output).unwrap().lines() {
        if line.ends_with(".ui") {
            // We build UI files from blueprint, so adapt the dependency
            sources.push(Path::new(line).with_extension("blp"))
        } else if line.ends_with(".metainfo.xml") {
            sources.push(Path::new(line).with_extension("xml.in"));
        } else {
            sources.push(line.into());
        }
    }

    sources
}

pub fn main() {
    let tasks = [std::thread::spawn(compile_blueprint)];

    let mut sources = tasks
        .into_iter()
        .flat_map(|task| task.join().unwrap())
        .collect::<Vec<_>>();

    sources.extend_from_slice(
        compile_resources(
            &["resources"],
            "resources/resources.gresource.xml",
            "umbrella.gresource",
        )
        .as_slice(),
    );

    for source in sources {
        println!("cargo:rerun-if-changed={}", source.display());
    }
}
