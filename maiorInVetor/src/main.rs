fn maior_numero(lista: Vec<i32>) -> i32{
    let mut num = 0;
    for i in lista{
        if i > num {
            num = i;
        }
    }
    return num;
}

fn main() {
    let list = vec![30, 10, 6, 9, 40, 20, 40, 44, 5, 4, 10];
    println!("O maior número é {}",maior_numero(list));
}
