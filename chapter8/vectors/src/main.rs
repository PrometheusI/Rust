fn main() {
    #[allow(unused_variables)]
    // Explicitly declairing the type
    let v: Vec<i32> = Vec::new();
    // Letting Rust declair the type
    let v_inverred = vec![1, 2, 3, 4, 5, 7];

    let third = &v_inverred[2];
    println!("the third element is {}.", third);

    match v_inverred.get(8){
        Some(nine) => println!("the third element is {}", nine),
        None => println!("there is no ninth element"),
    }

    let mut v_mut = Vec::new();
    v_mut.push(5);
    v_mut.push(6);
    v_mut.push(7);
    v_mut.push(8);
    {
        let _v_scope = vec![1, 2, 3, 4, 5];

        //do stuff with v
    }// <- v goes out of scope and is droped here

    loop_over_vector(v_inverred, v_mut);
    vector_of_enums();
}
#[allow(dead_code)]
fn inutable_borrow_and_mutable_borrow_in_the_same_scope(){
    #[allow(unused_mut)]
    let mut v = vec![1, 2, 3, 4];

    let first = &v[2];

    //v.push(5);

    println!("the first element is {}", first);
}

fn loop_over_vector(v: Vec<i32>, mut v_mut: Vec<i32>){
    
    // Immutable vector loop
    for i in &v {
        println!("{}",i);
    }

    //mutable vector loop
    for i in &mut v_mut {
        *i += 50;
        println!("{}",i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
fn vector_of_enums(){
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(12.2),
    ];
    for i in &row {
        println!("{:?}",i);
    }
}
