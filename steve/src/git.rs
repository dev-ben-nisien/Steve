use core::str;
use std::process::Command;

pub fn get_diff() -> String {
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to execute git diff");
    if !output.status.success() {
        panic!("Something terrible has occured")
    }
    return str::from_utf8(&output.stdout).unwrap().to_string();
}
