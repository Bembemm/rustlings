fn call_me(num: u8) {
    for i in 0..num {
        println!("Trim! Chamada número {}", i + 1);
    }
}

fn main() {
    // `call_me` espera receber um argumento.
    call_me(5);
}
