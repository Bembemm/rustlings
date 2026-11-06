// Vamos definir um modelo simples para acompanhar o progresso dos exercícios do Rustlings. O progresso
// será representado por um hash map. O nome do exercício é a chave e
// o progresso é o valor. Duas funções de contagem foram criadas para contar
// quantos exercícios possuem determinado progresso. Recrie essa
// funcionalidade usando iteradores. Tente não usar laços imperativos (`for`/`while`).

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` é um hash map com chaves `String` e valores `Progress`.
    // map = { "variables1": Complete, "conversions3": None, … }
    map.values().filter(|val| **val == value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        count += count_for(map, value);
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // `collection` é um slice de hash maps.
    // collection = [{ "variables1": Complete, "conversions3": None, … },
    //               { "variables2": Complete, … }, … ]
    collection
        .iter()
        .map(|map| count_iterator(map, value))
        .sum()
}

// Equivalente a `count_collection_iterator` e `count_iterator`, iterando como
// se a coleção fosse um único contêiner em vez de um contêiner de contêineres
// (ou, mais precisamente, um único iterador em vez de um iterador de iteradores).
fn count_collection_iterator_flat(
    collection: &[HashMap<String, Progress>],
    value: Progress,
) -> usize {
    // `collection` is a slice of hash maps.
    // collection = [{ "variables1": Complete, "conversions3": None, … },
    //               { "variables2": Complete, … }, … ]
    collection
        .iter()
        .flat_map(HashMap::values) // or just `.flatten()` when wanting the default iterator (`HashMap::iter`)
        .filter(|val| **val == value)
        .count()
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;
    use Progress::*;

    fn get_map() -> HashMap<String, Progress> {
        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmaps1"), Complete);
        map.insert(String::from("smart_pointers3"), Some);
        map.insert(String::from("conversions5"), None);
        map.insert(String::from("conversions3"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("conversions2"), None);
        other.insert(String::from("conversions4"), None);

        vec![map, other]
    }

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Complete), 3);
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Some), 1);
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(count_iterator(&map, None), 2);
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = [Complete, Some, None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state),
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Complete), 6);
        assert_eq!(count_collection_iterator_flat(&collection, Complete), 6);
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Some), 1);
        assert_eq!(count_collection_iterator_flat(&collection, Some), 1);
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, None), 4);
        assert_eq!(count_collection_iterator_flat(&collection, None), 4);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        let progress_states = [Complete, Some, None];

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state),
            );
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator_flat(&collection, progress_state),
            );
        }
    }
}
