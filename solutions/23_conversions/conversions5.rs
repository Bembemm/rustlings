// `AsRef` e `AsMut` permitem conversões baratas entre referências. Leia mais
// em https://doc.rust-lang.org/std/convert/trait.AsRef.html e
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectivamente.

// Obtenha o número de bytes (não de caracteres) do argumento
// (`.len()` retorna o número de bytes de uma string).
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtenha o número de caracteres (não de bytes) do argumento.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Eleva um número ao quadrado usando `as_mut()`.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    let arg = arg.as_mut();
    *arg *= *arg;
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
