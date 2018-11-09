mod tokenizer;

use tokenizer::*;

fn main() {
    let input = "2 + 3";
    println!("Tokenizing: {}", input);

    let tokens = match tokenizer::tokenize(&input) {
        Ok(tokens) => tokens,
        Err(s) => {
            eprintln!("Could not parse: {}", s);
            std::process::exit(1);
        }
    };

    match interpret(tokens) {
        Ok(n) => println!("Result = {}", n),
        Err(s) => println!("Calculation failed: {}", s),
    }
}
