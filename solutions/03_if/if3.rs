fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        // Inteiro, para que todos os ramos tenham o mesmo tipo.
        2
    } else if animal == "snake" {
        3
    } else {
        // Qualquer identificador não utilizado.
        4
    };

    // Em Rust, em vez de um identificador assim, normalmente usaríamos uma enumeração.
    // Mas ainda não chegamos às enumerações.
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
