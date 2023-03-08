[![Rust](https://github.com/Domiryuu/dicey/actions/workflows/rust.yml/badge.svg)](https://github.com/Domiryuu/dicey/actions/workflows/rust.yml) [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)]([https://opensource.org/licenses/Apache-2.0](https://github.com/Domiryuu/dicey/blob/master/LICENSE-APACHE)) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Domiryuu/dicey/blob/master/LICENSE-MIT)

# Dicey
Dicey is a Rust library for simulating dice rolls. It provides a simple and easy-to-use API for generating random dice rolls and calculating probabilities of different outcomes.

## Features

Supports standard dice notation (e.g. 1d6, 2d10, etc.)
Add, subtract, or even multiply modifiers to rolls

## Installation

To use Dicey in your Rust project, add the following to your Cargo.toml file:
```toml
[dependencies]
dicey = "0.1.0"
```

## Usage

To roll a standard six-sided die, you can use the roll function:
```rust
use dicey::Equation;

let die = Equation::new("1d6");
println!("You Rolled a {}", die.roll());
```
You can also add modifiers to your rolls:
```rust
use dicey::Equation;

let die = Equation::new("1d6+5");
println!("You Rolled a {} with a +5 modifier", die.roll());
```

It even takes order of operations into consideration so you could write an equation as complicated as
```rust
use dicey::Equation;

let die = Equation::new("3+1d6(4+3^2)-6/2");
println!("You Rolled a {}", die.roll());
```

## Contributing

Contributions are welcome! If you find a bug or want to add a new feature, please submit a pull request on GitHub.
