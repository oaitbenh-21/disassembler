#[derive(Debug)]
pub struct Operation {
    pub number_one: i64,
    pub number_two: i64,
    pub operation: char,
}

fn calculate_from_operation(op: &Operation) -> Option<f64> {
    let num1 = op.number_one as f64;
    let num2 = op.number_two as f64;

    let result = match op.operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                return None;
            }
            num1 / num2
        }
        _ => return None,
    };

    Some(result)
}

pub fn extract_number_and_calculate(input: &str) -> Result<(Vec<String>, Vec<String>), String> {
    let mut results = Vec::new();
    let mut operations: Vec<String> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let mut matched_bracket_content: String = String::new();
        if chars[i] == '[' {
            matched_bracket_content.push(chars[i]);
            i += 1;

            let mut num1 = String::new();
            let mut num2 = String::new();
            let mut operator: Option<char> = None;

            while i < chars.len() {
                let c = chars[i];
                matched_bracket_content.push(c);
                if c == ']' {
                    break;
                }

                if c.is_ascii_digit() {
                    if operator.is_none() {
                        num1.push(c);
                    } else {
                        num2.push(c);
                    }
                } else if "+-*/".contains(c) && !num1.is_empty() && operator.is_none() {
                    operator = Some(c);
                } else if !c.is_whitespace() {
                    break;
                }

                i += 1;
            }
            operations.push(matched_bracket_content);
            if !num1.is_empty() && !num2.is_empty() && operator.is_some() {
                let op = Operation {
                    number_one: num1.parse().unwrap_or(0),
                    number_two: num2.parse().unwrap_or(0),
                    operation: operator.unwrap_or('+'),
                };

                results.push(op);
            }
        }
        i += 1;
    }

    Ok((calculate_operations_to_strings(results), operations))
}

fn calculate_operations_to_strings(ops: Vec<Operation>) -> Vec<String> {
    ops.into_iter()
        .filter_map(|op| calculate_from_operation(&op).map(|res| res.to_string()))
        .collect()
}
