use dbms::query_lexer::lex_string;

fn main() {
    dbg!(dbms::query_lexer::lex_string(
        r#"

        ;--SELECT * FROM table WHERE table.column < 12.3 or table.column2 == false;

        ;--INSERT INTO table (col1, col2, col3) VALUES (3.14, "szia mia", false);-- this here is a comment so it should be ignoerd

        SELECT tab1.col1, tab2.col2 
        FROM tab1, tab2 
        WHERE tab1.col == tab2.col
        AND tab1.col123 != false OR tab1.col321 < 18.0

    "#
    ));

    // dbg!(lex_string("and and or"));
}
