[![Rust](https://github.com/Domiryuu/DiceForge/actions/workflows/rust.yml/badge.svg)](https://github.com/Domiryuu/DiceForge/actions/workflows/rust.yml)
[![Docs](https://docs.rs/dice_forge/badge.svg)](https://docs.rs/dice_forge)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)]([https://opensource.org/licenses/Apache-2.0](https://github.com/Domiryuu/DiceForge/blob/master/LICENSE-APACHE))
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Domiryuu/DiceForge/blob/master/LICENSE-MIT)

# DiceForge
DiceForge is a Rust library for simulating dice rolls. It provides a simple and easy-to-use API for generating random dice rolls and calculating probabilities of different outcomes.

## Features

Supports standard dice notation (e.g. 1d6, 2d10, etc.)
Add, subtract, or even multiply modifiers to rolls

## Installation

To use DiceForge in your Rust project, add the following to your Cargo.toml file:
```toml
[dependencies]
dice_forge = "0.2.2"
```

## Usage

To roll a standard six-sided die, you can use the roll function on a compiled `Equation` object:
```rust
use dice_forge::Equation;

let die = Equation::new("1d6").unwrap();
let roll = die.roll().unwrap();
println!("You Rolled a {}", roll);
```
You can also add modifiers to your rolls:
```rust
use dice_forge::Equation;

let die = Equation::new("1d6+5").unwrap();
let roll = die.roll().unwrap();
println!("You Rolled a {} with a +5 modifier", roll);
```

It even takes order of operations into consideration so you could write an equation as complicated as
```rust
use dice_forge::Equation;

let die = Equation::new("3+1d6(4+3^2)-6/2").unwrap();
let roll = die.roll().unwrap();
println!("You Rolled a {}", roll);
```

If you do not want to create an `Equation` object you can directly roll the equation:
```rust
use dice_forge::roll;

let damage = roll::roll("1d6").unwrap();
println!("You do {} damage", damage);
```

You can also roll with advantage or disadvantage:
```rust
use dice_forge::Equation;
let die = Equation::new("d20").unwrap();
let mut roll = die.advantage().unwrap();
println!("You rolled {} with advantage", roll);
roll = die.disadvantage().unwrap();
println!("You rolled {} with disadvantage", roll);
```

You can check out the rest of the documentation at [docs.rs](https://docs.rs/dice_forge/)

## Contributing

Contributions are welcome! If you find a bug or want to add a new feature, please submit a pull request on GitHub.
