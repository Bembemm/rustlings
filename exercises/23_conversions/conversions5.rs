// `AsRef` e `AsMut` permitem conversões baratas entre referências.
// Leia mais em https://doc.rust-lang.org/std/convert/trait.AsRef.html e
// https://doc.rust-lang.org/std/convert/trait.AsMut.html.

// Obtenha o número de bytes — não de caracteres — do argumento.
// Em uma string, `.len()` retorna a quantidade de bytes.
// TODO: Adicione a trait `AsRef` como limite de tipo (*trait bound*) adequado.
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtenha o número de caracteres — não de bytes — do argumento.
// TODO: Adicione a trait `AsRef` como limite de tipo (*trait bound*) adequado.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Eleva um número ao quadrado usando `as_mut()`.
// TODO: Adicione o limite de trait apropriado.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implemente o corpo da função.
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
