use dice_forge::roll;
use dice_forge::Equation;

fn main() {
    let my_equation = match Equation::new("3d5") {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };
    let my_roll = my_equation.roll().unwrap();
    println!("my_roll:{}", my_roll);

    let doc_equation = match Equation::new("3d5+10/2^2") {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            panic!()
        }
    };
    println!("You rolled {}", doc_equation.roll().unwrap());
    println!("Average roll {}", doc_equation.average().unwrap());
    let (low, high) = doc_equation.range().unwrap();
    println!("range {} to {}", low, high);
    println!("lowest possable number {}", doc_equation.low().unwrap());
    println!("highest possable number {}", doc_equation.high().unwrap());
    match roll::roll("test") {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}
