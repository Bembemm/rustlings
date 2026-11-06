// Neste exercício, recebemos um `Vec<u32>` chamado `numbers`, com valores
// de 0 a 99. Queremos usar esses números em 8
// threads diferentes ao mesmo tempo. Cada thread calculará a soma de
// cada oitavo valor, usando um deslocamento.
//
// A primeira thread (offset 0) soma 0, 8, 16, …
// A segunda thread (offset 1) soma 1, 9, 17, …
// A terceira thread (offset 2) soma 2, 10, 18, …
// …
// A oitava thread (offset 7) soma 7, 15, 23, …
//
// Cada thread deve possuir um ponteiro com contagem de referências para o vetor de
// números. Como `Rc` não é seguro entre threads, precisamos usar `Arc`.
//
// Não se preocupe ainda com a forma como as threads são criadas e aguardadas. Vamos praticar
// isso mais adiante nos exercícios sobre threads.

// Não altere as linhas abaixo.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    let shared_numbers = Arc::new(numbers);
    //                   ^^^^^^^^^^^^^^^^^

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        //                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^

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
