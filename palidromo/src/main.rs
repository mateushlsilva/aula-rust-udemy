use unicode_normalization::UnicodeNormalization;

fn eh_palindromo(s: &str) -> bool{
    let string: Vec<char> = s.nfd()
    .filter(|c| c.is_alphanumeric()) 
    .flat_map(|c| c.to_lowercase())
    .collect();
    let revertido = string.iter().rev().cloned().collect::<Vec<_>>();
    string == revertido
}
fn main() {
    println!("A frase {} é palidromo?: {}", "A mãe te ama", eh_palindromo("A mãe te ama"));
    println!("A frase {} é palidromo?: {}", "O lobo ama o bolo", eh_palindromo("O lobo ama o bolo"));
    println!("A frase {} é palidromo?: {}", "Hoje o sol está quente", eh_palindromo("Hoje o sol está quente"));
    println!("A frase {} é palidromo?: {}", "A maçã caiu da árvore", eh_palindromo("A maçã caiu da árvore"));
}
