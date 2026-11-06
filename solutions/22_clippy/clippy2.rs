fn main() {
    let mut res = 42;
    let option = Some(12);
    // Use `if let` em vez de iterar sobre o `Option`.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
