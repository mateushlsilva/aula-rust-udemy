// Exercício 1: Soma de Dois Números
/// Recebe dois números inteiros e retorna a soma deles.
fn soma(a: i32, b: i32) -> i32 {
    a + b
}

// Exercício 2: Verificar Número Par
/// Retorna `true` se o número for par e `false` caso contrário.
fn eh_par(n: i32) -> bool {
    n % 2 == 0
}

// Exercício 3: Encontrar o Maior de Três Números
/// Recebe três números inteiros e retorna o maior deles.
fn maior_de_tres(a: i32, b: i32, c: i32) -> i32 {
    let mut  lista = [a,b,c];
    lista.sort();
    lista[2]
}

// Exercício 4: Inverter uma String
/// Recebe uma string e retorna a mesma string invertida.
fn inverter_string(s: &str) -> String {
    let mut string = s.chars().rev().collect();
    string
}

// Exercício 5: Contar Vogais
/// Recebe uma string e retorna o número de vogais presentes.
fn contar_vogais(s: &str) -> usize {
    let vogais: [&str; 5] = ["a","e","i","o","u"];
    let mut string: Vec<char> = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    let mut c: usize = 0;
    for i in string{
        let letra: &str = &i.to_string();
        if vogais.contains(&letra){
            c += 1;
        }   
    }
    c
}

// Exercício 6: Verificar Número Primo
/// Retorna `true` se o número for primo e `false` caso contrário.
fn eh_primo(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}


// Exercício 7: Calcular Fatorial
/// Calcula o fatorial de um número inteiro não negativo.
fn fatorial(n: u32) -> u32 {
    if n <= 1{
        return 1;
    }
    n * fatorial(n-1)
}

// Exercício 8: Remover Duplicatas de um Vetor
/// Recebe um vetor de inteiros e retorna um novo vetor sem elementos duplicados.
fn remover_duplicatas(mut vetor: Vec<i32>) -> Vec<i32> {
    vetor.sort();
    vetor.dedup();
    vetor
}

// Exercício 9: Somar Elementos de uma Lista
/// Recebe uma lista de inteiros e retorna a soma de todos os elementos.
fn soma_lista(lista: &[i32]) -> i32 {
    lista.iter().sum()
}

// Exercício 10: Converter String para Título (Capitalizar)
/// Recebe uma string e retorna a string com a primeira letra de cada palavra em maiúscula.
fn para_titulo(s: &str) -> String {
    let mut cap: Vec<&str> = s.split_whitespace().collect();
    let mut frase = String::new();
    for palavra in cap{
        if let Some(primeira) = palavra.chars().next() {
            frase.push_str(&primeira.to_uppercase().to_string());
            for c in palavra.chars().skip(1) { // Pula o primeiro caractere já adicionado
                frase.push(c);
            }
            frase.push_str(" ");
            
        }
    }
    if frase.chars().last().map_or(false, |c| c.is_whitespace()){
        frase.pop();
    }
    frase
}

// Exercício 11: Calcular Média
/// Recebe uma lista de números inteiros e retorna a média deles como um valor de ponto flutuante.
fn calcular_media(lista: &[i32]) -> f64 {
    if lista.is_empty(){
        return 0.0;
    }
    let contidade_lista: i32 = lista.len().try_into().unwrap();
    let soma: i32 = lista.iter().sum();
    soma as f64 / contidade_lista as f64
}

// Exercício 13: Encontrar o Máximo de um Vetor
/// Recebe um vetor de inteiros e retorna o maior número presente.
fn encontrar_maximo(vetor: &[i32]) -> i32 {
    if vetor.is_empty(){
        return 0;
    }
    let max: i32 = vetor.iter().copied().max().unwrap();
    max
}

// Exercício 14: Concatenar Strings com Espaço
/// Recebe um vetor de strings e retorna uma única string onde cada palavra é separada por um espaço.
fn concatenar_com_espaco(vetor: &[&str]) -> String {
   let mut frase: String = String::new();
   for (i, s) in vetor.iter().enumerate() {
    frase.push_str(s);
        if i < vetor.len() - 1 { // Verifica se não é o último elemento
            frase.push_str(" ");
        }
    }
    frase
}

// Exercício 15: Verificar Subsequência
/// Recebe duas strings e retorna `true` se a primeira for uma subsequência da segunda.
fn verificar_subsequencia(sub: &str, seq: &str) -> bool {
    let mut sub_iter = sub.chars(); // Iterador para a string subsequência
    let mut sub_char = sub_iter.next(); // Obter o primeiro caractere da subsequência

    // Itera sobre cada caractere da sequência
    for c in seq.chars() {
        // Se o caractere da sequência corresponder ao caractere atual da subsequência
        if let Some(ch) = sub_char {
            if c == ch {
                sub_char = sub_iter.next(); // Avança no iterador da subsequência
            }
        }

        // Se já processamos todos os caracteres da subsequência, retornamos true
        if sub_char.is_none() {
            return true;
        }
    }

    // Se ainda há caracteres restantes na subsequência, não é uma subsequência
    false
}

// Exercício 16: Contar Consoantes
/// Recebe uma string e retorna o número de consoantes presentes.
fn contar_consoantes(s: &str) -> usize {
    let consoantes: [char; 19]= ['b', 'c', 'd', 'f', 'g', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];
    let cont: usize = s.to_ascii_lowercase().chars().filter(|c: &char| consoantes.contains(c)).count();
    cont
}

// Exercício 17: Elevar ao Quadrado
/// Recebe um número inteiro e retorna seu quadrado.
fn elevar_ao_quadrado(n: i32) -> i32 {
    n.pow(2)
}


