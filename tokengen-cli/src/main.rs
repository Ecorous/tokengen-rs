use clap::Parser;

#[derive(Parser)]
struct TokenGen {
    #[arg(long, short = 'l')]
    length: Option<i32>
}

fn main() {
    let cli = TokenGen::parse();
    println!("token: {}", tokengen_core::generate(cli.length));
}
