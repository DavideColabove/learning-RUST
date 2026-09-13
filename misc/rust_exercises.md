# 💻 Esercizi Pratici: The Rust Book

Qui trovi i file sorgente per gli esercizi pratici. Ogni blocco di codice rappresenta un file `.rs` che puoi creare nella tua repository. Le direttive sono commentate direttamente in cima al file, così saprai esattamente cosa devi programmare!

## Settimana 1: Le Fondamenta (Cap 1-3)

**File:** `ch03_fondamenta.rs`
```rust
// ---------------------------------------------------------
// Settimana 1 - Capitolo 1, 2, 3: Fondamenta
// Obiettivo: Prendere confidenza con variabili e funzioni.
// 
// Direttive:
// 1. Chiedi all'utente di inserire una temperatura in Celsius.
//    (Usa `std::io::stdin().read_line(...)`)
// 2. Scrivi una funzione `celsius_to_fahrenheit(c: f64) -> f64` 
//    che esegua la formula (C * 9.0/5.0 + 32.0).
// 3. Stampa il risultato.
// 4. Scrivi un loop che stampi i primi 10 numeri di Fibonacci.
// ---------------------------------------------------------

use std::io;

fn main() {
    println!("Benvenuto negli esercizi della Settimana 1!");
    // Inizia qui a scrivere il tuo codice...
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    // Implementami!
    0.0
}
```

## Settimana 2: Il Cuore di Rust (Cap 4-6)

**File:** `ch04_ownership.rs`
```rust
// ---------------------------------------------------------
// Settimana 2 - Capitolo 4: Ownership
// Obiettivo: Comprendere le regole del borrow checker.
//
// Direttive:
// 1. Scrivi una funzione `prendi_e_restituisci(s: String) -> String` 
//    che prende ownership di una stringa e la ridà indietro.
// 2. Scrivi una funzione `calcola_lunghezza(s: &String) -> usize` 
//    che prende un riferimento immutabile.
// 3. Scrivi una funzione `modifica_stringa(s: &mut String)` 
//    che appende " Rust!" alla stringa passata (mutabile).
// 4. Testa tutto nel main senza causare errori di ownership!
// ---------------------------------------------------------

fn main() {
    println!("Esercizio sull'ownership!");
    // Inizializza le tue variabili e chiama le funzioni qui...
}
```

**File:** `ch05_06_strutture.rs`
```rust
// ---------------------------------------------------------
// Settimana 2 - Capitoli 5 e 6: Structs & Enums
// Obiettivo: Creare dati complessi e matchare le varianti.
//
// Direttive:
// 1. Definisci una struct `Utente` con: username, email, attivo(bool).
// 2. Crea un enum `Messaggio` con varianti: 
//    `Login(String)`, `Logout`, `AggiornaStato(bool)`.
// 3. Scrivi una funzione che accetti un `Messaggio` e usi un
//    blocco `match` per stamparne il contenuto.
// 4. Istanzia un utente, creagli dei messaggi, e processali.
// ---------------------------------------------------------

fn main() {
    // ...
}
```

## Settimana 3: Collezioni e Organizzazione (Cap 7-8)

**File:** `ch08_collezioni.rs`
```rust
// ---------------------------------------------------------
// Settimana 3 - Capitolo 8: Common Collections
// Obiettivo: Lavorare con Vec, String e HashMap.
//
// Direttive:
// 1. Crea un `Vec<i32>`, inserisci numeri sparsi. Scrivi logica 
//    per calcolare la media (mean) e la mediana.
// 2. Crea un `HashMap<String, Vec<String>>` per mappare i 
//    dipartimenti aziendali. (es. "IT" -> ["Alice", "Bob"]).
// 3. Aggiungi un paio di dipendenti e poi itera sulla mappa
//    per stampare chi lavora dove.
// ---------------------------------------------------------

fn main() {
    // ...
}
```

## Settimana 4: Robustezza e Astrazione (Cap 9-10)

**File:** `ch09_10_generici.rs`
```rust
// ---------------------------------------------------------
// Settimana 4 - Capitolo 9, 10: Error Handling & Generics
//
// Direttive:
// 1. Definisci un trait `Riassumibile` con un metodo 
//    `riassunto(&self) -> String`.
// 2. Implementalo per due struct: `Articolo` e `Tweet`.
// 3. Scrivi una funzione che tenta di leggere un file (che 
//    non esiste) e restituisca un `Result<String, std::io::Error>`.
//    Gestisci l'errore nel main con un `match`.
// ---------------------------------------------------------

fn main() {
    // ...
}
```

## Settimana 5: Test (Cap 11)

**File:** `ch11_test.rs`
```rust
// ---------------------------------------------------------
// Settimana 5 - Capitoli 11
// Obiettivo: Imparare a scrivere gli unit test in Rust.
//
// Direttive:
// 1. Scrivi una funzione pubblica `aggiungi_due(a: i32) -> i32`.
// 2. Crea un modulo interno `#[cfg(test)] mod tests { ... }`.
// 3. Scrivi un test che verifica che aggiungi_due(2) faccia 4.
// 4. Scrivi un test con #[should_panic] per una funzione che
//    chiama la macro `panic!()`.
// ---------------------------------------------------------

pub fn aggiungi_due(a: i32) -> i32 {
    a + 2
}

fn main() {
    println!("Avvia con `rustc --test ch11_test.rs` oppure metti in src/lib.rs e usa `cargo test`.");
}
```

## Settimana 6: Funzionale (Cap 13)

**File:** `ch13_iterators.rs`
```rust
// ---------------------------------------------------------
// Settimana 6 - Capitolo 13: Iterators
// Obiettivo: Abbandonare il "for" in favore degli iteratori.
//
// Direttive:
// 1. Crea un vettore di numeri `1..=10`.
// 2. Concatenando i metodi sull'iteratore:
//    - Usa `filter` per tenere i pari.
//    - Usa `map` per moltiplicare per 10.
//    - Usa `collect` per ottenere il vettore finale.
// 3. Stampa il risultato!
// ---------------------------------------------------------

fn main() {
    // ...
}
```

## Settimana 7: Concorrenza (Cap 15-16)

**File:** `ch16_threads.rs`
```rust
// ---------------------------------------------------------
// Settimana 7 - Capitolo 15 e 16: Concorrenza
// Obiettivo: Multithreading sicuro e data sharing.
//
// Direttive:
// 1. Spawna 3 thread usando `std::thread::spawn`.
// 2. Usa un canale `mpsc::channel()` per fare inviare un
//    messaggio stringa al main thread.
// 3. Nel `main`, ricevi i messaggi (usando un loop sul `Receiver`)
//    e stampali in console.
// ---------------------------------------------------------

use std::thread;
use std::sync::mpsc;

fn main() {
    // ...
}
```

## Settimana 8: Web Server (Cap 20)

**Nota:** Per questo capitolo, la direttiva è una sola: **Segui il capitolo 20 del libro e costruisci il server HTTP da zero!** 
Ti servirà creare un intero progetto Cargo (`cargo new hello_server`), dividere il ThreadPool in `src/lib.rs` e il server in `src/main.rs`. Sarà il test finale delle tue competenze su: struct, ownership, lifetime, mutex, trait e chiusure!