// Exercício 19: Remover Elementos Menores que X
/// Recebe um vetor de inteiros e um valor `x`, e retorna um novo vetor contendo apenas os elementos maiores ou iguais a `x`.
fn remover_menores_que(vetor: Vec<i32>, x: i32) -> Vec<i32> {
    let mut vetor_maiores: Vec<i32> = vetor.into_iter().filter(|y| *y >= x).collect();
    vetor_maiores
}

// Exercício 20: Contar Ocorrências de Palavra
/// Recebe uma string e uma palavra, e retorna o número de vezes que a palavra aparece na string.
fn contar_ocorrencias(texto: &str, palavra: &str) -> usize {
    let mut cont: usize = 0;
    for s in texto.split_ascii_whitespace(){
        if s.to_ascii_lowercase().find(&palavra.to_ascii_lowercase()).is_some(){
            cont += 1;
        }
    }
    cont
}


// Testes para todas as funções
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soma() {
        assert_eq!(soma(2, 3), 5);
        assert_eq!(soma(-2, 3), 1);
        assert_eq!(soma(0, 0), 0);
    }

    #[test]
    fn test_eh_par() {
        assert!(eh_par(4));
        assert!(!eh_par(5));
        assert!(eh_par(0));
    }

    #[test]
    fn test_maior_de_tres() {
        assert_eq!(maior_de_tres(1, 2, 3), 3);
        assert_eq!(maior_de_tres(10, 5, 8), 10);
        assert_eq!(maior_de_tres(-1, -2, -3), -1);
    }

    #[test]
    fn test_inverter_string() {
        assert_eq!(inverter_string("Rust"), "tsuR");
        assert_eq!(inverter_string("palavra"), "arvalap");
        assert_eq!(inverter_string(""), "");
    }

    #[test]
    fn test_contar_vogais() {
        assert_eq!(contar_vogais("Rust"), 1);
        assert_eq!(contar_vogais("palavra"), 3);
        assert_eq!(contar_vogais(""), 0);
        assert_eq!(contar_vogais("AEIOUaeiou"), 10);
    }

    #[test]
    fn test_eh_primo() {
        assert!(eh_primo(2));
        assert!(eh_primo(3));
        assert!(!eh_primo(4));
        assert!(eh_primo(5));
        assert!(!eh_primo(1)); // 1 não é considerado primo
    }

    #[test]
    fn test_fatorial() {
        assert_eq!(fatorial(0), 1);
        assert_eq!(fatorial(1), 1);
        assert_eq!(fatorial(5), 120);
    }

    #[test]
    fn test_remover_duplicatas() {
        assert_eq!(remover_duplicatas(vec![1, 2, 2, 3, 4, 4, 5]), vec![1, 2, 3, 4, 5]);
        assert_eq!(remover_duplicatas(vec![1, 1, 1, 1]), vec![1]);
    }

    #[test]
    fn test_soma_lista() {
        assert_eq!(soma_lista(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(soma_lista(&[-1, -2, -3, -4, -5]), -15);
        assert_eq!(soma_lista(&[]), 0);
    }

    #[test]
    fn test_para_titulo() {
        assert_eq!(para_titulo("rust é divertido"), "Rust É Divertido");
        assert_eq!(para_titulo("exercício de programação"), "Exercício De Programação");
    }

    #[test]
    fn test_calcular_media() {
        assert_eq!(calcular_media(&[1, 2, 3, 4, 5]), 3.0);
        assert_eq!(calcular_media(&[10, 20, 30]), 20.0);
        assert_eq!(calcular_media(&[-1, -1, -1, -1]), -1.0);
        assert_eq!(calcular_media(&[]), 0.0); // Média de uma lista vazia
    }

    #[test]
    fn test_encontrar_maximo() {
        assert_eq!(encontrar_maximo(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(encontrar_maximo(&[-1, -2, -3, -4, -5]), -1);
        assert_eq!(encontrar_maximo(&[10]), 10);
    }

    #[test]
    fn test_concatenar_com_espaco() {
        assert_eq!(concatenar_com_espaco(&["Rust", "é", "legal"]), "Rust é legal");
        assert_eq!(concatenar_com_espaco(&["concatenando"]), "concatenando");
        assert_eq!(concatenar_com_espaco(&[]), "");
    }

    #[test]
    fn test_verificar_subsequencia() {
        assert!(verificar_subsequencia("abc", "aebdc"));
        assert!(verificar_subsequencia("ace", "abcde"));
        assert!(!verificar_subsequencia("aec", "abcde"));
    }

    #[test]
    fn test_contar_consoantes() {
        assert_eq!(contar_consoantes("Rust"), 3);
        assert_eq!(contar_consoantes("palavra"), 4);
        assert_eq!(contar_consoantes("aeiou"), 0);
        assert_eq!(contar_consoantes(""), 0);
    }

    #[test]
    fn test_elevar_ao_quadrado() {
        assert_eq!(elevar_ao_quadrado(2), 4);
        assert_eq!(elevar_ao_quadrado(-3), 9);
        assert_eq!(elevar_ao_quadrado(0), 0);
    }

    #[test]
    fn test_remover_menores_que() {
        assert_eq!(remover_menores_que(vec![1, 2, 3, 4, 5], 3), vec![3, 4, 5]);
        assert_eq!(remover_menores_que(vec![10, 20, 30], 25), vec![30]);
        assert_eq!(remover_menores_que(vec![1, 2, 3], 10), vec![]);
    }

    #[test]
    fn test_contar_ocorrencias() {
        assert_eq!(contar_ocorrencias("Rust é divertido", "é"), 1);
        assert_eq!(contar_ocorrencias("Rust é legal, Rust é rápido", "Rust"), 2);
        assert_eq!(contar_ocorrencias("Rust é legal", "C++"), 0);
    }
}
fn main(){

}