use dice_forge::Equation;

fn main() {
    let my_equation = Equation::new("3d5");
    let my_roll = my_equation.roll();
    println!("my_roll:{}", my_roll);

    let doc_equation = Equation::new("3d5+10/2^2");
    println!("You rolled {}", doc_equation.roll());
    println!("Average roll {}", doc_equation.average());
    let (low, high) = doc_equation.range();
    println!("range {} to {}", low, high);
    println!("lowest possable number {}", doc_equation.low());
    println!("highest possable number {}", doc_equation.high());
}
