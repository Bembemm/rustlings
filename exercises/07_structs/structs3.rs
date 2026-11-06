// As estruturas armazenam dados, mas também podem ter comportamentos.
// Neste exercício, definimos a estrutura `Fireworks` e algumas funções
// que trabalham com ela. Transforme essas funções independentes em métodos
// e funções associadas para deixar essa relação mais clara no código.

#![deny(clippy::use_self)] // pratique o uso do tipo `Self`

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

// TODO: Transforme esta função em uma função associada a `Fireworks`.
fn new_fireworks() -> Fireworks {
    Fireworks { rockets: 0 }
}

// TODO: Transforme esta função em um método de `Fireworks`.
fn add_rockets(fireworks: &mut Fireworks, rockets: usize) {
    fireworks.rockets += rockets
}

// TODO: Transforme esta função em um método de `Fireworks`.
fn start(fireworks: Fireworks) -> String {
    "🚀".repeat(fireworks.rockets)
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
        // No último teste, não usamos a sintaxe de método para garantir
        // que a função `start` assuma a propriedade do valor de fogos de artifício.
        assert_eq!(Fireworks::start(f), "🚀🚀🚀🚀🚀🚀🚀");
    }
}
