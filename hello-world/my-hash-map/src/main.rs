use std::collections::HashMap;

fn test_loops() {
    let mut counter = 1;
    let my_num = loop {
        counter += 1;
        if counter > 10 {
            break counter; // return counter value
        }
    };

    println!("My counter value: {}", counter);
    
    let starters = [
        "bulbasaur",
        "charmander",
        "squirtle",
        "pikachu",
        "chikorita",
        "cyndaquil",
        "totadile"
    ];

    for starter in starters {
        println!("I choose you!! {}", starter);
    }

    //// is this any different? Maybe for more complex types
    //for starter in starters.iter() {
    //    println!("I choose you!! {}", starter);
    //}

    for i in 0..5 {
        println!("My number ist: {}", i);
    }
}

fn main() {
    println!("Makin' some hash mapz.");

    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(
        String::from("Crying in Hmart"),
        String::from("Beautiful writing.")
    );

    reviews.insert(
        String::from("A Boy and His Dog..."),
        String::from("Walking around an island")
    );

    reviews.insert(
        String::from("ポケットモンスターspecial 45"),
        String::from("Is the girl not an actual trainer?")
    );

    let book: &str = "A Boy and His Dog...";
    println!("My book: {}: {:?}", book, reviews.get(book));

    reviews.remove(book);
    println!("My book: {}: {:?}", book, reviews.get(book));

    test_loops();
    //test_panic();
    test_option();
    test_if_let();
    test_unwrap();
    test_result();
}

#[derive(Debug)]
struct DivisionByZeroError;


fn safe_division(numerator: f64, denominator: f64) -> Result<f64, DivisionByZeroError> {
    if denominator == 0.0 { // interesting == works for floats...
        return Err(DivisionByZeroError);
    } else {
        return Ok(numerator / denominator);
    }
}

fn test_result() {
    println!("{:?}", safe_division(9.0, 30.0));
    println!("{:?}", safe_division(200.25, 0.0));
}

fn test_unwrap() {
    let pokeball = Some("lugia");
    assert_eq!(pokeball.unwrap(), "lugia");

    let another_pokeball: Option<&str> = None;
    //assert_eq!(another_pokeball.unwrap(), "lugia"); // panic

    // Custom panic message
    //another_pokeball.expect("No Pokemon in here....");
    
    println!("Testing unwrap_or()");
    let poke = another_pokeball.unwrap_or("default");
    println!("poke: {}", poke);
}

fn test_if_let() {
    // This is saying `num` can be either a u8 integer or None
    let num: Option<u8> = Some(7);
    match num {
        // comma-separated values!! I keep forgetting
        Some(7) => println!("Wahoo!"),
        _ => {},
    }

    // Short cut for above
    if let Some(7) = num {
        println!("Wahoo!");
    }
}

fn test_option() {
    // Trying to be random as possible
    let pokemon = vec![
        "raichu",
        "golem",
        "quilava",
        "muk",
        "wurmple"
    ];

    let first = pokemon.get(0);
    println!("0: {:?}", first);
    println!("99: {:?}", pokemon.get(99)); // note this does not raise a panic!
    println!("2: {:?}", pokemon.get(2));

    // Test pattern matching
    println!("Pattern matching test! いくぞ！");
    for &index in [0, 2, 3, 99].iter() {
        println!("index: {}", index);
        match pokemon.get(index) {
            Some(&"muk") => println!("Sludge!!!!"),
            Some(pokemon_name) => println!("I choose {}!", pokemon_name),
            None => println!("Empty pokéball...")
        }
    }
}

fn test_panic() {
    let panic = true;
    if panic {
        panic!("ダメダメダメダメ！進化なんてナシナシナシ！！！");
        println!("This shouldn't be printed riiiight?");
    }
}
