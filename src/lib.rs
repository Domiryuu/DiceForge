//! Compiled dice notation for rapid evaluation.
//!
//! use Dicey::Equation;
//! Dicey::Equation::new("3d6+2")
//!
//! you can then roll() the Equation
//! or you can check its high(), low() average() or range()
//!
//! this library takes into account order of operations and can handle
//! '/', '*', '^', 'dice notation(d3)', '+', '-'
//! as of yet has a verbos print. currently only gives the product of the evalutaion

pub mod equation;
pub use equation::Equation;
mod errors;
