fn esta_a_um_passo(str1: &str, str2: &str) -> bool {
    if (str1.len() as isize - str2.len() as isize).abs() > 1 {
        return false;
    }
    let mut diferenca = 0;
    let mut i = 0;
    let mut j = 0;

    let max_len = str1.len().max(str2.len()); 
    for _c in 0..max_len {
        let char1 = str1.chars().nth(i);
        let char2 = str2.chars().nth(j);
        if char1 != char2 {
            diferenca += 1; 
            if diferenca > 1 {
                return false;
            }
            if str1.len() == str2.len() {
                i += 1; 
                j += 1; 
            } else {
                if str1.len() > str2.len() {
                    i += 1; 
                } else {
                    j += 1; 
                }
                continue;
            }
        } else {
            i += 1;
            j += 1;
        }
    }
    true 
}
 
fn main() {
    // Testes de exemplo
    let str1 = "pale";
    let str2 = "ple";
    println!("Strings estão a uma edição de distância: {}", esta_a_um_passo(str1, str2));
 
    let str3 = "pales";
    let str4 = "pale";
    println!("Strings estão a uma edição de distância: {}", esta_a_um_passo(str3, str4));
 
    let str5 = "pale";
    let str6 = "bale";
    println!("Strings estão a uma edição de distância: {}", esta_a_um_passo(str5, str6));
 
    let str7 = "pale";
    let str8 = "bibo";
    println!("Strings estão a uma edição de distância: {}", esta_a_um_passo(str7, str8));
}
