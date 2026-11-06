struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Não altere esta função.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Retornar um `Result` seria melhor aqui, mas queremos aprender
            // a testar funções que podem causar `panic`.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: Este teste deve verificar se o retângulo possui as dimensões
        // fornecidas ao construtor.
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // Verifique a largura
        assert_eq!(todo!(), 20); // Verifique a altura
    }

    // TODO: Este teste deve verificar se ocorre `panic` ao tentar criar
    // um retângulo com largura negativa.
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: Este teste deve verificar se ocorre `panic` ao tentar criar
    // um retângulo com altura negativa.
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
