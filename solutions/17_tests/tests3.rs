struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Não altere esta função.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Retornar um `Result` seria melhor aqui, mas queremos aprender
            // como testar funções que podem causar `panic`.
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
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Check width
        assert_eq!(rect.height, 20); // Check height
    }

    #[test]
    #[should_panic] // Added this attribute to check that the test panics.
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic] // Added this attribute to check that the test panics.
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
