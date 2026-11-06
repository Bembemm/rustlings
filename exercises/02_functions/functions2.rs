// TODO: Adicione o tipo do argumento `num` depois dos dois-pontos `:`.
fn call_me(num:) {
    for i in 0..num {
        println!("Trim! Chamada número {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
