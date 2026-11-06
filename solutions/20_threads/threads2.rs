// Continuando o exercício anterior, queremos que todas as threads concluam
// seu trabalho. Desta vez, porém, as threads também precisam atualizar um
// valor compartilhado: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // `Arc` sozinho não basta para compartilhar um estado **mutável**.
    // Precisamos envolver o valor em um `Mutex`.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
    //                    ^^^^^^^^^^^                          ^

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Adquira o lock antes de atualizar um valor compartilhado.
            status_shared.lock().unwrap().jobs_done += 1;
            //           ^^^^^^^^^^^^^^^^
        });
        handles.push(handle);
    }

    // Aguardando todos os trabalhos terminarem.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Trabalhos concluídos: {}", status.lock().unwrap().jobs_done);
    //                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
