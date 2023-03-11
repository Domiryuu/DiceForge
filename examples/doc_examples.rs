use dice_forge::Equation;

fn main() {
    let my_equation = match Equation::new("3d5") {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };
    let my_roll = my_equation.roll();
    println!("my_roll:{}", my_roll);

    let doc_equation = match Equation::new("3d5+10/2^2") {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };
    println!("You rolled {}", doc_equation.roll());
    println!("Average roll {}", doc_equation.average());
    let (low, high) = doc_equation.range();
    println!("range {} to {}", low, high);
    println!("lowest possable number {}", doc_equation.low());
    println!("highest possable number {}", doc_equation.high());
}
