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
    /// Compiles and returns a new `Equation` object.
    ///
    /// The `input` parameter should be a string representing a valid mathematical equation. The equation can
    /// include dice notation in the format "NdM" where N is the number of dice to roll, and M is the number
    /// of sides on each die. For example, "2d6" would roll two six-sided dice. The equation can also include
    /// standard mathematical operators such as addition (+), subtraction (-), multiplication (*), and division (/).
    /// Parentheses can be used to group sub-expressions together.
    ///
    /// The function compiles the equation into a postfix format that is optimized for efficient evaluation.
    /// The resulting `Equation` object can then be used to roll the dice and perform basic math operations
    /// without the need for recompilation.
    ///
    /// # Example
    ///
    /// Creating a new `Equation` object and rolling the dice:
    ///
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let my_equation = Equation::new("3d5+10/2^2").unwrap();
    /// let result = my_equation.roll().unwrap();
    /// println!("Result: {}", result);
    /// ```
    pub fn new(input: &str) -> Result<Equation, errors::InvalidExpressionError> {
        let compiled_equation = infix_to_postfix(input)?;
        Ok(Equation { compiled_equation })
    }
    /// Rolls the given `Equation` object.
    ///
    /// The `input` parameter should be a string representing a valid mathematical equation that can include
    /// dice notation. Dice notation should be in the format "NdM" where N is the number of dice to roll,
    /// and M is the number of sides on each die. For example, "2d6" would roll two six-sided dice.
    /// Dice notation can also be combined with standard mathematical operators, such as addition (+),
    /// subtraction (-), multiplication (*), division (/), and exponent (^). Parentheses can also be used to group
    /// sub-expressions together. For example, "10+(3+2d6*2)+3(2d20)+d2" is a valid equation that includes
    /// dice notation.
    ///
    /// # Examples
    ///
    /// Rolling 1d4:
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let my_equation = Equation::new("1d4").unwrap();
    /// let result = my_equation.roll().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    ///
    /// Rolling 2d6 a modifier:
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let my_equation = Equation::new("2d6+4").unwrap();
    /// let result = my_equation.roll().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    ///
    /// Rolling a more complex equation:
    /// ```
    /// use dice_forge::Equation;
    /// let my_equation = Equation::new("10+(3+2d6*2)+3(2d20)+d2").unwrap();
    /// let result = my_equation.roll().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    #[inline(always)]
    pub fn roll(&self) -> Result<i32, errors::InvalidExpressionError> {
        //     todo!();
        Ok(roll::process(self, RollType::Default)?)
    }
    /// calculates the product of the equation assuming the average roll of all die in the equation
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
    /// Calculates the range of possible values that can be produced by the equation.
    ///
    /// The range is calculated by finding the product of the highest and lowest possible rolls for
    /// all dice in the equation. Note that this calculation will not take into account any additional
    /// mathematical operations in the equation, and may not accurately represent the true range of
    /// possible values.
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let (low, high) = Equation::new("3d5+10/2^2").unwrap().range().unwrap();
    ///
    /// println!("Range: {} - {}", low, high);
    /// ```
    #[inline(always)]
    pub fn range(&self) -> Result<(i32, i32), errors::InvalidExpressionError> {
        let low = roll::process(self, RollType::Low)?;
        let high = roll::process(self, RollType::High)?;
        Ok((low, high))
    }
    /// Calculates the lowest possible value that can be produced by the equation.
    ///
    /// The value is calculated by finding the product of the lowest possible rolls for
    /// all dice in the equation. Note that this calculation will not take into account any additional
    /// mathematical operations in the equation, and may not accurately represent the true lowest of
    /// possible values.
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let low = Equation::new("3d5+10/2^2").unwrap().low().unwrap();
    ///
    /// println!("Low: {}", low);
    /// ```
    #[inline(always)]
    pub fn low(&self) -> Result<i32, errors::InvalidExpressionError> {
        Ok(roll::process(self, RollType::Low)?)
    }
    /// Calculates the highest possible value that can be produced by the equation.
    ///
    /// The value is calculated by finding the product of the highest possible rolls for
    /// all dice in the equation. Note that this calculation will not take into account any additional
    /// mathematical operations in the equation, and may not accurately represent the true highest of
    /// possible values.
    ///
    /// # Example
    ///
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let high = Equation::new("3d5+10/2^2").unwrap().high().unwrap();
    ///
    /// println!("High: {}", high);
    /// ```
    #[inline(always)]
    pub fn high(&self) -> Result<i32, errors::InvalidExpressionError> {
        Ok(roll::process(self, RollType::High)?)
    }
    /// Rolls the given `Equation` object with advantage.
    ///
    /// The `input` parameter should be a string representing a valid mathematical equation that can include
    /// dice notation. Dice notation should be in the format "NdM" where N is the number of dice to roll,
    /// and M is the number of sides on each die. For example, "2d6" would roll two six-sided dice.
    /// Dice notation can also be combined with standard mathematical operators, such as addition (+),
    /// subtraction (-), multiplication (*), division (/), and exponent (^). Parentheses can also be used to group
    /// sub-expressions together. For example, "10+(3+2d6*2)+3(2d20)+d2" is a valid equation that includes
    /// dice notation.
    ///
    /// The function rolls the given equation twice and returns the greater of the two results. This
    /// emulates the "advantage" mechanic in some games, where a player can roll two dice and take the greater result.
    ///
    /// # Examples
    ///
    /// Rolling 1d4:
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let my_equation = Equation::new("1d4").unwrap();
    /// let result = my_equation.advantage().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    ///
    /// Rolling 2d6 a modifier:
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let my_equation = Equation::new("2d6+4").unwrap();
    /// let result = my_equation.advantage().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    ///
    /// Rolling a more complex equation:
    /// ```
    /// use dice_forge::Equation;
    /// let my_equation = Equation::new("10+(3+2d6*2)+3(2d20)+d2").unwrap();
    /// let result = my_equation.advantage().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    #[inline(always)]
    pub fn advantage(&self) -> Result<i32, errors::InvalidExpressionError> {
        let r1 = self.roll()?;
        let r2 = self.roll()?;
        Ok(std::cmp::max(r1, r2))
    }
    /// Rolls the given `Equation` object with disadvantage.
    ///
    /// The `input` parameter should be a string representing a valid mathematical equation that can include
    /// dice notation. Dice notation should be in the format "NdM" where N is the number of dice to roll,
    /// and M is the number of sides on each die. For example, "2d6" would roll two six-sided dice.
    /// Dice notation can also be combined with standard mathematical operators, such as addition (+),
    /// subtraction (-), multiplication (*), division (/), and exponent (^). Parentheses can also be used to group
    /// sub-expressions together. For example, "10+(3+2d6*2)+3(2d20)+d2" is a valid equation that includes
    /// dice notation.
    ///
    /// The function rolls the given equation twice and returns the lesser of the two results. This
    /// emulates the "disadvantage" mechanic in some games, where a player can roll two dice and take the lesser result.
    ///
    /// # Examples
    ///
    /// Rolling 1d4:
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let my_equation = Equation::new("1d4").unwrap();
    /// let result = my_equation.disadvantage().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    ///
    /// Rolling 2d6 a modifier:
    /// ```
    /// use dice_forge::Equation;
    ///
    /// let my_equation = Equation::new("2d6+4").unwrap();
    /// let result = my_equation.disadvantage().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    ///
    /// Rolling a more complex equation:
    /// ```
    /// use dice_forge::Equation;
    /// let my_equation = Equation::new("10+(3+2d6*2)+3(2d20)+d2").unwrap();
    /// let result = my_equation.disadvantage().unwrap();
    ///
    /// println!("Result: {}", result);
    /// ```
    #[inline(always)]
    pub fn disadvantage(&self) -> Result<i32, errors::InvalidExpressionError> {
        let r1 = self.roll()?;
        let r2 = self.roll()?;
        Ok(std::cmp::min(r1, r2))
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
