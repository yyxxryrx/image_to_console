use std::io::Read;

pub(crate) fn get_char() -> char {
    let mut buf = vec![0; 1];
    std::io::stdin().lock().read_exact(&mut buf).unwrap();
    buf[0] as char
}
