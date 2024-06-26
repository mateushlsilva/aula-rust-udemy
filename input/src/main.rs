use std::io;

fn convert_to_int(input:&str) -> i32{
    let x = input.trim().parse::<i32>().unwrap();
    return x
}

fn main() {
    let mut n1 = String::new();
    io::stdin().read_line(&mut n1).expect("Erro ao ler número"); 
    let mut n2 = String::new();
    io::stdin().read_line(&mut n2).expect("Erro ao ler número"); 

    if convert_to_int(&n1) > convert_to_int(&n2) {
        println!("O número {} é maior que {}", n1, n2)
    }
    else {
        println!("O número {} é menor ou igual que {}", n1, n2)
    }
}
