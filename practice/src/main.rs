fn cap(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

fn capitalize_words_vector(words: &[&str]) {
    //  let words = vec!["hello", "world"];

      words.iter()
    //
}

fn main(){

let printss = cap("hello");
let words = vec!["hello", "world"];

 let ww =   capitalize_words_vector(&words);
    

println!("{}",&printss);

println!("{:?}",&ww);
}
