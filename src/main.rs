fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error reading stdin");
    match check_brackets(buffer.trim()) {
        BracketResponse::NoErrorResponse(response) => println!("{}", response),
        BracketResponse::ErrorIndex(index) => println!("{}", index),
    }
}

enum BracketResponse {
    NoErrorResponse(String),
    ErrorIndex(usize)
}

#[derive(PartialEq, Eq)]
enum BracketType {
    Parenthesis,
    Square,
    Curly,
}


fn check_brackets(text: &str) -> BracketResponse {
    let brackets: [char; 6] = ['{', '}', '[', ']', '(', ')'];

    // keep track of the indices of all unclosed bracket of each type, when a closing bracket
    // is found the last opening bracket will be removed
    let mut unclosed_brackets: Vec<(BracketType, usize)> = Vec::new();

    // check brackets are closed correctly
    for (index, character) in text.chars().enumerate().filter(|(_, character)| brackets.contains(&character)) {
        let is_opening: bool;
        let current_bracket_type: BracketType;

        match character {
            '(' => {
                is_opening = true;
                current_bracket_type = BracketType::Parenthesis
            },
            '[' => {
                is_opening = true;
                current_bracket_type = BracketType::Square
            },
            '{' => {
                is_opening = true;
                current_bracket_type = BracketType::Curly
            },
            ')' => {
                is_opening = false;
                current_bracket_type = BracketType::Parenthesis
            },
            ']' => {
                is_opening = false;
                current_bracket_type = BracketType::Square
            },
            '}' => {
                is_opening = false;
                current_bracket_type = BracketType::Curly
            }
	        _ => panic!("should be impossible")
        }

        if is_opening { // opening bracket
            unclosed_brackets.push((current_bracket_type, index))
        }

        else { // closing bracket
            match unclosed_brackets.pop() {
                Some((bracket_type, _)) => {
                    if bracket_type != current_bracket_type {
                        return BracketResponse::ErrorIndex(index + 1)
                    }
                }
                None => {
                    return BracketResponse::ErrorIndex(index + 1)
                }
            }
        }
    }

    // check for any remaining opening brackets
    return {
        if unclosed_brackets.len() == 0 {
            BracketResponse::NoErrorResponse(String::from("Success"))
        }
        else {
            BracketResponse::ErrorIndex(unclosed_brackets[0].1 + 1)
        }
    }
}
