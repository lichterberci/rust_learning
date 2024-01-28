fn main() {
    dbg!(dbms::query_lexer::lex_string(
        "(+-)123.32;-334-(\"false\", true))(\n(**"
    ));
}
