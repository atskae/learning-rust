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
}
