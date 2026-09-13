# 🧠 Quiz Ripasso - The Rust Book

Usa questi quiz per testare la tua conoscenza alla fine di ogni settimana di studio. 
**Regola d'oro:** le risposte non sono scritte qui di proposito. Se non sei sicuro della risposta, esplora il codice o riapri il libro. Cercare la soluzione fisserà il concetto nella tua memoria!

## Settimana 1 (Capitoli 1, 2, 3)
1. Qual è il comando di Cargo per verificare la corretta compilazione del progetto senza perdere tempo a produrre l'eseguibile finale?
2. In Rust, le variabili sono mutabili o immutabili di default? Quale keyword si usa per cambiare questo comportamento?
3. Qual è la differenza sostanziale tra un'istruzione (*statement*) e un'espressione (*expression*)? 
4. Perché `let x = (let y = 6);` non compila?

## Settimana 2 (Capitoli 4, 5, 6)
1. Cosa succede al valore associato a `s1` (se `s1` è di tipo `String`) subito dopo aver eseguito `let s2 = s1;`?
2. In base alle regole del *Borrowing*, puoi avere contemporaneamente un riferimento mutabile (`&mut T`) e uno immutabile (`&T`) allo stesso dato nello stesso scope?
3. Nelle `struct`, a cosa serve la macro `#[derive(Debug)]` posizionata sopra la definizione?
4. Qual è il vantaggio principale dell'enum `Option<T>` rispetto al concetto di `null` tipico di Java o C#?

## Settimana 3 (Capitoli 7, 8)
1. Se un modulo `mod a` contiene un modulo figlio `mod b`, il codice in `mod b` ha accesso alle funzioni private definite in `mod a`? 
2. A cosa serve la keyword `pub` e a cosa si applica?
3. Se accedi a un indice di un vettore oltre il suo limite usando `v[100]`, cosa succede a runtime? E cosa succede invece se usi `v.get(100)`?
4. Poiché le `String` in Rust sono sempre codificate validamente in UTF-8, è possibile usare `s[0]` per ottenere in modo sicuro il primo carattere di una stringa? Perché no?

## Settimana 4 (Capitoli 9, 10)
1. Qual è la differenza concettuale tra causare un `panic!` e restituire un `Result<T, E>` da una funzione?
2. A cosa serve l'operatore magico `?` posizionato alla fine di una chiamata a funzione (es. `let file = File::open("hello.txt")?;`)?
3. Cosa definisce un `Trait` in Rust? Quale è la sua controparte in linguaggi come Java o C#?
4. Cosa rappresentano i Lifetimes (es. `<'a>`)? A chi servono e perché li devi esplicitare in alcuni contesti?

## Settimana 5 (Capitoli 11, 12)
1. Quale attributo si antepone a una funzione per indicare al framework di test che quella funzione è, appunto, un test?
2. Come si dice a Cargo che un determinato test *deve panicar* per avere successo (ad esempio, per testare che una validazione fallisca correttamente)?
3. Nel mondo Rust, dove si posizionano tipicamente i test unitari e dove quelli di integrazione?
4. Nel progetto Minigrep (Ch 12), qual è il razionale dietro il separare la logica in `src/lib.rs` lasciando `src/main.rs` il più vuoto possibile?

## Settimana 6 (Capitoli 13, 14)
1. Le *closure* (es. `|x| x + 1`) possono catturare variabili dall'ambiente circostante. Quali sono i tre trait nascosti che governano il modo in cui catturano le variabili?
2. Gli iteratori in Rust sono valutati in modo *lazy* (pigro). Cosa significa in pratica quando scrivi l'istruzione `v.iter().map(|x| x + 1);` ma poi non salvi/consumi il risultato?
3. A cosa serve il file `Cargo.lock`? 
4. Cosa sono i "Cargo Workspaces" e in quale scenario un programmatore dovrebbe iniziare a usarli?

## Settimana 7 (Capitoli 15, 16)
1. A cosa serve un `Box<T>` e quando potresti essere obbligato ad usarlo per definire una `struct`?
2. Qual è la differenza in memoria e ownership tra `Rc<T>` (Reference Counted) e una normale reference `&T`?
3. Quale funzione della standard library si usa per fare lo spawn di un thread hardware supportato dall'OS?
4. Se vuoi condividere lo stato tra più thread, perché usare `Rc<T>` genera errore di compilazione costringendoti a usare `Arc<T>`? A cosa serve poi il `Mutex` associato?

## Settimana 8 (Capitoli 17, 18, 19, 20)
1. Il Rust è un linguaggio *Object Oriented*? Offre l'ereditarietà di stato?
2. Oltre al blocco `match`, fai l'esempio di un'altra keyword/costrutto dove puoi applicare i *pattern matching*.
3. Quali sono i poteri che il compilatore disattiva (o ti concede) quando apri un blocco `unsafe { ... }`?
4. Nel Server Web finale, perché implementare un *Thread Pool* ti protegge contro attacchi di tipo Denial of Service (DoS) rispetto al semplice spawnare un thread per ogni richiesta web ricevuta?
