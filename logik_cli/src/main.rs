mod input;

use std::io::{stdin, Error};
use std::time::Instant;
use logik_lib::aussagen::structures::FormelKontext;
use logik_lib::script::parse_line;


fn main() {
    read_script().expect("msg");
}

pub fn read_script() -> Result<(), Error> {
    let mut kontext: FormelKontext = FormelKontext::new();
    let lines = stdin().lines();
    let  time = Instant::now();
    for line in lines {
        let line = line?;
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
