//mod roll;
use crate::errors;
use crate::roll;

/// struct containing the Equation compiled for faster evaluation
///
/// # Example
///
/// ```
/// use dice_forge::Equation;
/// let my_equation = Equation::new("3d5").unwrap();
/// let my_roll = my_equation.roll().unwrap();
/// ````
pub struct Equation {
    pub(crate) compiled_equation: Vec<Token>,
}
impl Equation {
    /// compiles and returns a new Equation
    /// can be used to rapidly roll the same dice equation or to preform basic math without dice rolls
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    /// Equation::new("3d5+10/2^2");
    /// ```
    pub fn new(input: &str) -> Result<Equation, errors::InvalidExpressionError> {
        let compiled_equation = infix_to_postfix(input)?;
        Ok(Equation { compiled_equation })
    }
    /// rolls the Equation preforms basic math returning the product
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    /// println!("you rolled {}", Equation::new("3d5+10/2^2").unwrap().roll().unwrap());
    /// ````
    #[inline(always)]
    pub fn roll(&self) -> Result<i32, errors::InvalidExpressionError> {
        //     todo!();
        Ok(roll::process(self, RollType::Default)?)
    }
    /// calculates the product of the equation assuming the average roll of all die in the equation
    /// if no die are present in the equation result will be the same as roll()
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    /// println!("average roll {}", Equation::new("3d5+10/2^2").unwrap().average().unwrap());
    /// ````
    #[inline(always)]
    pub fn average(&self) -> Result<i32, errors::InvalidExpressionError> {
        Ok(roll::process(self, RollType::Average)?)
    }
    ///calculates the product resulting from both the highest and lowest possable rolls to give you the range
    /// if no die notation is present in the equation both numbers will be the same as roll()
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    /// let (low, high) = Equation::new("3d5+10/2^2").unwrap().range().unwrap();
    /// println!("{} to {}", high, low);
    /// ````
    #[inline(always)]
    pub fn range(&self) -> Result<(i32, i32), errors::InvalidExpressionError> {
        let low = roll::process(self, RollType::Low)?;
        let high = roll::process(self, RollType::High)?;
        Ok((low, high))
    }
    /// calculates the lowest possable number given the die
    /// if no die notation is present in the equation this number will be the same as roll()
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    /// println!("lowest number possable: {}", Equation::new("3d5+10/2^2").unwrap().low().unwrap());
    /// ````
    #[inline(always)]
    pub fn low(&self) -> Result<i32, errors::InvalidExpressionError> {
        Ok(roll::process(self, RollType::Low)?)
    }
    /// calculates the highest possable number given the die
    /// if no die notation is present in the equation this number will be the same as roll()
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    /// println!("Highest number possable: {}", Equation::new("3d5+10/2^2").unwrap().high().unwrap());
    /// ````
    #[inline(always)]
    pub fn high(&self) -> Result<i32, errors::InvalidExpressionError> {
        Ok(roll::process(self, RollType::High)?)
    }
}
pub(crate) enum RollType {
    Default,
    Average,
    Low,
    High,
}

#[derive(Clone, Copy)]
pub(crate) enum Token {
    Operand(u32),
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
    L,
    Dice(Die),
}
#[derive(Clone, Copy)]
pub(crate) struct Die {
    pub(crate) number: u32,
    pub(crate) sides: u32,
}

pub(crate) fn infix_to_postfix(input: &str) -> Result<Vec<Token>, errors::InvalidExpressionError> {
    let mut output_queue: Vec<Token> = Vec::with_capacity(input.len());
    let mut operator_stack: Vec<Token> = Vec::with_capacity(input.len());
    let mut last_token_was_operand = false;
    let mut last_token_was_die = false;
    let mut error = None;

    for token in input.chars().filter(|c| !c.is_whitespace()) {
        match token {
            '0'..='9' => {
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
                    let top_precedence = operator_precedence(top);
                    if token_precedence <= top_precedence {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
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
            _ => {
                error = Some(errors::InvalidExpressionError::InvalidToken(token));
                break;
            }
        }
    }

    if let Some(err) = error {
        return Err(err);
    }

    while let Some(operator) = operator_stack.pop() {
        output_queue.push(operator);
    }

    Ok(output_queue)
}
#[inline(always)]
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
