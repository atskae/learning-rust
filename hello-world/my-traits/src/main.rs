trait Pokemon {
    fn cry(&self);
    fn attack(&self);
}

struct Pikachu {
    cry: String,
    attack: String,
}

struct Magikarp {
    level: i32,
}

impl Pokemon for Pikachu {
    fn cry(&self) {
        println!("{:#}! {:#}!", self.cry, self.cry);
    }
    fn attack(&self) {
        println!("Pikachu used {:#}!", self.attack);
    }
}

impl Pokemon for Magikarp {
    fn cry(&self) {
        println!("Splash.... splash...");
    }
    fn attack(&self) {
        println!("Just flopping about... But nothing happened...");
    }
}

fn main() {
    println!("ポケモン traits and stuff");

    let pikachu = Pikachu { 
        cry: String::from("pika"),
        attack: String::from("thunderbolt")
    };

    let magikarp = Magikarp { level: 5 };

    pikachu.cry();
    magikarp.cry();
    
    pikachu.attack();
    magikarp.attack();
}
