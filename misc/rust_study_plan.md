# 🦀 Scaletta 8 Settimane: Master The Rust Book

Questa è la roadmap per completare la lettura del "Rust Book" (20 capitoli) in circa 8-10 settimane (tenendosi del margine). Una media di 2-3 capitoli a settimana è il ritmo ideale per assimilare i concetti, in particolare l'Ownership, che richiede tempo per essere metabolizzata.

## 📅 Roadmap Settimanale

### Settimana 1: Le Fondamenta (Capitoli 1, 2, 3)
- **Cosa imparerai:** Installazione, uso base di Cargo, Hello World, il Guessing Game (I/O, numeri casuali), Variabili, Tipi di base, Funzioni, Controllo del Flusso (`if`, `loop`, `while`, `for`).
- **Focus:** Abituati alla sintassi, alla tipizzazione forte e ai messaggi di errore del compilatore (che saranno i tuoi migliori amici).

### Settimana 2: Il Cuore di Rust (Capitoli 4, 5, 6)
- **Cosa imparerai:** Ownership, Borrowing, Slice (Ch 4). Structs e implementazione di Metodi (Ch 5). Enums e l'onnipotente operatore `match` (Ch 6).
- **Focus:** Il Capitolo 4 è fondamentale. Rileggilo se serve. Capire l'ownership ti risparmierà molta frustrazione in futuro. Non correre.

### Settimana 3: Organizzazione e Strutture Dati (Capitoli 7, 8)
- **Cosa imparerai:** Moduli, Crates, controllo della Visibilità (`pub`) e organizzazione del codice in file multipli (Ch 7). Common Collections: `Vec`, `String`, `HashMap` (Ch 8).
- **Focus:** Organizzare il codice e destreggiarsi con le Stringhe (che in Rust sono complesse per garantire sicurezza e supporto corretto all'UTF-8).

### Settimana 4: Robustezza e Astrazione (Capitoli 9, 10)
- **Cosa imparerai:** Gestione degli Errori con `Result`, `Option`, l'operatore `?` e `panic!` (Ch 9). Generic Types, Traits (simili alle interfacce), e i Lifetimes (Ch 10).
- **Focus:** Lifetimes. Un altro scoglio importante. Se riesci a padroneggiare Lifetimes + Ownership, sei ufficialmente un Rustecean.

### Settimana 5: Test e Progetto (Capitoli 11, 12)
- **Cosa imparerai:** Scrivere Test automatici (Ch 11). Costruire un tool CLI "Minigrep" reale che legge file e variabili d'ambiente (Ch 12).
- **Focus:** Il Capitolo 12 è il tuo primo vero progetto pratico in Rust! Metti insieme tutto quello che hai imparato finora.

### Settimana 6: Stile Funzionale e Cargo (Capitoli 13, 14)
- **Cosa imparerai:** Closures e Iterators (Ch 13). Approfondimenti su Cargo, profili di release, pubblicazione su Crates.io, Workspaces (Ch 14).
- **Focus:** Sostituire i classici cicli `for` con la potenza e la concisione (ed incredibile efficienza) degli `Iterators`.

### Settimana 7: Puntatori e Concorrenza (Capitoli 15, 16)
- **Cosa imparerai:** Smart Pointers (`Box`, `Rc`, `RefCell`) (Ch 15). Fearless Concurrency: Thread, Message Passing via Channels (`mpsc`), Stato condiviso con `Mutex` e `Arc` (Ch 16).
- **Focus:** Concorrenza senza *data race*. Rust brilla qui, imparerai perché viene scelto per i sistemi backend ad alte prestazioni.

### Settimana 8: Avanzato e Progetto Finale (Capitoli 17, 18, 19, 20)
- **Cosa imparerai:** Object Oriented Rust (Ch 17). Pattern matching avanzato (Ch 18). Unsafe Rust, Advanced Macros (Ch 19). E infine: costruire un Server Web Multithread da zero! (Ch 20).
- **Focus:** Goditi la costruzione del server web. È il culmine del viaggio e una grandissima soddisfazione.

---

## 🛠️ Come Gestire la Repository GitHub "Showcase"

Ho appena generato per te una cartella in locale `c:\Users\Davide\Downloads\rust\Rust-Showcase` con uno scaffold completo di file `.rs` e commenti che ti guideranno negli esercizi.
Ecco come renderla una repository di successo per il tuo portfolio!

### 1. Inizializzare la repo
1. Vai su GitHub e crea una nuova repository pubblica (es. `rust-learning-journey`).
2. Apri il terminale nella cartella `Rust-Showcase`.
3. Esegui questi comandi:
   ```bash
   git add .
   git commit -m "Inizializzazione repo e piano di studio"
   git branch -M main
   git remote add origin https://github.com/TUO-USERNAME/rust-learning-journey.git
   git push -u origin main
   ```

### 2. Best Practices per lo Showcase
- **Commit atomici:** Fai un commit per ogni capitolo/esercizio completato (es. `git commit -m "feat: solved chapter 4 ownership exercises"`). Questo mostra che lavori in modo incrementale e pulito.
- **Aggiorna il README.md:** Tieni traccia dei tuoi progressi. I recruiter amano leggere il *tuo* percorso: scrivi cosa hai trovato difficile e come lo hai superato.
- **Passa a Cargo:** I file che ti ho generato sono semplici `.rs` che puoi compilare con `rustc`. Man mano che scrivi progetti più grossi, cancellerai i `.rs` singoli e li rimpiazzerai con progetti Cargo (`cargo new nome_esercizio`). Una repo può contenere tranquillamente molteplici cartelle Cargo al suo interno (o usare i *Cargo Workspaces* che imparerai al capitolo 14!).
