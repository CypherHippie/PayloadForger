mod attack_types;

use attack_types::{AttackType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <raw> <attack_type>", args[0]);
        return;
    }

    let payload_type = &args[1];
    let attack_type_str = &args[2];
    let custom_payload = if args.len() > 3 {
        Some(args[3].clone())
    } else {
        None
    };

    if payload_type == "raw" {
        if let Some(attack_type) = AttackType::from_str(attack_type_str, custom_payload) {
            let payload = attack_type.generate();
            println!("{}", payload);
        } else {
            eprintln!("Invalid attack type specified.");
        }
    } else {
        eprintln!("Invalid payload type specified.");
    }
}

