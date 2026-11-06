// Aqui estão mais alguns problemas simples para mostrar como o Clippy ajuda.
// TODO: Corrija todos os lints do Clippy.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Suponha que você não conheça o valor de `my_option`.
    // Caso seja `Some`, queremos exibir o valor.
    if my_option.is_none() {
        println!("{}", my_option.unwrap());
    }

    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.resize(0, 5);
    println!("This Vec is empty, see? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Vamos trocar esses dois valores!
    value_a = value_b;
    value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}
