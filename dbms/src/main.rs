fn main() {
    dbg!(dbms::query_lexer::lex_string("(+-);--())(\n(**"));
}
