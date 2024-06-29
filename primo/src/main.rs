fn eh_primo(num: i32) -> bool{
    let limite = (num as f32).sqrt().ceil() as i32;
    if num <= 1{
        return false;
    }
    for n in 2..limite {
        if num % n == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let numero = 15;
    println!("O número {}, é primo: {}",numero,eh_primo(numero));
}
