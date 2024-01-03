use crashc_lexer::tokenize_string;

fn main() {
    let tokens = tokenize_string("== '\'' \"hi\" !=");

    for token in tokens {
        println!("{:?}", token)
    }
}