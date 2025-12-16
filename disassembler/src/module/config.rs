use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct BinaryConfig {
    pub magic_code: [u8; 4],
    pub program_name: [u8; 128],
    pub program_padding: [u8; 8],
    pub program_size: [u8; 4],
    pub program_description: [u8; 2048],
}

impl BinaryConfig {
    pub const fn new() -> Self {
        Self {
            magic_code: [0; 4],
            program_name: [0; 128],
            program_padding: [0; 8],
            program_size: [0; 4],
            program_description: [0; 2048],
        }
    }
    pub fn read_code_size(self) -> u32 {
        return u32::from_be_bytes(self.program_size);
    }

    // initialize config of cor file
    pub fn intialize_config(file_data: &mut File) -> BinaryConfig {
        let mut config: BinaryConfig = BinaryConfig::new();
        file_data.read_exact(&mut config.magic_code);
        file_data.read_exact(&mut config.program_name);
        file_data.read_exact(&mut config.program_padding);
        file_data.read_exact(&mut config.program_description);
        file_data.read_exact(&mut config.program_size);
        let mut remaining_data = Vec::new();
        file_data.read_to_end(&mut remaining_data).unwrap();

        // println!("config: {:?}", config);
        println!("remaining_data: {:?}", remaining_data);
        return config;
    }

    pub fn read_magic_code(magic_code_slice: &[u8]) -> u32 {
        let mut magic_code: u32 = 0;
        for (i, magic_part) in magic_code_slice.iter().enumerate() {
            magic_code |= (*magic_part as u32) << (24 - 8 * i);
            // println!("part {} >> {:b}", i, magic_part);
            // println!("magic code = {:b}", magic_code);
        }
        return magic_code;
    }
}
