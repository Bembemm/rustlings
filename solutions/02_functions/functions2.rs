// O tipo dos argumentos de uma função precisa ser anotado.
// Foi adicionada a anotação de tipo `u64`.
fn call_me(num: u64) {
    for i in 0..num {
        println!("Trim! Chamada número {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
