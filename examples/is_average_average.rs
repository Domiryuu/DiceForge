use dice_forge::Equation;

fn main() {
    let my_die = Equation::new("d100").expect("handle the error in real code");
    let mut results: u64 = 0;
    let mut hit_low = false;
    let mut hit_high = false;
    let mut highest: u64 = 0;
    let mut lowest: u64 = 100;
    for _n in 0..10000 {
        let rez = my_die.roll().unwrap();
        if rez == 1 {
            hit_low = true;
        } else if rez == 100 {
            hit_high = true;
        }
        if (rez as u64) < lowest {
            lowest = rez as u64;
        }
        if (rez as u64) > highest {
            highest = rez as u64;
        }
        results += my_die.roll().unwrap() as u64;
    }
    println!("average: {}", results / 10000);
    println!("Hit High: {}, number: {}", hit_high, highest);
    println!("Hit Low: {}, number: {}", hit_low, lowest);
    println!("single roll {}", my_die.roll().unwrap());
}
