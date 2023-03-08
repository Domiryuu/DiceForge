//use dicey::Equation;

struct Person {
    health: i32,
    damage: dice_forge::Equation,
}

impl Person {
    fn new(health: i32, d: &str) -> Self {
        let damage = dice_forge::Equation::new(d);
        Person { health, damage }
    }
    fn attack(&self) -> i64 {
        self.damage.roll()
    }
    // fn damage(mut self, d: i64) {
    //     self.health -= d as i32;
    // }
}

fn main() {
    let person1 = Person::new(100, "2d20");
    let person2 = Person::new(100, "3d10");
    let mut turns = 0;
    let mut health = person2.health;
    while health > 0 {
        turns += 1;
        health -= person1.attack() as i32;
    }
    println!("It took {} turns for person 1 to kill person 2", turns);
    health = person1.health;
    while health > 0 {
        turns += 1;
        health -= person2.attack() as i32;
    }
    println!("It took {} turns for person 2 to kill person 1", turns);
}
