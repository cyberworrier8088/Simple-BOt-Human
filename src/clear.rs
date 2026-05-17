use std::io;
use std::io::Write;
// clear function
pub fn clear() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    io::stdout().flush().unwrap();
}
