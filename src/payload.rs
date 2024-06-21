use std::fs::{File};
use std::io::{BufWriter, Write, BufRead};
use std::path::Path;
use crate::attack_types::AttackType;

pub fn generate_payloads(attack_type: &AttackType, count: usize) -> Vec<String> {
    let mut payloads = Vec::new();
    for payload in attack_type.generate(count, false) {
        payloads.push(payload);
    }
    payloads
}

pub fn save_payloads(payloads: &[String]) {
    let file_path = Path::new("archives/generated_payloads.txt");
    let file = File::create(file_path).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "===============================").expect("Unable to write to file");
    writeln!(writer, "          Generated Payloads          ").expect("Unable to write to file");
    writeln!(writer, "===============================").expect("Unable to write to file");
    writeln!(writer, "").expect("Unable to write to file");

    for (i, payload) in payloads.iter().enumerate() {
        writeln!(writer, "Payload {}: {}", i + 1, payload).expect("Unable to write to file");
    }

    writeln!(writer, "").expect("Unable to write to file");
    writeln!(writer, "End of Payloads").expect("Unable to write to file");
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> std::io::Result<impl Iterator<Item = std::io::Result<String>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
