use crate::equation;
use crate::equation::{Die, Equation, Token};
use crate::errors::InvalidExpressionError;
use rand::Rng;

pub(super) fn process(
    equation: &Equation,
    ty: equation::RollType,
) -> Result<i32, InvalidExpressionError> {
    let mut stack: Vec<i32> = Vec::with_capacity(equation.compiled_equation.len());
    for token in &equation.compiled_equation {
        match *token {
            Token::Operand(value) => stack.push(value as i32),
            Token::Dice(die) => match ty {
                equation::RollType::Default => stack.push(roll_die(die)),
                equation::RollType::Low => stack.push(die.number as i32),
                equation::RollType::High => stack.push((die.number * die.sides) as i32),
                equation::RollType::Average => {
                    stack.push((die.number as f32 * (die.sides as f32 / 2.0 + 0.5)) as i32)
                }
            },
            Token::Plus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            Token::Minus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs - rhs);
            }
            Token::Times => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
            Token::Divide => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                if rhs == 0 {
                    return Err(InvalidExpressionError::DivideByZero);
                }
                stack.push(lhs / rhs);
            }
            //cant handle fractional exponents as I am too dumb to know how to do them trunkates the desimal
            Token::Exponent => {
                let ex = stack.pop().unwrap();
                let d = stack.pop().unwrap();
                match ex {
                    0 => stack.push(1),
                    1 => stack.push(d),
                    _ => {
                        let mut b: i32 = 1;
                        for _n in 0..ex {
                            b *= d;
                        }
                        stack.push(b);
                    }
                }
            }
            _ => {}
        }
    }
    Ok(stack.pop().unwrap())
}
fn roll_die(die: Die) -> i32 {
    let mut rng = rand::thread_rng();
    let mut current: i32 = 0;
    for _n in 0..die.number {
        current += rng.gen_range(1..die.sides + 1) as i32;
    }
    current
}
/// Rolls the given dice equation.
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
/// use dice_forge::roll;
///
/// let result = roll::roll("1d4").unwrap();
/// println!("Result: {}", result);
/// ```
///
/// Rolling 2d6 a modifier:
/// ```
/// use dice_forge::roll;
///
/// let result = roll::roll("2d6+4").unwrap();
/// println!("Result: {}", result);
/// ```
///
/// Rolling a more complex equation:
/// ```
/// use dice_forge::roll;
///
/// let result = roll::roll("10+(3+2d6*2)+3(2d20)+d2").unwrap();
/// println!("Result: {}", result);
/// ```
pub fn roll(input: &str) -> Result<i32, InvalidExpressionError> {
    let compiled_equation = equation::infix_to_postfix(input)?;
    let a = Equation { compiled_equation };
    a.roll()
}
/// Rolls the given dice equation with advantage.
///
/// The `input` parameter should be a string representing a valid mathematical equation that can include
/// dice notation. Dice notation should be in the format "NdM" where N is the number of dice to roll,
/// and M is the number of sides on each die. For example, "2d6" would roll two six-sided dice.
/// Dice notation can also be combined with standard mathematical operators, such as addition (+),
/// subtraction (-), multiplication (*), division (/), and exponent (^). Parentheses can also be used to group
/// sub-expressions together. For example, "10+(3+2d6*2)+3(2d20)+d2" is a valid equation that includes
/// dice notation.
///
/// The function rolls the given equation twice and returns the higher of the two results. This
/// emulates the "advantage" mechanic in some games, where a player can roll two dice and take the higher result.
///
/// # Examples
///
/// Rolling 1d4 with advantage:
/// ```
/// use dice_forge::roll;
///
/// let result = roll::advantage("1d4").unwrap();
/// println!("Result: {}", result);
/// ```
///
/// Rolling 2d6 with advantage and a constant modifier:
/// ```
/// use dice_forge::roll;
///
/// let result = roll::advantage("2d6+4").unwrap();
/// println!("Result: {}", result);
/// ```
///
/// Rolling a more complex equation with advantage:
/// ```
/// use dice_forge::roll;
///
/// let result = roll::advantage("10+(3+2d6*2)+3(2d20)+d2").unwrap();
/// println!("Result: {}", result);
/// ```
pub fn advantage(input: &str) -> Result<i32, InvalidExpressionError> {
    let compiled_equation = equation::infix_to_postfix(input)?;
    let a = Equation { compiled_equation };
    a.advantage()
}
/// Rolls the given dice equation with disadvantage.
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
/// Rolling 1d4 with disadvantage:
/// ```
/// use dice_forge::roll;
///
/// let result = roll::disadvantage("1d4").unwrap();
/// println!("Result: {}", result);
/// ```
///
/// Rolling 2d6 with disadvantage and a constant modifier:
/// ```
/// use dice_forge::roll;
///
/// let result = roll::disadvantage("2d6+4").unwrap();
/// println!("Result: {}", result);
/// ```
///
/// Rolling a more complex equation with disadvantage:
/// ```
/// use dice_forge::roll;
///
/// let result = roll::disadvantage("10+(3+2d6*2)+3(2d20)+d2").unwrap();
/// println!("Result: {}", result);
/// ```
pub fn disadvantage(input: &str) -> Result<i32, InvalidExpressionError> {
    let compiled_equation = equation::infix_to_postfix(input)?;
    let a = Equation { compiled_equation };
    a.disadvantage()
}
