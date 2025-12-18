use shared::{
    instructions::{INSTRUCTIONS, ParamType},
    utils::{read_i16_be, read_i32_be},
};

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
        println!("instructions: {:?}", instructions_slice);
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
            print!("{}", current_instruction_data.name);
            if current_instruction_data.has_pcode {
                current_instruction_values.pcode =
                    &instructions_slice.get(decoded_byte_count).unwrap();
                decoded_byte_count += 1;
                for (i, _) in current_instruction_data.params.iter().enumerate() {
                    let pcode = (*current_instruction_values.pcode) >> (6 - i * 2) | 0b00;
                    match pcode {
                        0b11 => {
                            // ParamType::Indirect == i16
                            let value = instructions_slice
                                .get(decoded_byte_count as usize..decoded_byte_count as usize + 2)
                                .unwrap();
                            print!(" %{:?}", read_i16_be(value, &mut 0));
                            decoded_byte_count += 2;
                        }
                        0b10 => {
                            // ParamType::Direct
                            if current_instruction_data.has_idx {
                                let value = instructions_slice
                                    .get(
                                        decoded_byte_count as usize
                                            ..decoded_byte_count as usize + 2,
                                    )
                                    .unwrap();
                                print!(" {:?}", read_i16_be(value, &mut 0).unwrap());
                                decoded_byte_count += 2;
                            } else {
                                let value = instructions_slice
                                    .get(
                                        decoded_byte_count as usize
                                            ..decoded_byte_count as usize + 4,
                                    )
                                    .unwrap();
                                print!(" {:?}", read_i32_be(value, &mut 0).unwrap());
                                decoded_byte_count += 4;
                            };
                        }
                        0b01 => {
                            // ParamType::Register u8
                            let value = instructions_slice
                                .get(decoded_byte_count as usize..decoded_byte_count)
                                .unwrap();
                            print!(" r{}", value[0]);
                            decoded_byte_count += 1;
                        }
                        _ => {}
                    }
                    println!(";")
                }
            } else {
                for params_arr in current_instruction_data.params {
                    if matches!(params_arr[0], ParamType::Direct) {
                        if current_instruction_data.has_idx {
                            let value = instructions_slice
                                .get(decoded_byte_count as usize..decoded_byte_count as usize + 2)
                                .unwrap();
                            println!(" %{:?}", read_i16_be(value, &mut 0).unwrap());
                            decoded_byte_count += 2;
                        } else {
                            let value = instructions_slice
                                .get(decoded_byte_count as usize..decoded_byte_count as usize + 4)
                                .unwrap();
                            println!(" %{:?}", read_i32_be(value, &mut 0).unwrap());
                            decoded_byte_count += 4;
                        };
                    }
                }
            }
        }
    }
}
