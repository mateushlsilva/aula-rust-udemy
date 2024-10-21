fn compress_string(input: &str) -> String{
    let mut cont: i32 = 1;
    let mut s = input.chars().nth(0).unwrap();
    let mut string = String::new();
    for c in input.chars().skip(1){
        if c == s {
            cont += 1;
        }
        else {
            string.push(s); 
            string.push_str(&cont.to_string());
            s = c; 
            cont = 1; 
        }
    }
    string.push(s);
    string.push_str(&cont.to_string());
    if string.len() >= input.len(){
        return input.to_string();
    }
    string
}

fn main() {
    let original_str = "aabcccccaaa";
    let compressed_str = compress_string(&original_str);
    println!("Original: {}", original_str);
    println!("Compressed: {}", compressed_str);
 
    let other_str = "abcdefgh";
    let compressed_other = compress_string(&other_str);
    println!("Original: {}", other_str);
    println!("Compressed: {}", compressed_other);
}
