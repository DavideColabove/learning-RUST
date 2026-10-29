use std::collections::HashMap;

fn main() {
    let mut numbers = vec![42, 1, 36, 99, 42, 8, 23, 42, 1, 8];

    // --- 1. Calcolo della Media (Mean) ---
    // TODO: Implementa qui la logica.
    let sum : i32 = numbers.iter().sum();
    let mean = (sum as f64) / (numbers.len() as f64);
    println!("Media: {}", mean);


    // --- 2. Calcolo della Mediana (Median) ---
    // TODO: Implementa qui la logica.
    numbers.sort();
    let median : &i32 = &numbers[numbers.len()/ 2 ];
    println!("Mediana: {}", median);


    // --- 3. Calcolo della Moda (Mode) ---
    // TODO: Implementa qui la logica.
    let mut map = HashMap::new();
    for &mut num in &mut numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max_count : i32 = 0;
    let mut mode : i32 = 0; 
    for (key, value) in &map{
        if *value > max_count{
            max_count = *value;
            mode = *key;
        }
    }
    println!("Moda {}: {}", mode, max_count);


    
}