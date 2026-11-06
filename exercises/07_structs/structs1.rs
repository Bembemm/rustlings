struct ColorRegularStruct {
    // TODO: Adicione os campos esperados pelo teste `regular_structs`.
    // Quais devem ser os tipos desses campos? Quais são os valores mínimo e máximo das cores RGB?
}

struct ColorTupleStruct(/* TODO: Adicione os campos esperados pelo teste `tuple_structs` */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Crie uma instância de uma estrutura com campos nomeados.
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Crie uma instância de uma estrutura em forma de tupla.
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Crie uma instância de uma estrutura unitária.
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
