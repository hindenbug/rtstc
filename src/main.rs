mod tokenizer;

fn main() {
    let input = "(add 22 (subtract 4 2))";
    println!("{:?}", tokenizer::tokenizer::tokenize(input));
}
