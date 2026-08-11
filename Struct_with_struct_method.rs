struct Book{
    title: String,
    pages: u32,
}

impl Book{
    fn reading_time(&self) -> u32
    {
        self.pages * 2
    }
}

fn main() {
    let book = Book {
        title: String::from("Rust Programming"),
        pages: 500,
    };
    println!("{}: Reading time: {} minutes", book.title, book.reading_time());
}
