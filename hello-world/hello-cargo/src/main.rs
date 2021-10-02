// Import arrays module
mod my_arrays;

fn test_enum() {
    // This is weird that it is allowed 
    //enum Event {
    //     ClockStarted,
    //     ClockRunning(f32),
    //     ClockStopped {  x: i32, y: i32 }
    // }

    // Still bothers me that tuples need a semicolon but
    // classical structs don't???
    #[derive(Debug)] // add this statement so we can print its contents
    // with `println!()`
    struct MouseClick { x: i32, y: i32 }
    
    #[derive(Debug)]
    struct KeyPress(String, char);
    
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress)
    }

    // Instantiate
    let loaded = WebEvent::WELoad(true);
    let click = MouseClick { x: 25, y: 45 };

    let we_click = WebEvent::WEClick(click);

    let shift_a = KeyPress(String::from("shift"), 'A');
    let key_press_event = WebEvent::WEKeys(shift_a);

    println!("key_press_event: {:#?}", key_press_event);
}

fn test_structs() {
    // Define some
    // No semicolon needed
    struct Pokemon { name: String, level: u8 }
    
    // A tuple struct defines the types of a tuple that can
    // be instantiated später
    // But this needs a semicolon? Warum???
    struct Pet(String, i32);

    // Let's instantiate some!!
    let one = Pokemon { name: String::from("Ampharos"), level: 5 };
    println!("My favorite Pokémon: {}, level {}", one.name, one.level);

    let pet = Pet(String::from("roger"), 5);
    println!("Class pet: {}, {}", pet.0, pet.1);
}

fn test_tuple() {
    println!("Define a tuple!");
    let pet = ("roger", 5i32);
    println!("My pet tuple: {}", pet.0);
}

fn main() {
    //todo!("Learn Rust");
    println!("Hallo, Welt!");
    println!("I like {}'s way of substituting arguments better...", "Python");

    let pikachu = "Arnold";
    println!("Who's that Pokémon? It's {}!", pikachu);
    println!("{}! {}!", pikachu, pikachu);
    // can't do let mut pikachu: str

    let mut level = 20;
    println!("My level ist: {}.", level);
    level = level + 1; 
    println!("After a rare candy, my level ist: {}.", level);

    // Variable shadowing
    // A fancy name for something that sounds intuitive?
    let timestamp = 1;
    let timestamp = timestamp + 1;
    let timestamp = timestamp + 1;
    println!("The timestamp is now {}", timestamp);

    // Type annotations
    let number: u32 = 151;
    println!("{} の喜び", number);

    // Specify type to print
    println!("Some nombres with types: {}, {}, y {}!", 45u32, 10.0f32, 2f64 + 0.5);
    println!("Signed {}, unsigned {}", -45i8, 45u8);
    println!("What is isize? {}. What is usize? {}", 5isize, 5usize);

    // Let's print emoji!
    println!("えもじ: {}", '🎲');

    // str
    let trainer1 = "ブラックくん";
    let trainer2 = "ホワイト社長"; // now I want to know how this relationship came to be
    println!("I'm currently reading about {} and {}", trainer1, trainer2);
    // &str
    let trainer3: &str = "アロエ"; // not really a trainer but whatever
    //let trainer4: str = "that bug guy"; // will crash
    println!("What about &str? {}", trainer3);

    test_tuple();
    test_structs();
    
    test_enum();

    let my_name = "Alexander";
    let my_last_name = "Hamilton";
    test_pass_some_stuff(my_name);
    let val = test_pass_some_stuff(my_last_name);
    println!("Got the val: {}", val);

    car_exercise();

    my_arrays::test_my_arrays();

    my_arrays::test_conditionals();
}

// Can be defined after main()
fn test_pass_some_stuff(name: &str) -> bool {
    println!("What's my name? {}", name);
    return true;

    // Can do this too... I don't like this
    // Return a value with a return key word
    false
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual, SemiAuto, Automatic
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    age: (Age, u32)
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car = Car { 
        color: String::from(color),
        transmission: transmission,
        convertible: convertible,
        age: (Age::New, 0)
    };

    return car;
    //car // <- this works too, ugh
} 

fn car_exercise() {
    let car = car_factory(String::from("Red"), Transmission::Manual, false);

    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    
    // These names immediately reminded me of ポケスペ 笑`
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    //println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.age.1;

    //car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    //println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.age.1);

    //car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    //println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.age.1);

}
