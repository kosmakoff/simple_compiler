mod tokenizer;

use tokenizer::*;

fn main() {
    let input = "2 + 2 * 2";
    println!("Tokenizing: {}", input);

    let tokens = match tokenizer::tokenize(&input) {
        Ok(tokens) => tokens,
        Err(s) => {
            eprintln!("Could not parse: {}", s);
            std::process::exit(1);
        }
    };

    let result = interpret(tokens);
    println!("Result = {}", result);
}
