use std::collections::VecDeque;

fn infix_to_postfix(input: &str) -> String {
    let mut output_queue = VecDeque::new();
    let mut operator_stack = Vec::new();
    let mut last_token_was_operand = false;

    for token in input.chars().filter(|c| !c.is_whitespace()) {
        match token {
            '0'..='9' => {
                //if last_token_was_operand {
                    //output_queue.push_back('*');
                //}
                output_queue.push_back(token);
                last_token_was_operand = true;
            }
            '(' => {
                if last_token_was_operand {
                    operator_stack.push('*');
                }
                operator_stack.push(token);
                last_token_was_operand = false;
            }
            ')' => {
                while let Some(operator) = operator_stack.pop() {
                    if operator == '(' {
                        break;
                    } else {
                        output_queue.push_back(operator);
                    }
                }
                last_token_was_operand = false;
            }
            '+' | '-' | '*' | '/' | '^' => {
                if !last_token_was_operand && (token == '+' || token == '-') {
                    // Handle unary plus and minus operators
                    output_queue.push_back('0');
                }
                let token_precedence = operator_precedence(token);
                while let Some(&top) = operator_stack.last() {
                    if top == '(' {
                        break;
                    } else {
                        let top_precedence = operator_precedence(top);
                        if token_precedence <= top_precedence {
                            output_queue.push_back(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                }
                operator_stack.push(token);
                last_token_was_operand = false;
            }
            _ => panic!("Invalid token: {}", token),
        }
    }

    while let Some(operator) = operator_stack.pop() {
        output_queue.push_back(operator);
    }

    output_queue.into_iter().collect()
}

fn operator_precedence(operator: char) -> i32 {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => panic!("Invalid operator: {}", operator),
}
}

fn main() {
let infix_expression = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
let postfix_expression = infix_to_postfix(infix_expression);
println!("Infix expression: {}", infix_expression);
println!("Postfix expression: {}", postfix_expression);
let infix_expression = "256*2";
let postfix_expression = infix_to_postfix(infix_expression);
println!("Infix expression: {}", infix_expression);
println!("Postfix expression: {}", postfix_expression);
let infix_expression = "3+-2";
let postfix_expression = infix_to_postfix(infix_expression);
println!("Infix expression: {}", infix_expression);
println!("Postfix expression: {}", postfix_expression);
let infix_expression = "3+2*7+3-1/2";
let postfix_expression = infix_to_postfix(infix_expression);
println!("Infix expression: {}", infix_expression);
println!("Postfix expression: {}", postfix_expression);
}
