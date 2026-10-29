// =====================================================================
// ESERCIZIO 3: Interfaccia aziendale (Gestione Dipendenti)
// 
// Obiettivo: Usare una hash map e dei vettori per creare un'interfaccia 
// testuale che permetta a un utente di aggiungere nomi di dipendenti a 
// un dipartimento in un'azienda. 
// Esempio: "Add Sally to Engineering" o "Add Amir to Sales".
// Permettere poi all'utente di recuperare una lista di tutte le persone 
// in un dipartimento o di tutte le persone nell'azienda raggruppate per 
// dipartimento, ordinate alfabeticamente.
// =====================================================================

use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Inserisci la richiesta (es. 'Add Sally to Engineering'):");

        let mut query = String::new();

        io::stdin().read_line(&mut query)
            .expect("Failed to read line!");
        
        let cleared_query: Vec<&str> = query.trim().split_whitespace().collect();

        if cleared_query.len() == 4 && cleared_query[0] == "Add" && cleared_query[2] == "to"{
            let dipendenti = map.entry(cleared_query[3].to_string()).or_insert(Vec::new()); 
            dipendenti.push(cleared_query[1].to_string());
            println!("{:#?}", map);
        }
        else if cleared_query[0] == "List" && cleared_query.len() == 2{
            if cleared_query[1] != "All"{
                if let Some(dipendenti) = map.get_mut(cleared_query[1]){
                    dipendenti.sort();
                    println!("{}: {:#?}", cleared_query[1], dipendenti);
                }
            }
            else if cleared_query.len() == 2 && cleared_query[0] == "List" {
                let mut dipartimenti = map.keys().cloned().collect::<Vec<_>>();
                dipartimenti.sort();
                for dip in dipartimenti {
                    if let Some(dipendenti) = map.get_mut(&dip){
                        dipendenti.sort();
                        println!("{}: {:#?}", dip, dipendenti);
                    }
                }
            }
        }
    }
}
