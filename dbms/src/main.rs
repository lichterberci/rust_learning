use dbms::query_parser::{parse_boolean_expression, TokenSupplier};

fn main() {
    let tokenizer = dbms::query_tokenizer::QueryTokenizer::new();

    let token_vec = dbg!(tokenizer.tokenize(
        r#"

        ;--SELECT * FROM table WHERE table.column < 12.3 or table.column2 == false;

        ;--INSERT INTO table (col1, col2, col3) VALUES (3.14, "szia mia", false);-- this here is a comment so it should be ignoerd

        ;--SELECT tab1.col1, tab2.col2 
        ;--FROM tab1, tab2 
        ;--WHERE tab1.col == tab2.col
        ;--AND tab1.col123 != false OR tab1.col321 < 18.0

        ;-- (asd.col == 123 or dsa.col2 > 12) and not (sda.loc == false)
        asd == dsa.loc
    "#
    ).unwrap());

    if let Err(err) = parse_boolean_expression(&mut TokenSupplier::new(token_vec)) {
        println!("Parsing error: {}", err);
    } else {
        println!("Parsing successful!")
    }

    // dbg!(lex_string("and and or"));
}
