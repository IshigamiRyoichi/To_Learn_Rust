use rand::distributions::Standard;
pub fn random<T>() -> T
where Standard: Distribution<T> {
    thread_rng().gen()
}

use rand::prelude::*;
fn test1(){
    let x: i32 = random();
    println!("x is {}", x);
    let mut rng = thread_rng();
    let y: i32 = rng.gen_range(0,10);
    println!("y is {}",y);
}

use my_random::random::Dice;
fn main(){
    let dice = Dice{max: 6};
    let x = dice.get();
    println!("x is {}",x);

    let mut data = vec![0,0,6];
    dice.fill(&mut data);
    for i in data{
        println!("i is {}",i);
    }
}

// fn main() {
//     test1();
//     println!("Hello World!");
// }
