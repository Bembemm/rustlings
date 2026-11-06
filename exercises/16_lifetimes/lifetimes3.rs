// Lifetimes também são necessários quando structs armazenam referências.

// TODO: Corrija os erros de compilação relacionados à struct.
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} por {}", book.title, book.author);
}
