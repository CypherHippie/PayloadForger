mod attack_types;
mod payload;

use std::env;
use std::process;
use std::io::Write;  // Add this line to import the Write trait
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn print_header() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
    writeln!(&mut stdout, "\n===============================").unwrap();
    writeln!(&mut stdout, "      CertusHack PayloadForger      ").unwrap();
    writeln!(&mut stdout, "===============================\n").unwrap();
    stdout.reset().unwrap();
    
    writeln!(&mut stdout, "Developed by CertusHack").unwrap();
    writeln!(&mut stdout, "---------------------------------\n").unwrap();
}

fn main() {
    print_header();

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <raw> <attack_type> <number_of_payloads>", args[0]);
        process::exit(1);
    }

    let payload_type = &args[1];
    let attack_type_str = &args[2];
    let count_str = &args[3];
    
    // Parse the number of payloads
    let count = match count_str.parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number of payloads: {}", count_str);
            process::exit(1);
        }
    };

    // Process the attack type and generate payloads
    if payload_type == "raw" {
        if let Some(attack_type) = attack_types::AttackType::from_str(attack_type_str) {
            let payloads = payload::generate_payloads(&attack_type, count);
            payload::save_payloads(&payloads);
            
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
            writeln!(&mut stdout, "\nGenerated {} payloads and saved to archives/generated_payloads.txt", count).unwrap();
            stdout.reset().unwrap();
        } else {
            eprintln!("Invalid attack type specified.");
            process::exit(1);
        }
    } else {
        eprintln!("Invalid payload type specified.");
        process::exit(1);
    }
}
