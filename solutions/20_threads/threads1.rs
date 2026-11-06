// Este programa cria várias threads, cada uma executando por pelo menos 250 ms, e
// cada thread retorna quanto tempo levou para terminar. O programa deve
// aguardar todas as threads criadas terminarem e coletar seus
// valores de retorno em um vetor.

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
        // Colete os resultados de todas as threads no vetor `results`.
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Ah não! Alguma thread ainda não terminou!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} levou {result} ms");
    }
}
