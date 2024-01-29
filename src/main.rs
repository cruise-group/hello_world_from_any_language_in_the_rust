/**
* Rust version hello world.
* see: <https://users.rust-lang.org/t/rust-lang-version-hello-world-starting-here>
*/
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

    /// Struct(first_name, middle_name, last_name)
    struct MyStruct(String, Option<String>, String);
}
