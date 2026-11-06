// Alice é professora e precisa calcular a média das notas de três turmas.
// Em vez de calcular uma turma por vez, ela pede ajuda a Bob e Catherine.
// Trabalhando ao mesmo tempo, eles terminam a tarefa mais rapidamente.
//
// Vamos simular isso com programação assíncrona. Cada pessoa será representada
// por uma tarefa assíncrona que pode ser executada concorrentemente.

// Tarefas assíncronas precisam de um "runtime", que não faz parte da biblioteca
// padrão do Rust. Aqui usamos o popular runtime `tokio`.
// A macro `tokio::main` executa toda a função `main` dentro desse runtime.
#[tokio::main]
async fn main() {
    let mean_score_a = tokio::spawn(calculate_mean_score("scores_class_a.txt"));
    let mean_score_b = tokio::spawn(calculate_mean_score("scores_class_b.txt"));
    let mean_score_c = tokio::spawn(calculate_mean_score("scores_class_c.txt"));

    // TODO: Aguarde (`await`) as tarefas criadas para verificar seus resultados.
    assert_eq!(mean_score_a, 84); // alice
    assert_eq!(mean_score_b, 89); // bob
    assert_eq!(mean_score_c, 76); // catherine
}

// TODO: Corrija os erros de compilação tornando a função executada uma função `async`.
fn calculate_mean_score(scores_file: &str) -> usize {
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
