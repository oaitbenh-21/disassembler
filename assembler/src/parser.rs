use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::{
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
        let line = line.map_err(|e| format!("error reading for the buffer: {e}"))?;
        let line_trimmed = line.trim();
        if line_trimmed.is_empty() || line_trimmed.starts_with(';') || line_trimmed.starts_with("#")
        {
            continue;
        }
        // basic macro system
        if line_trimmed.starts_with(".macro") {
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
        if line_trimmed.starts_with(".endmacro") {
            if !collect_macro {
                return Err(format!(
                    "Error on line {}: No matching macro end found",
                    line_num + 1
                ));
            }
            macros_map.insert(
                macro_data.borrow().clone().macro_name,
                macro_data.borrow_mut().clone(),
            );
            collect_macro = false;
            continue;
        }
        // Capture .usemacro
        if line_trimmed.starts_with(".usemacro") {
            let splited_macro_definer = line_trimmed.split(" ").collect::<Vec<&str>>();

            if splited_macro_definer.len() != 2 {
                return Err(format!(
                    "Error on line {}: Macro use is not called correctly",
                    line_num + 1
                ));
            }
            if let Some(called_macro_data) = macros_map.get(splited_macro_definer[1]) {
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
                    macro_data
                        .borrow_mut()
                        .clone()
                        .macro_instructions
                        .push(instr.clone());
                }
                player.instructions.push(instr.clone());
            }
            Err(e) => return Err(format!("Error line {}: {}", line_num + 1, e)),
        }
    }
    println!("Player Values: {:?}", player);
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
