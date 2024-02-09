use dbms::query_parser::{parse, parse_boolean_expression, TokenSupplier};

fn main() {
    let tokenizer = dbms::query_tokenizer::QueryTokenizer::new();

    let token_vec = tokenizer.tokenize(
        r#"

        ;--SELECT * FROM table WHERE table.column < 12.3 or table.column2 == false;

        ;--INSERT INTO table (col1, col2, col3) VALUES (3.14, "szia mia", false);-- this here is a comment so it should be ignoerd

        SELECT tab1.col1, tab2.col2, tab3.col3
        FROM tab1, tab2, tab3
        WHERE tab1.col == tab2.col
        AND tab2.col222 == tab3.col222
        AND tab1.col123 != false 
        OR tab3.col321 < 18.0;

        ;--asd.col / (13 + asd.col2) == 123 or (asd == true or not dsa.col2 > 12 - 3) and sda != false
        ;--asd == dsa.loc or 1 == 1 *2
    "#
    ).unwrap();

    println!("{:#?}", parse(&mut TokenSupplier::new(token_vec)))

    // dbg!(lex_string("and and or"));
}
