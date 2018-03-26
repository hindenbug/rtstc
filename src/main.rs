mod tokenizer;
mod parser;

fn main() {
    let input = "(concat \"add\" \"this\")";
    let tokens = tokenizer::tokenize(input);

    println!("{:?}", parser::parse(tokens.unwrap()));
}
