mod parser;
mod tokenizer;

fn main() {
    //let input = "(concat \"add\" \"this\")";
    let input = "(add 2 (subtract 4 2))";
    let tokens = tokenizer::tokenize(input);

    println!("{:?}", parser::parse(&tokens.unwrap()));
}
