use crate::module::{config, record};

use super::config::BinaryConfig;
use std::fs::File;

#[derive(Debug)]

pub struct BinaryRecord {
    pub program_name: String,
    pub program_description: String,
    pub code_size: u32,
    pub code: Vec<String>,
}

impl BinaryRecord {
    pub fn new(config: BinaryConfig) -> BinaryRecord {
        return BinaryRecord {
            program_name: String::from_utf8_lossy(&config.program_name)
                .trim_end_matches('\0')
                .to_string(),
            program_description: String::from_utf8_lossy(&config.program_description)
                .trim_end_matches('\0')
                .to_string(),
            code_size: config.read_code_size(),
            code: Vec::new(),
        };
    }
    pub fn run_disassembler(file_path: String) {
        let mut file_open = File::open(file_path).unwrap();
        let config: BinaryConfig = BinaryConfig::intialize_config(&mut file_open);
        let record: BinaryRecord = BinaryRecord::new(config);
        println!("{:?}", record);
    }
}
