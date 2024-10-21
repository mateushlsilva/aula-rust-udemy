use std::io;


fn convert_to_int(input: &str) -> Vec<f64> {
    input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok()) 
        .collect()
}

fn soma_par(numeros: Vec<f64>) {
    let mut soma = 0.0;
    for &n in &numeros {
        if n % 2.0 == 0.0 { 
            soma += n; 
        }
    }
    println!("A soma dos números pares é: {}", soma);
}

fn main() {
  
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler a linha");
    let numeros: Vec<f64> = convert_to_int(&input);
    soma_par(numeros);
}
