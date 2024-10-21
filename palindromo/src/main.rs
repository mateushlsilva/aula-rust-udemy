fn palindromo(input: &i32) -> bool{
    let string:String = input.to_string().chars().rev().collect(); 
    string == input.to_string()
}

fn main() {
    let n1 = 10;
    let n2 = 121;
    let n3 = -121;
    let n4 = 585;
    println!("O número {} é um palindromo? {}", n1, palindromo(&n1));
    println!("O número {} é um palindromo? {}", n2, palindromo(&n2));
    println!("O número {} é um palindromo? {}", n3, palindromo(&n3));
    println!("O número {} é um palindromo? {}", n4, palindromo(&n4));
}
