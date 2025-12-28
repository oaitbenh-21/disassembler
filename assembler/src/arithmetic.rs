pub fn calculate_from_brackets(caps: Vec<&str>) -> Option<f64> {
    let num1: f64 = caps[1].parse().ok()?;
    let op = caps[2];
    let num2: f64 = caps[3].parse().ok()?;

    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => return None,
    };
    Some(result)
}

pub fn extract_number_operations(input: &str) -> Vec<String> {
    let mut results = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '[' {
            let start = i;
            i += 1;
            let mut has_operator = false;
            while i < chars.len() && chars[i] != ']' {
                if "+-*/".contains(chars[i]) {
                    has_operator = true;
                }
                i += 1;
            }

            if i < chars.len() && chars[i] == ']' && has_operator {
                let end = i;
                results.push(chars[start..=end].iter().collect());
            }
        }
        i += 1;
    }
    println!("results: {:?}", results);
    results
}
