use std::process::Command;

fn main() {
    Command::new("mkdir")
        .arg("out")
        .spawn()
        .expect("Failed to create output directory.");
}