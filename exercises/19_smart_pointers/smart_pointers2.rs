// Neste exercício, vamos representar múltiplos donos usando `Rc<T>`.
// O exemplo modela o Sistema Solar: existe um tipo `Sun` e vários `Planet`.
// Cada planeta mantém uma referência compartilhada ao mesmo Sol.

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
        println!("Hi from {self:?}!");
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
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 referência

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 referências
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 referências
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 referências
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 referências
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 referências
        jupiter.details();

        // TODO
        let saturn = Planet::Saturn(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 referências
        saturn.details();

        // TODO
        let uranus = Planet::Uranus(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 referências
        uranus.details();

        // TODO
        let neptune = Planet::Neptune(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 9 referências
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 referências

        drop(uranus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 referências

        drop(saturn);
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 referências

        drop(jupiter);
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 referências

        drop(mars);
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 referências

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 referências

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 referências

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 referência

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
