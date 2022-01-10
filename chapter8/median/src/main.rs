use std::collections::HashMap;

use rand::Rng;

fn main() {
    // generiert einen Vector mit intigern(i32)
    let mut vec = Vec::new();
    for _a in 0..33000000{
        vec.push(rand::thread_rng().gen_range(0..101));
    }
    vec.sort();
    println!("{:?}",vec);
    println!("The mean is: {}", mean(&vec));
    println!("The median is: {:?}",median(&vec));
    println!("The mode is: {:?}", mode(&vec));
    
}

//ermittelt den Durchschnittswert
fn mean(vec: &Vec<i32>) -> f64{
    let mut sum: i32 = 0;
    for i in vec{
        sum += i; 
    }

    (sum as f64) / (vec.len() as f64)
}

// ermittelt den mitleren Wert(Bei einer geraden anzahl gib es zwei Werte zurück)
fn median(vec: &Vec<i32>) -> Vec<i32>{
    if vec.len() % 2 == 0 {
        return vec![vec[vec.len()/2],
            vec[vec.len()/2+1]];
    }
    vec![vec[vec.len()/2+1]]
}

//ermittelt die häufigste Zahl und deren anzahl
fn mode(vec: &Vec<i32>) -> (&i32, i32){
    let mut numbers = HashMap::new();
    for number in vec{
        let entry_number = numbers.entry(number).or_insert(0);
        *entry_number += 1;
    }

    let mut mode_number: (&i32,i32) = (&0,0);
    for number in numbers {
        if number.1 > mode_number.1 {
            mode_number = number;
        }
    }
    mode_number
}
