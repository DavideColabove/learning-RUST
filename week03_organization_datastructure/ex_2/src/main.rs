fn main() {
    let text = String::from("first apple");
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for word in text.split_whitespace() {
        // TODO: Implementa la logica del Pig Latin qui
        
        if let Some(first_char) = word.chars().next() {
            if vowels.contains(&first_char){
                let pig_latin = format!("{}-hay", word);
                println!("La parola '{}' inzia per vocale", pig_latin);
            }
            else{
                let rest_of_word = &word[first_char.len_utf8()..];

                let pig_latin = format!("{}-{}ay", rest_of_word, first_char);
                println!("La parola '{}' inzia per consonante", pig_latin);
            }
        }
    }
}