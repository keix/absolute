use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use serde_json::json;

const INPUT: &str = "data/linux-x86_64-syscall_64.tbl";
const OUTPUT: &str = "scripts/seed-linux-x86_64.json";
const OS: &str = "linux";
const ARCH: &str = "x86_64";

fn main() -> std::io::Result<()> {
    let file = File::open(INPUT)?;
    let reader = BufReader::new(file);

    let mut items: Vec<serde_json::Value> = Vec::new();
    let mut skipped_x32 = 0usize;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let nr: u32 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let abi = parts[1];
        let name = parts[2];
        let entry = parts.get(3).copied().unwrap_or("");

        if abi == "x32" {
            skipped_x32 += 1;
            continue;
        }

        let pk = format!("{}#{}", OS.to_uppercase(), ARCH);

        for sk in [
            format!("SYSCALL#NAME#{}", name),
            format!("SYSCALL#NR#{}", nr),
        ] {
            items.push(json!({
                "PutRequest": {
                    "Item": {
                        "pk": { "S": pk },
                        "sk": { "S": sk },
                        "entity_type": { "S": "syscall" },
                        "os": { "S": OS },
                        "arch": { "S": ARCH },
                        "number": { "N": nr.to_string() },
                        "abi": { "S": abi },
                        "name": { "S": name },
                        "entry": { "S": entry },
                        "args": { "L": [] }
                    }
                }
            }));
        }
    }

    let mut out = File::create(OUTPUT)?;
    let json = serde_json::to_string_pretty(&items)?;
    out.write_all(json.as_bytes())?;
    out.write_all(b"\n")?;

    println!(
        "wrote {} items to {} (skipped {} x32 entries)",
        items.len(),
        OUTPUT,
        skipped_x32
    );
    Ok(())
}
