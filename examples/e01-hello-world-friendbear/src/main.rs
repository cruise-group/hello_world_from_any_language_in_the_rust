/// Rust version hello world.
///
/// `cargo new hello_world && cd hello_world && cargo run`
/// ```
/// println!("Helllo, world!");
/// ````
/// see: <https://users.rust-lang.org/t/rust-lang-version-hello-world-starting-here>
///
fn main() {
    /// Trait
    trait MyTrait<U, T, V> {
        fn hello_world(_: T, _: V) -> U;
    }

    /// Snum
    enum MyEnum {
        // Lannguage, year
        Programming(Vec<(String, u8)>),
        Etc,
    }

    struct MyStruct {
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
    }

    impl MyTrait<String, MyStruct, MyEnum> for MyStruct {
        fn hello_world(s: MyStruct, e: MyEnum) -> String {
            let history = match e {
                MyEnum::Programming(lang) => lang
                    .iter()
                    .map(|(lang, _year)| lang.to_owned())
                    .collect::<Vec<_>>()
                    .join(", "),
                MyEnum::Etc => "".to_owned(),
            };
            format!(
                "Hello! world, My name is {} {} {}\nProgramming is {}",
                s.first_name,
                s.middle_name.unwrap_or_default(),
                s.last_name,
                history,
            )
        }
    }
    let my_struct = MyStruct {
        first_name: "T".to_owned(),
        middle_name: None,
        last_name: "Kumagai".to_owned(),
    };
    let history = MyEnum::Programming(vec![
        ("Basic".to_owned(), 2),
        ("Fortran".to_owned(), 1),
        ("VisualBasic".to_owned(), 4),
        ("VB.NET".to_owned(), 1),
        ("VC++".to_owned(), 3),
        ("Java".to_owned(), 6),
        ("C".to_owned(), 7),
        ("Perl".to_owned(), 1),
        ("JavaScript".to_owned(), 2),
        ("Scala".to_owned(), 1),
        ("Rust".to_owned(), 1),
    ]);
    let hello_world_string =
        <MyStruct as MyTrait<String, MyStruct, MyEnum>>::hello_world(my_struct, history);

    println!("{hello_world_string}");
}
