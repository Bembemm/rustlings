// Alice é professora e precisa calcular a média das notas de
// três turmas. Em vez de calcular uma após
// a outra, ela pede ajuda aos amigos Bob e Catherine. Trabalhando
// juntos, eles terminam a tarefa muito mais rápido.
//
// Vamos simular isso usando programação assíncrona. Cada pessoa é
// representada por uma tarefa assíncrona, que pode ser executada concorrentemente.

// Tarefas assíncronas precisam ser executadas por um "runtime", que não é fornecido pela
// biblioteca padrão do Rust. Aqui usamos o popular runtime `tokio`.
// A macro `tokio::main` executa toda a função `main` dentro de um runtime.
#[tokio::main]
async fn main() {
    let mean_score_a = tokio::spawn(calculate_mean_score("scores_class_a.txt"));
    let mean_score_b = tokio::spawn(calculate_mean_score("scores_class_b.txt"));
    let mean_score_c = tokio::spawn(calculate_mean_score("scores_class_c.txt"));

    assert_eq!(mean_score_a.await.unwrap(), 84); // alice
    assert_eq!(mean_score_b.await.unwrap(), 89); // bob
    assert_eq!(mean_score_c.await.unwrap(), 76); // catherine
}

async fn calculate_mean_score(scores_file: &str) -> usize {
    // Leia o arquivo de forma assíncrona
    let file = tokio::fs::read_to_string(scores_file).await.unwrap();

    // Inicialize a soma e a quantidade de notas
    let mut sum = 0;
    let mut n = 0;
    for line in file.lines() {
        // Converta cada linha em uma nota
        let score = line.parse::<usize>().unwrap();
        sum += score;
        n += 1;
    }

    sum / n
}
