use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::{
    arithmetic::extract_number_and_calculate,
    instruction::{InstructionInstance, Param, ValueType},
    lexer::{Token, tokenize},
};
use shared::instructions::{INSTRUCTIONS, ParamType};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub comment: String,
    pub instructions: Vec<InstructionInstance>,
    pub labels: HashMap<String, usize>,
}

impl Player {
    fn new() -> Self {
        Player {
            name: String::new(),
            comment: String::new(),
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }
}
#[derive(Debug, Clone)]
struct AssemblyMacro {
    macro_name: String,
    macro_instructions: Vec<InstructionInstance>,
}

pub fn parse_file(path: &Path) -> Result<Player, String> {
    let file = File::open(path).map_err(|e| format!("{e}"))?;
    let reader = BufReader::new(file);
    let mut player = Player::new();
    // macro system used variables
    let mut collect_macro: bool = false;
    let macro_data: RefCell<AssemblyMacro> = RefCell::new(AssemblyMacro {
        macro_name: String::new(),
        macro_instructions: Vec::new(),
    });
    let mut macros_map: HashMap<String, AssemblyMacro> = HashMap::new();
    // preprocessing loop
    for (line_num, line) in reader.lines().enumerate() {
        let mut line = line.map_err(|e| format!("error reading for the buffer: {e}"))?;
        let mut line_trimmed = line.trim().to_string();
        let operations = extract_number_and_calculate(&line.as_str());
        match operations {
            Ok(val) => {
                let (to, from) = val;
                let len = from.len().min(to.len());

                for i in 0..len {
                    line = line.replace(&from[i], &to[i]);
                    line_trimmed = line_trimmed.replace(&from[i], &to[i]);
                }
            }
            Err(error) => {
                return Err(format!("Error on line {}: {}", line_num + 1, error));
            }
        };

        if line_trimmed.is_empty() || line_trimmed.starts_with(';') || line_trimmed.starts_with("#")
        {
            continue;
        }
        // basic macro system
        if line_trimmed.starts_with(".macro ") {
            let splited_macro_definer = line_trimmed.split(" ").collect::<Vec<&str>>();
            if splited_macro_definer.len() != 2 {
                return Err(format!(
                    "Error on line {}: Macro definition is not called correctly",
                    line_num + 1
                ));
            }
            macro_data.borrow_mut().macro_name = splited_macro_definer[1].to_string();
            macro_data.borrow_mut().macro_instructions = Vec::new();
            collect_macro = true;
            continue;
        }
        // Capture .endmacro
        if line_trimmed == ".endmacro" {
            if !collect_macro {
                return Err(format!(
                    "Error on line {}: No matching macro end found",
                    line_num + 1
                ));
            }
            let current_macro_data = macro_data.borrow().clone();
            macros_map.insert(current_macro_data.macro_name.clone(), current_macro_data);
            collect_macro = false;
            continue;
        }
        // Capture .usemacro
        if line_trimmed.starts_with(".usemacro ") {
            let splited_macro_definer = line_trimmed.split(" ").collect::<Vec<&str>>();

            if splited_macro_definer.len() != 2 {
                return Err(format!(
                    "Error on line {}: Macro use is not called correctly",
                    line_num + 1
                ));
            }
            if let Some(called_macro_data) = macros_map.get(splited_macro_definer[1]) {
                println!("called macro: {:?}", called_macro_data.clone());
                for (_, instr) in called_macro_data.macro_instructions.iter().enumerate() {
                    player.instructions.push(instr.clone());
                }
                continue;
            } else {
                return Err(format!(
                    "Error on line {}: No matching macro use found",
                    line_num + 1
                ));
            }
        }

        // Capture .name directive
        if line_trimmed.starts_with(".name") {
            if let Some(start) = line_trimmed.find('"') {
                if let Some(end) = line_trimmed[start + 1..].find('"') {
                    if !player.name.is_empty() {
                        return Err(format!(
                            "Error line {}: multiple name declaration",
                            line_num + 1
                        ));
                    }
                    player.name = line_trimmed[start + 1..start + 1 + end].to_string();
                    continue;
                }
            }
            return Err(format!(
                "Error line {}: Invalid .name directive",
                line_num + 1
            ));
        }

        // Capture .comment directive
        if line_trimmed.starts_with(".description") {
            if let Some(start) = line_trimmed.find('"') {
                if let Some(end) = line_trimmed[start + 1..].find('"') {
                    if !player.comment.is_empty() {
                        return Err(format!(
                            "Error line {}: multiple comment declaration",
                            line_num + 1
                        ));
                    }
                    player.comment = line_trimmed[start + 1..start + 1 + end].to_string();
                    continue;
                }
            }
            return Err(format!(
                "Error line {}: Invalid .comment directive",
                line_num + 1
            ));
        }

        let tokens = tokenize(&line).map_err(|e| {
            format!(
                "error parsing the file line: {}\nerror: {}",
                line_num + 1,
                e
            )
        })?;
        match parse_tokens(&tokens) {
            Ok(instr) => {
                if collect_macro {
                    // println!("i should to copy: {:?}", instr);
                    // println!();
                    macro_data
                        .borrow_mut()
                        .macro_instructions
                        .push(instr.clone());
                    // println!("copied: {:?}", macro_data);
                }
                player.instructions.push(instr.clone());
            }
            Err(e) => return Err(format!("Error line {}: {}", line_num + 1, e)),
        }
    }
    if collect_macro {
        return Err(format!("Error: There is no end of macro"));
    }
    Ok(player)
}

// Convert tokens into InstructionInstance
// here should the return should be a inst or labelDef(String) so we need a general type contains the both
fn parse_tokens(tokens: &[Token]) -> Result<InstructionInstance, String> {
    let mut iter = tokens.iter();
    let mut token = iter.next().ok_or("Empty line")?;
    let mut instruction_instance = InstructionInstance::new();
    if let Token::LabelDef(lbl) = token {
        instruction_instance.label = Some(lbl.clone());
        let option_token = iter.next();
        match option_token {
            Some(tkn) => token = tkn,
            None => return Ok(instruction_instance),
        }
    }
    let instr_name = match token {
        Token::Instr(name) => name,
        _ => return Err("Expected instruction".to_string()),
    };

    // Lookup instruction in INSTRUCTIONS table
    let instr = INSTRUCTIONS
        .iter()
        .find(|i| i.name == instr_name.as_str())
        .ok_or(format!("Unknown instruction: {}", instr_name))?;

    let mut params = Vec::new();
    for token in iter {
        match token {
            Token::Register(r) => params.push(Param {
                param_type: ParamType::Register,
                value: ValueType::Value(*r as i32),
            }),
            Token::Direct(v) => params.push(Param {
                param_type: ParamType::Direct,
                value: ValueType::Value(*v),
            }),
            Token::Indirect(v) => params.push(Param {
                param_type: ParamType::Indirect,
                value: ValueType::Value(*v),
            }),
            Token::LabelRef(v) => params.push(Param {
                param_type: ParamType::Direct,
                value: ValueType::Label(v.clone()),
            }),
            Token::Comma => continue,
            _ => return Err(format!("Unexpected token: {:?}", token)),
        }
    }

    if params.len() != instr.nb_params as usize {
        return Err(format!(
            "Expected {} params, got {}",
            instr.nb_params,
            params.len()
        ));
    }
    instruction_instance.instr = Some(instr);
    instruction_instance.params = params;
    Ok(instruction_instance)
}
