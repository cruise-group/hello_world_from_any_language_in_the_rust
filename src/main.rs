/**
* Rust version hello world.
* see: <https://users.rust-lang.org/t/rust-lang-version-hello-world-starting-here>
*/
fn main() {
    /// Trait
    trait MyTrait<U, T, V> {
        fn hello_world(_: T, _: V) -> U;
    }

    /// Enum
    enum MyEnum {
        // Lannguage, year
        Programming(Vec<(String, u8)>),
        Etc,
    }

    /// Struct
    struct MyStruct {
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
    }
}
