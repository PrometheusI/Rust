
fn main() {
    let mut _s = String::new();

    let data = "new Data";
    
    let _s = data.to_string();

    // The methode also works on a literal directly.
    // Equvalent to above
    let _s = "new Data".to_string();

    let _s = String::from("new Data");
    
    /* All Strings ar UFT-8 encoded
    / let hello = String::from("السلام عليكم");
    / let hello = String::from("Dobrý den");
    / let hello = String::from("Hello");
    / let hello = String::from("שָׁלוֹם");
    / let hello = String::from("नमस्ते");
    / let hello = String::from("こんにちは");
    / let hello = String::from("안녕하세요");
    / let hello = String::from("你好");
    / let hello = String::from("Olá");
    / let hello = String::from("Здравствуйте");
    / let hello = String::from("Hola");
    */

    let mut s = String::from("Foo");
    let s2 = "bar";
    // Doesn't take ownership of s2.
    s.push_str(s2);
    println!("{}",s2);

    let mut s3 = "Lo".to_string();
    s3.push('l');
    println!("{}",s3);

    s3 = add_strings(s, &s3);
    println!("{}",s3);

    format_string();
    
    iterate_over_string(&s3);
   
}

fn add_strings(s1: String, s2: &String) -> String{

    let s3 = s1 + s2;//s1 is been used and can no longer be used
    return s3;
}

fn format_string(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}",s1,s2,s3);
    // is equvalent to but doesn't take ownership:
    // let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is: {}",s);
    println!("{}, {}, {}", s1, s2, s3);
}

fn iterate_over_string(s: &String){
    for c in s.chars(){
        println!("{}",c)
    }
}