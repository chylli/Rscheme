// Here std::io::Write needed for Write trait flush()
use std::io::{self, Write};

fn main() {
    match repl() {
        Err(err) => println!("Error: {}", err),
        _ => ()
    }
}

fn repl() -> io::Result<()>{
    println!("\nWelcome to the Rscheme!\n");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match buffer.as_str() {
            ".quit\n" => {
                println!("Bye!");
                break;
            },
            _ =>  print!("{}", buffer)
        }
    }
    Ok(())

}
