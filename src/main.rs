const DIGITS: [char;10] = ['0','1','2','3','4','5','6','7','8','9'];

pub struct SyntaxDiagram {
    //input_string: &'a str,
    input_vectored: Vec<char>,
    current_char_num: usize
}
impl SyntaxDiagram
{
    pub fn new(input_string: &str) -> Self {

        let mut v = Vec::new();
        input_string.chars().for_each(|c| { v.push(c); });

        Self {
            input_vectored: v,
            current_char_num: 0
        }
    }
    pub fn curch(&self) -> char {
        self.input_vectored[self.current_char_num]
    }
    pub fn next(&mut self) -> bool {
        self.current_char_num = self.current_char_num + 1;

        if self.current_char_num >= self.input_vectored.len() {
            return false;
        }

        true
    }
    pub fn is_in(&mut self, symbols: &[char]) -> bool {
        symbols.contains(&self.curch())
    }
}

fn main() {
    let input: String = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Argument error: No input string");
        std::process::exit(1);
    });

    let mut diag = SyntaxDiagram::new(input.as_str());

    if diag.is_in(&['+', '-']) {
        diag.next();
    }
    
    if diag.is_in(&DIGITS) {
        diag.next();
    }
    else {
        eprintln!("Error");
    }

    while diag.is_in(&DIGITS) {
        diag.next();
    }

    if diag.next() == true
    {
        eprintln!("Error EOT");
    }
    
}