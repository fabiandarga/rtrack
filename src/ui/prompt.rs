use std::io::Write;

pub fn prompt(msg:&str) -> String {
    let mut line = String::new();
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}