// Lifetimes também são necessários quando structs armazenam referências.

struct Book<'a> {
    //     ^^^^ anotação de lifetime adicionada
    author: &'a str,
    //       ^^
    title: &'a str,
    //      ^^
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
