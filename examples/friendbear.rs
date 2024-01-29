use write_hello_world_from_any_language_in_the_rust::{Greet, HistoryItem, Names, Programmer};

fn main() {
    let me = Programmer {
        names: Names {
            first_name: "T".to_owned(),
            middle_name: None,
            last_name: "Kumagai".to_owned(),
        },
        history: vec![
            HistoryItem::Programming("Basic".to_owned(), 2),
            HistoryItem::Programming("Fortran".to_owned(), 1),
            HistoryItem::Programming("VisualBasic".to_owned(), 4),
            HistoryItem::Programming("VB.NET".to_owned(), 1),
            HistoryItem::Programming("VC++".to_owned(), 3),
            HistoryItem::Programming("Java".to_owned(), 6),
            HistoryItem::Programming("C".to_owned(), 7),
            HistoryItem::Programming("Perl".to_owned(), 1),
            HistoryItem::Programming("JavaScript".to_owned(), 2),
            HistoryItem::Programming("Scala".to_owned(), 1),
            HistoryItem::Programming("Rust".to_owned(), 1),
        ],
    };

    println!("{}", me.hello_world());
}
