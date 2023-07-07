fn main() {
    let gen = tokengen_core::generate(Some(888));
    println!("{}", gen);
    println!("length: {}", gen.len())
}
