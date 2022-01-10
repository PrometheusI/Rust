use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    ownership_and_hashmaps();
    vectors_of_tupels_to_hashmap();

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name);
    println!("{}'s score is: {:?}", team_name, score);

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
    overwriting_values();
    insert_value_if_the_key_has_no_value();
    update_value();
}

fn vectors_of_tupels_to_hashmap(){
    let teams = 
        vec!["Blue".to_string(), "Red".to_string()];
    let initial_scores = vec![10, 50];

    let mut _scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}

fn ownership_and_hashmaps(){
    let field_name = String::from("Favorit color");
    let flield_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, flield_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

}

fn overwriting_values(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    scores.insert("Blue".to_string(), 25);
    println!("{:?}",scores);
}

fn insert_value_if_the_key_has_no_value(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    scores.entry(String::from("Red")).or_insert(60);
    scores.entry(String::from("Yellow")).or_insert(30);

    println!("{:?}",scores);
}
fn update_value(){
    let text = "hello world wonderfull world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
