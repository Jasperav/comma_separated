#[macro_use]
extern crate comma_separated;

#[derive(SomeDerive)]
struct AnotherStruct;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_another_struct() {
        assert_eq!("name, another_name", AnotherStruct::useless())
    }
}