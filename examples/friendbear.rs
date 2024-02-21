
/// This is a Rust program that demonstrates how to write "Hello, World!" in various programming languages.
/// It uses the `write_hello_world_from_any_language_in_the_rust` crate, which provides the necessary functionality.
/// The program defines a `Programmer` struct with a `Names` field and a `history` vector of `HistoryItem` structs.
/// Each `HistoryItem` represents a programming language and the number of years the programmer has experience with it.
/// The `Programmer` struct implements the `Greet` trait, which defines a `hello_world` method that prints a greeting message.
/// The main function creates an instance of the `Programmer` struct and calls the `hello_world` method to print the greeting.
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