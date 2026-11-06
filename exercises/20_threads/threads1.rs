// Este programa cria várias threads. Cada uma executa por pelo menos 250 ms
// e retorna quanto tempo levou para terminar. O programa deve esperar todas
// as threads finalizarem e armazenar seus valores de retorno em um vetor.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} concluída");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Armazene os resultados de todas as threads no vetor `results`.
        // Use a struct `JoinHandle`, retornada por `thread::spawn`.
    }

    if results.len() != 10 {
        panic!("Ah não! Alguma thread ainda não terminou!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} levou {result} ms");
    }
}
