use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct BinaryConfig {
    pub magic_code: [u8; 4],
    pub program_name: [u8; 128],
    pub program_padding: [u8; 4],
    pub program_size: [u8; 4],
    pub program_description: [u8; 2048],
    pub program_instructions: Vec<u8>,
}

impl BinaryConfig {
    pub const fn new() -> Self {
        Self {
            magic_code: [0; 4],
            program_name: [0; 128],
            program_padding: [0; 4],
            program_size: [0; 4],
            program_description: [0; 2048],
            program_instructions: Vec::new(),
        }
    }

    // initialize config of cor file
    pub fn intialize_config(file_data: &mut File) -> BinaryConfig {
        let mut config: BinaryConfig = BinaryConfig::new();
        //_______________________________________________________
        // Serialize header_____________________________________|
        // buffer.extend(&head.magic.to_be_bytes());            |
        // buffer.extend(&head.name);                           |
        // buffer.extend(&head.padding);                        |
        // buffer.extend(&head.prog_size.to_be_bytes());        |
        // buffer.extend(&head.comment);                        |
        // buffer.extend(&head.padding);                        |
        //______________________________________________________|

        let _ = file_data.read_exact(&mut config.magic_code);
        let _ = file_data.read_exact(&mut config.program_name);
        let _ = file_data.read_exact(&mut config.program_padding);
        let _ = file_data.read_exact(&mut config.program_size);
        let _ = file_data.read_exact(&mut config.program_description);
        let _ = file_data.read_exact(&mut config.program_padding);
        let _ = file_data.read_to_end(&mut config.program_instructions);
        return config;
    }
}
