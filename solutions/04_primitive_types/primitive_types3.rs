fn main() {
    // Um array com 100 elementos, todos com o valor 42.
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Uau, que array grande!");
    } else {
        println!("Ah, arrays desse tamanho eu como no café da manhã.");
        panic!("O array ainda é pequeno; adicione mais elementos");
    }
}
