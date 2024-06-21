mod attack_types;
mod payload;

use std::env;
use std::process;

fn print_header() {
    println!();
    println!("===============================");
    println!("      CertusHack PayloadForger      ");
    println!("===============================");
    println!();
    println!("Developed by CertusHack");
    println!("---------------------------------");
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
            println!("Generated {} payloads and saved to archives/generated_payloads.txt", count);
        } else {
            eprintln!("Invalid attack type specified.");
            process::exit(1);
        }
    } else {
        eprintln!("Invalid payload type specified.");
        process::exit(1);
    }
}
