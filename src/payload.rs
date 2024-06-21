use std::fs::File;
use std::io::{BufWriter, Write, BufRead};
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::BufReader; 

pub fn generate_payloads(attack_type: &crate::attack_types::AttackType, count: usize) -> Vec<String> {
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

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
    writeln!(&mut stdout, "\n===============================").unwrap();
    writeln!(&mut stdout, "          Generated Payloads          ").unwrap();
    writeln!(&mut stdout, "===============================\n").unwrap();
    stdout.reset().unwrap();

    for (i, payload) in payloads.iter().enumerate() {
        writeln!(&mut writer, "---------------------------------").expect("Unable to write to file");
        writeln!(&mut writer, "Payload #{}: ", i + 1).expect("Unable to write to file");
        writeln!(&mut writer, "---------------------------------").expect("Unable to write to file");
        writeln!(&mut writer, "{}", payload).expect("Unable to write to file");
        writeln!(&mut writer, "").expect("Unable to write to file");
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
    writeln!(&mut stdout, "Payloads Generated\n").unwrap();
    stdout.reset().unwrap();

    writeln!(&mut writer, "").expect("Unable to write to file");
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> std::io::Result<impl Iterator<Item = std::io::Result<String>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines()) // Use BufReader::new
}
