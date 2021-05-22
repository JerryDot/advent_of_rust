use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::process::Command;


static MS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Time: (\d+)ms").unwrap());

fn extract_time(s: &str) -> u32 {
    let capture = MS_REGEX.captures_iter(&s).next().unwrap();
    capture[1].parse().unwrap()
}

fn main() {
    let exn = std::ffi::OsStr::new("rs");
    let total_time = fs::read_dir("./src/bin")
        .unwrap()
        .into_iter()
        .filter_map(|entry| {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => {
                    return None;
                }
            };
            let path = entry.path();
            if path.extension() == Some(exn) {
                let file_without_extension = path.file_stem()?.to_str()?;
                let cmd = Command::new("cargo")
                    .args(&["run", "--release", "--bin", file_without_extension])
                    .output()
                    .unwrap();
                let output = String::from_utf8(cmd.stdout).unwrap();
                println!("{}:\n{}", file_without_extension, output);
                Some(extract_time(&output))
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("Total time: {}ms", total_time);
}