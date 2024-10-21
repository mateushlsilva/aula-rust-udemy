fn eh_permutacao(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len(){
        return false;
    }
    let mut string1: Vec<char> = str1.chars().collect();
    let mut string2: Vec<char> = str2.chars().collect();
    string1.sort();
    string2.sort();
    string1 == string2
}
fn teste(str1: &str, str2: &str){
    if eh_permutacao(str1, str2) {
        println!("As strings {} e {} são permutações uma da outra.", str1, str2);
    } else {
        println!("As strings {} e {} não são permutações uma da outra.", str1, str2);
    }
}

fn main() {
    teste("abc", "bac");
    teste("listen", "silent");
    teste("abc", "abd");
    teste("hello", "billion");
}
