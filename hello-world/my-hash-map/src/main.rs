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
}
