// Continuando o exercício anterior, queremos que todas as threads terminem
// seu trabalho. Desta vez, porém, elas também precisam atualizar um valor
// compartilhado: `JobStatus.jobs_done`.

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` sozinho não basta para compartilhar um estado **mutável**.
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: Você precisa fazer algo antes de modificar um valor compartilhado.
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Aguarda a conclusão de todos os trabalhos.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Exiba o valor de `JobStatus.jobs_done`.
    println!("Trabalhos concluídos: {}", todo!());
}
