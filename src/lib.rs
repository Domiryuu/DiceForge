//! Compiled dice notation for rapid evaluation.
//!
//! ```rust
//! use dice_forge::Equation;
//! let attack = Equation::new("3d6+2").unwrap();
//! println!("You attack for {} damage", attack.roll().unwrap());
//! ```
//!
//! you can then roll() the Equation
//! or you can check its high(), low() average() or range()
//!
//! this library takes into account order of operations and can handle
//! '/', '*', '^', 'dice notation', '+', '-'
//! as of yet has a verbos print. currently only gives the product of the evalutaion
//! high() and low() check for lowest rolls or highest rolls not highest or lowest possable values of the equation as a whole

pub mod equation;
mod roll;
pub use equation::Equation;
mod errors;
