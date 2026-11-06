// Neste exercício, representamos o conceito de múltiplos donos usando
// o tipo `Rc<T>`. O exemplo modela o Sistema Solar: há um tipo `Sun` e
// vários `Planet`. Os planetas compartilham a propriedade do Sol, indicando que
// orbitam ao redor dele.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Olá de {self:?}!");
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        let sun = Rc::new(Sun);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 1 referência

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 2 referências
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 3 referências
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 4 referências
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 5 referências
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 6 referências
        jupiter.details();

        let saturn = Planet::Saturn(Rc::clone(&sun));
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 7 referências
        saturn.details();

        let uranus = Planet::Uranus(Rc::clone(&sun));
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 8 referências
        uranus.details();

        let neptune = Planet::Neptune(Rc::clone(&sun));
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 9 referências
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 8 referências

        drop(uranus);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 7 referências

        drop(saturn);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 6 referências

        drop(jupiter);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 5 referências

        drop(mars);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 4 referências

        drop(earth);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 3 referências

        drop(venus);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 2 referências

        drop(mercury);
        println!("contagem de referências = {}", Rc::strong_count(&sun)); // 1 referência

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
