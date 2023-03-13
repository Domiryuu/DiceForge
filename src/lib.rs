//! Dice Forge is a Rust library for parsing and evaluating mathematical expressions that include dice rolls.
//! It takes into account the order of operations and supports standard mathematical operators, such as addition (+),
//!  subtraction (-), multiplication (*), division (/), and exponentiation (^).
//!
//! To use the library, you create an Equation object by passing a string representing a valid mathematical expression
//! that can include dice notation. Dice notation should be in the format "NdM" where N is the number of dice to roll,
//! and M is the number of sides on each die. For example, "2d6" would roll two six-sided dice.
//!
//! Once you have an Equation object, you can call its roll() method to evaluate the expression and obtain a result.
//! You can also call its high(), low(), average(), or range() methods to obtain information about the
//! highest possible result, lowest possible result, average result, or range of possible results, respectively.
//! their are also methods to roll with advantage() or disadvantage() check the rest of the documentation
//! for a full list and examples.
//!
//! Here's an example of using the Dice Forge library to roll a 3d6+2 attack:
//! ```
//! use dice_forge::Equation;
//!
//! let attack = Equation::new("3d6+2").unwrap();
//! let damage = attack.roll().unwrap();
//!
//! println!("You attack for {} damage", damage);
//! ```
//! In this example, the Equation::new() method creates a new Equation object representing the expression "3d6+2",
//! and the attack.roll() method rolls the dice and evaluates the expression to obtain a result.
//! The unwrap() method is used to extract the actual result from the Result object returned by roll(), assuming the
//! dice roll is successful.
//!
//! For one off equations that you do not want to save a compiled version for future rolls you can use dice_forge::roll
//! and call it's roll function
//! ```
//! use dice_forge::roll;
//!
//! let damage = roll::roll("3d6+2").unwrap();
//!
//! println!("You attack for {} damage", damage);
//! ```
//! In this example the roll function will take care of all computation and retun the Result of the equation without breaking
//! the steps up into different parts.

pub mod equation;
pub mod roll;
pub use equation::Equation;
mod errors;
