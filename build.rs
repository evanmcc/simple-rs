use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .lexer_in_src_dir("simple.l")
        .unwrap()
        .build()
        .unwrap();
}
