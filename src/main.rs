mod tokenizer;
mod parser;

fn main() {
    let input = "(concat \"add\" \"this\")";
    println!("{:?}", tokenizer::tokenize(input));
}
