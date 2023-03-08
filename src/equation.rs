mod roll;

pub struct Equation {
    compiled_equation: Vec<Token>,
}
impl Equation {
    pub fn new(input: &str) -> Self {
        let compiled_equation = infix_to_postfix(input);
        Equation { compiled_equation }
    }
    // pub fn roll() -> f64 {
    //     todo!();
    //     //roll::Process(Self)
    // }
}
trait Rollable {
    fn roll(&self) -> f64;
}
impl Rollable for Equation {
    fn roll(&self) -> f64 {
        roll::process(self)
    }
}
#[derive(Debug, Clone, Copy)]
enum Token {
    Operand(u32),
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
    L,
    Dice(Die),
}
#[derive(Debug, Clone, Copy)]
struct Die {
    number: u32,
    sides: u32,
}

fn infix_to_postfix(input: &str) -> Vec<Token> {
    let mut output_queue = Vec::new();
    let mut operator_stack = Vec::new();
    let mut last_token_was_operand = false;
    let mut last_token_was_die = false;

    for token in input.chars().filter(|c| !c.is_whitespace()) {
        // println!("o{:?}", output_queue);
        // println!("s{:?}", operator_stack);
        // println!("{}", token);

        // println!("Operand {:?}", last_token_was_operand);
        // println!("Die {:?}", last_token_was_die);
        match token {
            '0'..='9' => {
                //if last_token_was_operand {
                //output_queue.push(Token::Times);
                //}
                //let digit: u32;
                if last_token_was_operand {
                    let digit: u32;
                    if let Token::Operand(value) = output_queue.pop().unwrap() {
                        digit = value * 10 + token.to_digit(10).unwrap();
                        output_queue.push(Token::Operand(digit));
                        last_token_was_operand = true;
                    } else {
                        panic!()
                    }
                } else if last_token_was_die {
                    //println!("{:?}", output_queue.pop().unwrap())
                    if let Token::Dice(cdie) = output_queue.pop().unwrap() {
                        let number = cdie.number;
                        let sides = cdie.sides * 10 + token.to_digit(10).unwrap();
                        output_queue.push(Token::Dice(Die { number, sides }));
                        last_token_was_die = true;
                    } else {
                        panic!()
                    }
                } else {
                    let digit = token.to_digit(10).unwrap();
                    output_queue.push(Token::Operand(digit));
                    last_token_was_operand = true;
                }
                //println!("Operand {:?}", last_token_was_operand);
                //println!("Die {:?}", last_token_was_die);
            }
            '(' => {
                if last_token_was_operand | last_token_was_die {
                    operator_stack.push(Token::Times);
                }
                operator_stack.push(Token::L);
                last_token_was_operand = false;
                last_token_was_die = false;
            }
            ')' => {
                while let Some(operator) = operator_stack.pop() {
                    if let Token::L = operator {
                        break;
                    } else {
                        output_queue.push(operator);
                    }
                }
                last_token_was_operand = false;
                last_token_was_die = false;
            }
            '+' => {
                if !last_token_was_operand && !last_token_was_die {
                    output_queue.push(Token::Operand(0));
                }
                let token_precedence = operator_precedence(Token::Plus);
                while let Some(&top) = operator_stack.last() {
                    if let Token::L = top {
                        break;
                    } else {
                        let top_precedence = operator_precedence(top);
                        if token_precedence <= top_precedence {
                            output_queue.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                }
                operator_stack.push(Token::Plus);
                last_token_was_operand = false;
                last_token_was_die = false;
            }
            '-' => {
                if !last_token_was_operand && !last_token_was_die {
                    output_queue.push(Token::Operand(0));
                }
                let token_precedence = operator_precedence(Token::Minus);
                while let Some(&top) = operator_stack.last() {
                    if let Token::L = top {
                        break;
                    } else {
                        let top_precedence = operator_precedence(top);
                        if token_precedence <= top_precedence {
                            output_queue.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                }
                operator_stack.push(Token::Minus);
                last_token_was_operand = false;
                last_token_was_die = false;
            }
            '*' => {
                let token_precedence = operator_precedence(Token::Times);
                while let Some(&top) = operator_stack.last() {
                    if let Token::L = top {
                        break;
                    } else {
                        let top_precedence = operator_precedence(top);
                        if token_precedence <= top_precedence {
                            output_queue.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                }
                operator_stack.push(Token::Times);
                last_token_was_operand = false;
                last_token_was_die = false;
            }
            '/' => {
                let token_precedence = operator_precedence(Token::Divide);
                while let Some(&top) = operator_stack.last() {
                    if let Token::L = top {
                        break;
                    } else {
                        let top_precedence = operator_precedence(top);
                        if token_precedence <= top_precedence {
                            output_queue.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                }
                operator_stack.push(Token::Divide);
                last_token_was_operand = false;
                last_token_was_die = false;
            }
            '^' => {
                let token_precedence = operator_precedence(Token::Exponent);
                while let Some(&top) = operator_stack.last() {
                    //if let Token::L = top {
                    //break;
                    //} else {
                    let top_precedence = operator_precedence(top);
                    if token_precedence <= top_precedence {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                //}
                operator_stack.push(Token::Exponent);
                last_token_was_operand = false;
                last_token_was_die = false;
            }
            'd' => {
                if last_token_was_operand {
                    if let Token::Operand(die_count) = output_queue.pop().unwrap() {
                        output_queue.push(Token::Dice(Die {
                            number: die_count,
                            sides: 0,
                        }))
                    }
                } else {
                    output_queue.push(Token::Dice(Die {
                        number: 1,
                        sides: 0,
                    }))
                }
                last_token_was_operand = false;
                last_token_was_die = true;
            }
            _ => panic!("Invalid token: {}", token),
        }
    }

    while let Some(operator) = operator_stack.pop() {
        output_queue.push(operator);
    }

    output_queue
}

fn operator_precedence(token: Token) -> i32 {
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Times | Token::Divide => 2,
        Token::Exponent => 3,
        Token::Operand(_) => panic!("Expected operator, found operand"),
        Token::L => 4,
        Token::Dice(_) => panic!("Expected operator, found operand"),
    }
}
// fn main() {
//     let infix_expression = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
//     let postfix_expression = infix_to_postfix(infix_expression);
//     println!("Infix expression: {}", infix_expression);
//     println!("Postfix expression: {:?}", postfix_expression);
//     let infix_expression = "256*2";
//     let postfix_expression = infix_to_postfix(infix_expression);
//     println!("Infix expression: {}", infix_expression);
//     println!("Postfix expression: {:?}", postfix_expression);
//     let infix_expression = "3+-2";
//     let postfix_expression = infix_to_postfix(infix_expression);
//     println!("Infix expression: {}", infix_expression);
//     println!("Postfix expression: {:?}", postfix_expression);
//     let infix_expression = "3+2*7+3-1/2";
//     let postfix_expression = infix_to_postfix(infix_expression);
//     println!("Infix expression: {}", infix_expression);
//     println!("Postfix expression: {:?}", postfix_expression);
//     let infix_expression = "1d5+3";
//     letopostfix_expression = infix_to_postfix(infix_expression);
//     println!("Infix expression: {}", infix_expression);
//     println!("Postfix expression: {:?}", postfix_expression);
//     // let me = Equation::new("1d5+4*2");
//     // //println!("{:?}", me);
//     // println!("{}", me.roll());
// }
