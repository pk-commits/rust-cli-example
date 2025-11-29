use std::process::Command;
use clap::{Arg, Command as ClapCommand}; // 👈 note the import + alias
use serde_json::Value;

fn run_command(command: &str) -> String {
    // Slightly safer split (handles multiple spaces, etc.)
    let args: Vec<&str> = command.split_whitespace().collect();

    let output = Command::new(args[0])
        .args(&args[1..])
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn run_lsblk(device: &str) -> Value {
    let command = "lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT";
    let output = run_command(command);

    let devices: Value = serde_json::from_str(&output).expect("Failed to parse lsblk JSON");
    let devices = devices["blockdevices"]
        .as_array()
        .expect("blockdevices is not an array");

    for parent in devices {
        if parent["name"] == device {
            return parent.clone();
        }
        if let Some(children) = parent["children"].as_array() {
            for child in children {
                if child["name"] == device {
                    return child.clone();
                }
            }
        }
    }

    panic!("Device not found");
}

fn main() {
    let matches = ClapCommand::new("lsblk")
        .version("0.0.1")
        .author("Alfredo Deza")
        .about("lsblk in Rust")
        .arg(
            Arg::new("device")        // 👈 was Arg::with_name(...)
                .help("Device to query")
                .required(true)
                .index(1),
        )
        .get_matches();

    // 👇 clap 4 uses get_one::<T>() instead of value_of()
    let device = matches
        .get_one::<String>("device")
        .expect("device argument is required");

    let output = serde_json::to_string(&run_lsblk(device)).unwrap();
    println!("{output}");
}
