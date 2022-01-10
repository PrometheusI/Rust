use std::cmp::PartialOrd;
use std::fmt::Display;
#[derive(Debug)]
struct Point<T, U>{
x: T,
y: U,
}
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f64, f64>{
    fn distance_from_origin(&self) -> f64{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U>{
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {

    let point_a = Point{x: 12, y: 3.3};
    let point_b = Point{x: 1.0 , y: 3.6};
    

    println!("{}", point_a.x);
    println!("{}", point_b.distance_from_origin());

    let point_c = point_a.mixup(point_b);

    println!("{:?}", point_c);

    let vec = vec![12,23,43,500,100,34];

    let result = largest(&vec);
    println!("The largest number is: {:?}",result);

    let vec = vec!['a', 'e', 'i', 'o'];

    let result = largest(&vec);
    println!("The largest char is: {:?}",result);

}

fn largest_i32 (list: &[i32]) -> &i32{
    let mut largest = &list[0];

    for number in list{
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char (list: &[char]) -> &char{
    let mut largest = &list[0];

    for number in list{
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest<T: PartialOrd> (list: &[T]) -> &T{
    let mut largest = &list[0];

    for item in list{
        if item > largest {
            largest = item;
        }
    }
    &largest
}

struct Pair<T>{
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self{
        Self {a, b }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.a >= self.b {
            println!("The largest member is a:{}",self.a);
        }
        else{
            println!("The largest member is b: {}", self.b);
        }
    }
}
