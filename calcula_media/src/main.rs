use std::io;


fn convert_to_int(input: &str) -> Vec<f64> {
    input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok()) 
        .collect()
}

fn calcula_media(notas: Vec<f64>) -> f64{
    let mut soma:f64 = 0.0;
    for &nota in &notas{
        soma += nota;
    }
    return soma / notas.len() as f64;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler a linha");
    let numeros: Vec<f64> = convert_to_int(&input);
    println!("A média é {}", calcula_media(numeros));
}
