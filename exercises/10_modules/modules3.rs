// Você pode usar `use` para importar caminhos de outros módulos para
// seu escopo, especialmente os da biblioteca padrão.

// TODO: Importe `SystemTime` e `UNIX_EPOCH` do módulo `std::time` para
// seu escopo. Você ganha pontos extras de estilo se fizer isso em uma linha!
// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC foi há {} segundos!", n.as_secs()),
        Err(_) => panic!("SystemTime anterior à UNIX_EPOCH!"),
    }
}
