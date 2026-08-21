use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let profile_dir = out_dir
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("Cargo OUT_DIR has the expected target directory layout");

    copy_directory(Path::new("assets"), &profile_dir.join("assets"));

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("linux") {
        return;
    }

    let dependency_build_dir = profile_dir.join("build");
    let steam_api = fs::read_dir(&dependency_build_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("steamworks-sys-")
        })
        .map(|entry| entry.path().join("out/libsteam_api.so"))
        .find(|path| path.is_file())
        .unwrap_or_else(|| {
            panic!(
                "could not find libsteam_api.so in {}",
                dependency_build_dir.display()
            )
        });

    let runtime_steam_api = profile_dir.join("libsteam_api.so");
    fs::copy(&steam_api, &runtime_steam_api).unwrap_or_else(|error| {
        panic!(
            "could not copy {} to {}: {error}",
            steam_api.display(),
            runtime_steam_api.display()
        )
    });

    println!("cargo:rustc-link-arg-bin=prop-hunt=-Wl,-rpath,$ORIGIN");
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap_or_else(|error| {
        panic!(
            "could not create asset directory {}: {error}",
            destination.display()
        )
    });

    for entry in fs::read_dir(source)
        .unwrap_or_else(|error| {
            panic!(
                "could not read asset directory {}: {error}",
                source.display()
            )
        })
        .flatten()
    {
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_directory(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).unwrap_or_else(|error| {
                panic!(
                    "could not copy asset {} to {}: {error}",
                    source_path.display(),
                    destination_path.display()
                )
            });
        }
    }
}
