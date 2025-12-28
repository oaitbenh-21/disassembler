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

    pub fn run_disassembler(file_path: String) -> String {
        let mut file_open = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("{:?}", err);
            }
        };
        let config: BinaryConfig = BinaryConfig::intialize_config(&mut file_open);
        if config.magic_code != [0x00, 0xea, 0x83, 0xf3] {
            panic!("Invalid magic code. Not a valid core file.");
        }
        let record: BinaryRecord = BinaryRecord::new(config);
        return record.decode_instructions();
    }
    pub fn decode_instructions(self) -> String {
        let mut result: String = String::new();
        result.push_str(&format!(".name \"{}\"\n", self.program_name));
        result.push_str(&format!(".description \"{}\"\n", self.program_description));
        if self.code_size != self.code.len() as u32 {
            panic!("Mismatch in code size and actual instructions length.");
        } else if self.code_size == 0 {
            panic!("No instructions to decode.");
        }
        result
            .push_str(InstructionValues::get_instruction_data(self.code, &self.code_size).as_str());
        return result;
    }
}
