fn reverse_string(input: &str) -> String {
    let reverse = input.chars().rev().collect();
    reverse
}


fn main() {
    // Teste sua função com diferentes strings de entrada
    let input_string1 = String::from("hello");
    let reversed_string1 = reverse_string(&input_string1);
    println!("Original: {}", input_string1);
    println!("Reversed: {}", reversed_string1);
    let input_string2 = String::from("rust");
    let reversed_string2 = reverse_string(&input_string2);
    println!("Original: {}", input_string2);
    println!("Reversed: {}", reversed_string2);
}