pub fn test_my_arrays() {
    println!("Hello from the array module!");

    // why is it called grapefruit days???
    // I do like how he started on a Monday 😎
    let grapefruit_days = [
        "マンデイ",
        "チュズデイ",
        "ウェンズデイ",
        "サーズデイ",
        "フライデイ",
        "サタデイ",
        "サンデイ" // pika pika ... t-shirt, something
    ];

    // an array of ten 0.0s
    let list_of_zeros = [0.0; 10];

    println!("The first day: {}", grapefruit_days[0]);

    test_vectors();
}

fn test_vectors() {
    let three_nums = vec![151, 493];
    println!("Initial vector: {:?}", three_nums);

    // vectors are still immutable unless stated otherwise
    let mut fruits = Vec::new();
    fruits.push("Banana");
    fruits.push("Tomato");
    fruits.push("Radish");
    println!("Fruits: {:?}", fruits);
    fruits.pop(); // Remove last item
    println!("Fruits after: {:?}", fruits);
    
    // Arrays would emit compiler warning
    // But vectors crash at run time
    //fruits[100] = "Apple";
}

pub fn test_conditionals() {
    let is_japanese = true;   
    let greeting = if is_japanese {
        "こんにちは！"
    } else {
        "Howdy"
    };
    println!("{}", greeting);
}