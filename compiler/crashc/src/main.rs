use crashc_lexer::tokenize_string;

fn main() {
    let tokens = tokenize_string(r"
        if (true) {
            return false;
        }
        switch () {
        }


    ");

    for token in tokens {
        println!("{:?}", token)
    }
}