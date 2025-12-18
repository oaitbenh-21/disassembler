use shared::instructions::{INSTRUCTIONS, ParamType};

pub struct InstructionValues<'a> {
    pub opcode: &'a u8,
    pub pcode: &'a u8,
    pub param: Vec<ParamTypeAndValue>,
}

pub enum ParamTypeAndValue {
    Register(u8),
    Direct(u16),
    DirectHasIdx(u32),
    Indirect(u64),
}

impl<'a> InstructionValues<'a> {
    pub fn get_instruction_data(instructions_slice: Vec<u8>, code_size: &u32) {
        let mut decoded_byte_count: usize = 0;
        while decoded_byte_count < *code_size as usize {
            let mut current_instruction_values = InstructionValues {
                opcode: &(0),
                pcode: &(0),
                param: (Vec::new()),
            };
            current_instruction_values.opcode =
                &instructions_slice.get(decoded_byte_count).unwrap();
            decoded_byte_count += 1;
            let current_instruction_data =
                INSTRUCTIONS[*current_instruction_values.opcode as usize - 1];
            if current_instruction_data.has_pcode {
                current_instruction_values.pcode =
                    &instructions_slice.get(decoded_byte_count).unwrap();
                println!("pcode: {}", *current_instruction_values.pcode);
            } else {
                for params_arr in current_instruction_data.params {
                    if matches!(params_arr[0], ParamType::Direct) {
                        if current_instruction_data.has_idx {
                            let value = instructions_slice
                                .get(decoded_byte_count as usize..decoded_byte_count as usize + 2)
                                .unwrap();
                            println!("arg: {:?}", value);
                            decoded_byte_count += 2;
                        } else {
                            let value = instructions_slice
                                .get(decoded_byte_count as usize..decoded_byte_count as usize + 4)
                                .unwrap();
                            println!("arg: {:?}", value);
                            decoded_byte_count += 4;
                        };
                    }
                }
            }
            println!("current ins: {:?}", current_instruction_data);
        }
    }
}
