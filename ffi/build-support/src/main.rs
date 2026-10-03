use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn argument(name: &str) -> Option<String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    args.windows(2).find(|pair| pair[0] == name).map(|pair| pair[1].clone())
}

fn digest(path: &Path) -> String {
    format!("{:x}", Sha256::digest(std::fs::read(path).expect("read package file")))
}

fn copy_directory(source: &Path, destination: &Path) {
    std::fs::create_dir_all(destination).expect("create package directory");
    for entry in std::fs::read_dir(source).expect("read bindings") {
        let path = entry.expect("binding entry").path();
        assert!(path.extension().is_some_and(|e| e == "h"), "unexpected binding file");
        std::fs::copy(&path, destination.join(path.file_name().expect("header filename")))
            .expect("copy header");
    }
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("ffi directory")
        .parent()
        .expect("workspace root")
        .to_owned();
    let output = argument("--out").expect("usage: cargo run -p baad-ffi-build -- --out DIRECTORY [--target TARGET] [--profile debug|release]");
    let output = std::env::current_dir().expect("current directory").join(output);
    let target = argument("--target");
    let profile = argument("--profile").unwrap_or_else(|| "release".into());
    assert!(profile == "release" || profile == "debug", "profile must be debug or release");
    let metadata = Command::new("cargo")
        .args(["metadata", "--locked", "--no-deps", "--format-version", "1"])
        .current_dir(&root)
        .output()
        .expect("cargo metadata");
    assert!(metadata.status.success(), "Cargo metadata failed");
    let metadata: Value = serde_json::from_slice(&metadata.stdout).expect("Cargo metadata JSON");
    let package = metadata["packages"]
        .as_array()
        .expect("workspace packages")
        .iter()
        .find(|p| p["name"] == "baad-ffi")
        .expect("FFI package");
    let mut build = Command::new("cargo");
    build.current_dir(&root).args(["build", "--locked", "-p", "baad-ffi"]);
    if profile == "release" {
        build.arg("--release");
    }
    if let Some(target) = &target {
        build.args(["--target", target]);
    }
    assert!(build.status().expect("build packaged library").success(), "library build failed");
    let generated = PathBuf::from(metadata["target_directory"].as_str().expect("target directory"))
        .join(format!(".baad-bindings-{}", std::process::id()));
    assert!(!generated.exists(), "temporary bindings directory already exists");
    std::fs::create_dir_all(&generated).expect("create temporary bindings directory");
    for family in ["baad-shared", "baad-utils", "baad-dm", "baad"] {
        let out = generated.join(family);
        std::fs::create_dir(&out).expect("create family output");
        baad_ffi_build::generate(family, &root, &out);
    }
    std::fs::remove_dir_all(generated).expect("remove temporary bindings output");
    let host = Command::new("rustc").arg("-vV").output().expect("Rust host");
    let host = String::from_utf8(host.stdout).expect("Rust version UTF-8");
    let triple = target.as_deref().unwrap_or_else(|| {
        host.lines().find_map(|l| l.strip_prefix("host: ")).expect("host triple")
    });
    let mut artifacts =
        PathBuf::from(metadata["target_directory"].as_str().expect("target directory"));
    if let Some(target) = &target {
        artifacts.push(target);
    }
    artifacts.push(&profile);
    let library = if triple.contains("windows") {
        "baad.dll"
    } else if triple.contains("apple") {
        "libbaad.dylib"
    } else {
        "libbaad.so"
    };
    if output.exists() && std::fs::read_dir(&output).expect("output directory").next().is_some() {
        let manifest: Value = serde_json::from_slice(
            &std::fs::read(output.join("manifest.json"))
                .expect("choose an empty directory or an existing BAAD C package")
        )
        .expect("package manifest");
        assert!(manifest["package"] == "baad-c-api", "output is not a BAAD C package");
    }
    let parent = output.parent().expect("package parent");
    std::fs::create_dir_all(parent).expect("create package parent");
    let staged = parent.join(format!(".baad-c-{}", std::process::id()));
    assert!(!staged.exists(), "staging directory already exists");
    std::fs::create_dir(&staged).expect("create staging directory");
    std::fs::copy(artifacts.join(library), staged.join(library)).expect("copy shared library");
    if triple.contains("windows") {
        let mut found = false;
        for name in ["baad.dll.lib", "libbaad.dll.a"] {
            if artifacts.join(name).exists() {
                std::fs::copy(artifacts.join(name), staged.join(name))
                    .expect("copy import library");
                found = true;
            }
        }
        assert!(found, "Windows package requires the matching import library");
    }
    let mut headers = BTreeMap::new();
    for family in ["baad", "baad-shared", "baad-utils", "baad-dm"] {
        let destination = staged.join(family).join("bindings");
        copy_directory(&root.join("ffi").join(family).join("bindings"), &destination);
        for entry in std::fs::read_dir(&destination).expect("packaged bindings") {
            let path = entry.expect("package header").path();
            headers.insert(
                path.strip_prefix(&staged)
                    .expect("relative package path")
                    .to_string_lossy()
                    .replace('\\', "/"),
                digest(&path)
            );
        }
    }
    let manifest = json!({"package": "baad-c-api", "version": package["version"], "target": triple, "library": library, "diplomat": "0.14.0", "library_sha256": digest(&staged.join(library)), "headers_sha256": headers});
    std::fs::write(
        staged.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).expect("manifest JSON") + "\n"
    )
    .expect("write manifest");
    if output.exists() {
        std::fs::remove_dir_all(&output).expect("replace old package");
    }
    std::fs::rename(&staged, &output).expect("publish package");
    println!("{}", output.display());
}
