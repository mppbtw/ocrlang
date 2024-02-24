use libocr;

fn main() {
    let mut l = libocr::Lexer::new("test string");
    l.next_token();
}
