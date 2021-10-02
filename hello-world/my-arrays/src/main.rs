#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32) // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
}

fn car_quality(miles: u32) -> (Age, u32) {
    let quality = if miles == 0 {
        (Age::New, miles)
    } else {
        (Age::Used, miles)
    };
    
    return quality;
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type 
    // - Print details for New or Used car based on roof type
    let car_age = car_quality(miles);
    println!("Got {:?}", car_age);
    if car_age.0 == Age::Used {
        if roof == true {
            println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);
        }
    }
    // todo!("Add conditional expression: If car is Used age, then check roof type");
        //todo!("Add conditional expression: If roof is a hard top, print details");
            // Call the `println!` macro to show the car order details
            //println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);
    
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument 
    let car = Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_age, // todo!("Replace `mileage: miles` with `age` tuple field, call `car_quality()` with `miles` as input argument");
    };

    return car;
}

fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"]; //todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");

    // Declare the car type and initial values
    let mut car = Car {
        color: String::from("Red"),
        motor: Transmission::Manual,
        roof: false,
        age: car_quality(0)
    }; //todo!("Create `car` as a `Car` struct");     
    
    let mut engine: Transmission = Transmission::Manual; // todo!("Declare `engine` as a `Transmission` enum, initialize to `Manual`");

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[3]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);


    println!("Next exercise ---- <>< ");

    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
