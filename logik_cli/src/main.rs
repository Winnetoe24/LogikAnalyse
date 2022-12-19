mod input;

use std::io::{stdin, Error};
use std::time::Instant;
use clap::Parser;
use logik_lib::aussagen::structures::FormelKontext;
use logik_lib::script::parse_line;
use termimad::*;

static SYNTAX_HELP: &'static str = include_str!("../resources/syntax_help.md");


#[derive(Parser, Debug)]
#[command(author = "Alexander Brand", version = "0.2", about = "Führt alle Commands aus dem stdin aus.")]
pub struct CLIOptions {
    #[arg(short = 'i', long = "printInstructions", help = "Ob jeder Eingegebene Command ausgegeben wird.\nHilfreich für Script-Dateien")]
    pub print_instructions: bool,

    #[arg(short = 's', long = "printSyntax", help = "Gibt die Hilfe für die Syntax der Commands aus")]
    pub print_syntax: bool,
}



fn main() {
    let  args  = CLIOptions::parse();
    if args.print_syntax {
        print_inline(SYNTAX_HELP);
        return;
    }
    read_script(args).expect("msg");
}

pub fn read_script(args: CLIOptions) -> Result<(), Error> {
    let mut kontext: FormelKontext = FormelKontext::new();
    let lines = stdin().lines();
    let  time = Instant::now();
    for line in lines {
        let line = line?;
        if args.print_instructions {
            println!("{}",line);
        }
        match parse_line(line.as_str(), &mut kontext) {
            Ok(action) => println!("{}", action),
            Err(error) => {
                println!("{:?}",error)
            },
        }
    }
    println!("Finished in {:?}", time.elapsed());

    Ok(())
}

