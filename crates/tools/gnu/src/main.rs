use std::collections::BTreeMap;
use std::io::prelude::*;

fn main() {
    let mut cmd = std::process::Command::new("where");
    cmd.arg("dlltool.exe");

    let output = cmd.output().unwrap();

    if !output.status.success() {
        println!("dlltool.exe not found");
        return;
    }

    let output = unsafe { core::str::from_utf8_unchecked(&output.stdout) };

    let platform = if output.contains("mingw64") {
        "x86_64_gnu"
    } else if output.contains("mingw32") {
        "i686_gnu"
    } else {
        println!("mingw not found");
        return;
    };

    println!("Platform: {}", platform);

    let libraries = lib::libraries();
    let output = std::path::PathBuf::from(format!("crates/targets/{}/lib", platform));
    let _ = std::fs::remove_dir_all(&output);
    std::fs::create_dir_all(&output).unwrap();

    for (library, functions) in &libraries {
        build_library(&output, library, functions, platform);
    }

    build_mri(&output, &libraries);

    for library in libraries.keys() {
        std::fs::remove_file(output.join(format!("lib{}.a", library))).unwrap();
    }
}

fn build_library(output: &std::path::Path, library: &str, functions: &BTreeMap<String, usize>, platform: &str) {
    println!("{}", library);

    // Note that we don't use set_extension as it confuses PathBuf when the library name includes a period.

    let def_path = output.join(format!("{}.def", library));
    let mut def = std::fs::File::create(&def_path).unwrap();

    def.write_all(
        format!(
            r#"
LIBRARY {}
EXPORTS
"#,
            library
        )
        .as_bytes(),
    )
    .unwrap();

    for (function, params) in functions {
        if platform.eq("i686_gnu") {
            def.write_all(format!("{}@{}\n", function, params).as_bytes()).unwrap();
        } else {
            def.write_all(format!("{}\n", function).as_bytes()).unwrap();
        }
    }

    drop(def);

    let mut cmd = std::process::Command::new("dlltool");
    cmd.current_dir(&output);

    if platform.eq("i686_gnu") {
        cmd.arg("-k");
    }

    cmd.arg("-d");
    cmd.arg(format!("{}.def", library));
    cmd.arg("-l");
    cmd.arg(format!("lib{}.a", library));
    cmd.output().unwrap();

    std::fs::remove_file(output.join(format!("{}.def", library))).unwrap();
}

fn build_mri(output: &std::path::Path, libraries: &BTreeMap<String, BTreeMap<String, usize>>) {
    let mri_path = output.join("unified.mri");
    let mut mri = std::fs::File::create(&mri_path).unwrap();
    println!("Generating {}", mri_path.to_string_lossy());

    mri.write_all(b"CREATE libwindows.a\n").unwrap();

    for library in libraries.keys() {
        mri.write_all(format!("ADDLIB lib{}.a\n", library).as_bytes()).unwrap();
    }

    mri.write_all(b"SAVE\nEND\n").unwrap();

    let mut cmd = std::process::Command::new("ar");
    cmd.current_dir(&output);
    cmd.arg("-M");
    cmd.stdin(std::fs::File::open(&mri_path).unwrap());
    cmd.output().unwrap();

    std::fs::remove_file(&mri_path).unwrap();
}
