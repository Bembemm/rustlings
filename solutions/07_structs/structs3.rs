#![deny(clippy::use_self)] // practice using the `Self` type

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

impl Fireworks {
    fn new() -> Self {
        Self { rockets: 0 }
    }

    fn add_rockets(&mut self, rockets: usize) {
        self.rockets += rockets
    }

    fn start(self) -> String {
        "🚀".repeat(self.rockets)
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_some_fireworks() {
        let f = Fireworks::new();
        assert_eq!(f.start(), "");

        let mut f = Fireworks::new();
        f.add_rockets(3);
        assert_eq!(f.start(), "🚀🚀🚀");

        let mut f = Fireworks::new();
        f.add_rockets(7);
        // No último teste, não usamos a sintaxe de método para garantir que `start`
        // assuma a propriedade do valor de fogos de artifício.
        assert_eq!(Fireworks::start(f), "🚀🚀🚀🚀🚀🚀🚀");
    }
}
