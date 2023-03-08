mod roll;

/// struct containing the Equation compiled for faster evaluation
///
/// # Example
///
/// ```
/// let my_equation = Equation::new("3d5");
/// let my_roll = my_equation.roll();
/// ````
pub struct Equation {
    compiled_equation: Vec<Token>,
}
impl Equation {
    /// compiles and returns a new Equation
    /// can be used to rapidly roll the same dice equation or to preform basic math without dice rolls
    ///
    /// # Example
    ///
    /// ```
    /// Equation::new("3d5+10/2^2");
    /// ````
    pub fn new(input: &str) -> Self {
        let compiled_equation = infix_to_postfix(input);
        Equation { compiled_equation }
    }
    /// rolls the Equation preforms basic math returning the product
    ///
    /// # Example
    ///
    /// ```
    /// println!("you rolled {}", Equation::new("3d5+10/2^2").roll());
    /// ````
    pub fn roll(&self) -> i64 {
        //     todo!();
        roll::process(self, RollType::Default)
    }
    /// calculates the product of the equation assuming the average roll of all die in the equation
    /// if no die are present in the equation result will be the same as roll()
    ///
    /// # Example
    ///
    /// ```
    /// println!("average roll {}", Equation::new("3d5+10/2^2").average());
    /// ````
    pub fn average(&self) -> i64 {
        roll::process(self, RollType::Average)
    }
    ///calculates the product resulting from both the highest and lowest possable rolls to give you the range
    /// if no die notation is present in the equation both numbers will be the same as roll()
    ///
    /// # Example
    ///
    /// ```
    /// let (low, high) = Equation::new("3d5+10/2^2").range();
    /// println!("{} to {}", high, low);
    /// ````
    pub fn range(&self) -> (i64, i64) {
        let low = roll::process(self, RollType::Low);
        let high = roll::process(self, RollType::High);
        (low, high)
    }
    /// calculates the lowest possable number given the die
    /// if no die notation is present in the equation this number will be the same as roll()
    ///
    /// # Example
    ///
    /// ```
    /// println!("lowest number possable: {}", Equation::new("3d5+10/2^2").low());
    /// ````
    pub fn low(&self) -> i64 {
        roll::process(self, RollType::Low)
    }
    /// calculates the highest possable number given the die
    /// if no die notation is present in the equation this number will be the same as roll()
    ///
    /// # Example
    ///
    /// ```
    /// println!("Highest number possable: {}", Equation::new("3d5+10/2^2").high());
    /// ````
    pub fn high(&self) -> i64 {
        roll::process(self, RollType::High)
    }
}
enum RollType {
    Default,
    Average,
    Low,
    High,
}
// trait Rollable {
//     fn roll(&self) -> i64;
// }
// impl Rollable for Equation {
//     fn roll(&self) -> i64 {
//         roll::process(self)
//     }
// }
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

// #[cfg(test)]
// #[test]
// fn infix() {
//     assert_eq!("", format!("{:?}", infix_to_postfix("2(1-5)^2+3*2")))
// }
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
