fn is_anagram(s: &str, t: &str) -> bool {
    let mut s: Vec<char> = s.chars().collect();
    let mut t: Vec<char>  = t.chars().collect();
    s.sort();
    t.sort();
    s == t
}

fn main() {
    println!("A palavra {} é anagrama de {}?: {}", "amor","roma", is_anagram("amor","roma"));
    println!("A palavra {} é anagrama de {}?: {}", "carro","rocar", is_anagram("carro","rocar"));
    println!("A palavra {} é anagrama de {}?: {}", "sol","lua", is_anagram("sol","lua"));
    println!("A palavra {} é anagrama de {}?: {}", "peixe","feliz", is_anagram("peixe","feliz"));
}
