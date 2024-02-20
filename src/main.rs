enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn print_publication_info(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(book) => {
                println!("Kitap: {} yazar: {}, {} sayfa", book.title, book.author, book.page_count);
            },
            Publication::Magazine(magazine) => {
                println!("Dergi: {} - Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
            },
        }
    }
}

fn main() {
    let book = Publication::Book(Book {
        title: String::from("Rust Programlama Dili"),
        author: String::from("K. Yazar"),
        page_count: 320,
    });

    let magazine = Publication::Magazine(Magazine {
        title: String::from("Bilim ve Teknik"),
        issue: 202,
        topic: String::from("Uzay Keşifleri"),
    });

    let publications = vec![book, magazine];


    print_publication_info(publications);
}
