use std::mem;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Chamar `unwrap` em um `Option` depois de verificar que ele é `None` causará `panic`.
    // Use `if let` em vez disso.
    if let Some(value) = my_option {
        println!("{value}");
    }

    // Faltava uma vírgula.
    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("Meu array! Aqui está: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    // `resize` modifica o vetor em vez de retornar um novo.
    // `resize(0, …)` esvazia um vetor, então é melhor usar `clear`.
    my_vec.clear();
    println!("Este Vec está vazio, viu? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Use `mem::swap` para trocar corretamente dois valores.
    mem::swap(&mut value_a, &mut value_b);
    println!("valor a: {value_a}; valor b: {value_b}");
}
