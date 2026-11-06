// Neste exercício, temos um `Vec<u32>` chamado `numbers`, com valores de 0 a 99,
// e queremos compartilhá-lo entre 8 threads. Cada thread somará um conjunto
// diferente de valores usando um deslocamento.
//
// A primeira thread (offset 0) soma 0, 8, 16, …
// A segunda (offset 1) soma 1, 9, 17, …
// A terceira (offset 2) soma 2, 10, 18, …
// …
// A oitava (offset 7) soma 7, 15, 23, …
//
// Cada thread precisa possuir um ponteiro com contagem de referências para o
// vetor. Como `Rc` não é seguro entre threads, usaremos `Arc`.
//
// Não se preocupe ainda com a criação e sincronização das threads; isso será
// praticado mais adiante.

// Não altere as linhas abaixo.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Defina `shared_numbers` usando `Arc`.
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Defina `child_numbers` usando `shared_numbers`.
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("A soma do deslocamento {offset} é {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
