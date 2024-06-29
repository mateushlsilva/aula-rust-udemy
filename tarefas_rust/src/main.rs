fn count(num: i32){
    for i in 1..num{
        println!("Número atual {}", i);
    } 
}

fn count_down(num:i32){
    let mut n = num;
    while n > 0 {
        println!("Número atual {}", n);
        n -= 1;
    }
}

fn main() {
    count(10);
    count_down(10);
}
