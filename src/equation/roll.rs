use super::Die;
use super::Equation;
use super::Token;
use crate::equation;
use rand::Rng;

pub(super) fn process(equation: &Equation, ty: equation::RollType) -> i64 {
    // todo!();
    let mut stack = Vec::new();
    for token in &equation.compiled_equation {
        match *token {
            Token::Operand(value) => stack.push(value as i64),
            Token::Dice(die) => match ty {
                equation::RollType::Default => stack.push(roll(die)),
                equation::RollType::Low => stack.push(die.number as i64),
                equation::RollType::High => stack.push((die.number * die.sides) as i64),
                equation::RollType::Average => {
                    stack.push((die.number as f32 * (die.sides as f32 / 2.0 + 0.5)) as i64)
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
                    panic!("Divide by zero error");
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
                        let mut b: i64 = 1;
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
    stack.pop().unwrap()
}
fn roll(die: Die) -> i64 {
    // todo!();
    let mut rng = rand::thread_rng();
    let mut current: i64 = 0;
    //let die = dice.unwrap();
    for _n in 0..die.number {
        current += rng.gen_range(1..die.sides + 1) as i64;
    }
    current
}
