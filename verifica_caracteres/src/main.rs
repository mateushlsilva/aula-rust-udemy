use std::io;
use std::collections::HashSet;

fn tem_caracteres_unicos(input: &str) -> bool{
    let string: Vec<char> = input.chars().collect();
    let mut existe: HashSet<char> = HashSet::new();
    for s in &string{
        if existe.contains(&s){
            return false
        }
        else {
            existe.insert(*s);
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler");
    if tem_caracteres_unicos(&input){
        println!("A string possui todos os caracteres únicos.");     
    } 
    else {         
        println!("A string não possui todos os caracteres únicos.");     
    }
}
