#[macro_use]
extern crate dotenv_codegen;

pub mod phone;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub const IS_DEV: bool = const_str::equal!(dotenv!("DEV"), "1")
    || const_str::equal!(dotenv!("DEV"), "true")
    || const_str::equal!(dotenv!("DEV"), "t");
