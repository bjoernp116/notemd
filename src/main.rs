mod music;
mod parser;

fn main() {
    println!("Hello, world!");
}




#[cfg(test)]
mod test {
    #[test]
    fn main_test() {
        let _file = include_bytes!("../test.nomd");

    }
}
