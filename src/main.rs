use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    match repl() {
        Err(err) => println!("Error: {}", err),
        _ => ()
    }
}

fn repl() -> Result<(),String>{
    println!("\nWelcome to the Rscheme!\n");
    let mut rl = Editor::<()>::new();
    let history = "/tmp/rscheme_history"; // TODO change it to ./.rscheme_history
    if let Err(e) = rl.load_history(history){ // TODO handle error more elegently
        println!("There a problem when loading {}: {}.", history, e);
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                match line.as_str(){
                    ".quit" => {
                        println!("Bye!");
                        break;
                    },
                    _ =>  println!("{}", line)
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Bye!");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Bye!");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(history).unwrap();
    Ok(())

}
