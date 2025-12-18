use super::config::BinaryConfig;
use super::instruction::InstructionValues;
use std::fs::File;

#[derive(Debug)]
pub struct BinaryRecord {
    pub program_name: String,
    pub program_description: String,
    pub code_size: u32,
    pub code: Vec<u8>,
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
            code_size: u32::from_be_bytes(config.program_size),
            code: config.program_instructions, //config.program_instructions,
        };
    }
    pub fn run_disassembler(file_path: String) {
        let mut file_open = File::open(file_path).unwrap();
        let config: BinaryConfig = BinaryConfig::intialize_config(&mut file_open);
        let record: BinaryRecord = BinaryRecord::new(config);
        record.decode_instructions();
    }
    pub fn decode_instructions(self) {
        println!(".name \"{}\"", self.program_name);
        println!(".description \"{}\"", self.program_description);
        InstructionValues::get_instruction_data(self.code, &self.code_size);
    }
}
