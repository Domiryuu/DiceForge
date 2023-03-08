use rand::Rng;
use Die;
use Equation;
use Token;

pub(super) fn process(equation: &Equation) -> f64 {
    // todo!();
    let stack = Vec::new();
    for token in equation.compiled_equation {
        match token {
            Token::Operand(value) => stack.push(value as i64),
            Token::Dice(T) => roll(T),
        }
    }
    69.0
}
fn roll(die: Die) -> i64 {
    // todo!();
    let mut rng = rand::thread_rng();
    let mut current: i64 = 0;
    //let die = dice.unwrap();
    for n in 0..die.number {
        current = current + rng.gen_range(1..die.sides);
    }
    current
}
// pub(super) trait roll {
//     pub(super) fn roll() -> f64;
// }
// pub(super) impl roll for Equation {
//     fn roll() -> f64 {
//         todo!();
//     }
// }
