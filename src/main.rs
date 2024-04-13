use std::fs::{read_dir, remove_dir_all};
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;

// -----

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Only print out directories that would be deleted, don't actually change filesystem
    #[arg(short, long)]
    dry_run: bool,
    /// Root to scan recursively for Rust projects
    scan_path: Option<PathBuf>,
}

// -----

fn recursive_find(path: &Path, delete: bool) -> Result<()> {
    let mut directories = Vec::new();

    let mut target_dir = None;
    let mut found_toml = false;
    let mut found_src = false;

    for e in read_dir(path)? {
        let e = e?;

        if e.file_type()?.is_dir() {
            if e.file_name() == "target" {
                target_dir = Some(e.path());
            } else if e.file_name() == "src" {
                found_src = true;
            }
            directories.push(e.path())
        } else {
            if e.file_name() == "Cargo.toml" {
                found_toml = true;
                
            }
        }

        if target_dir.is_some() & found_toml & found_src {
            break
        }
    }

    if target_dir.is_some() & found_toml & found_src {
        if let Some(target) = target_dir {
            println!("Deleting {:?}", target);
            if delete {
                remove_dir_all(target)?;
            }
        }
    } else {
        for subdir in directories {
            recursive_find(&subdir, delete)?;
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let path = if let Some(p) = args.scan_path {
        p
    } else {
        let mut p = PathBuf::new();
        p.push(".");
        p
    };

    println!("Scanning @ {:?}", &path);

    recursive_find(&path, !args.dry_run).unwrap();
}
