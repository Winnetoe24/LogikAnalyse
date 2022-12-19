extern crate derive_more;

use std::str::SplitWhitespace;

use derive_more::Display;

use crate::aussagen::structures::FormelKontext;
use crate::aussagen::parsing::ParseError;
use crate::script::print::print;
use crate::script::set::set;
use crate::script::tabelle::tabelle;



#[derive(Debug, Display)]
pub enum ScriptAction {
    #[display(fmt = "Funktion Gesetzt: {}", name)]
    ParseFunction {
        name: String,
    },
    #[display(fmt = "{}", ausgabe)]
    Print {
        ausgabe: String,
    },
    #[display(fmt = "Tabelle generiert")]
    GenerateTabelle(),
}

#[derive(Debug)]
pub enum ScriptError {
    FunctionTypeNotImplemented(String),
    WrongSyntax(String),
    ParseNotPossible(String, ParseError),
    TabelleNotGenerated{
        string: String
    },
    FunktionNotFound(String),
}

impl ScriptError {
    pub fn get_string(&self) -> &String {
        match self {
            ScriptError::FunctionTypeNotImplemented(string) => &string,
            ScriptError::WrongSyntax(string) => &string,
            ScriptError::ParseNotPossible(string, _) => &string,
            ScriptError::TabelleNotGenerated {string} => &string,
            ScriptError::FunktionNotFound(string) => &string,
        }
    }
    pub fn set_string(&mut self, new_string: String) {
        match self {
            ScriptError::FunctionTypeNotImplemented(string) => *string = new_string,
            ScriptError::WrongSyntax(string) => *string = new_string,
            ScriptError::ParseNotPossible(string, _) => *string = new_string,
            ScriptError::TabelleNotGenerated{string} => *string = new_string,
            ScriptError::FunktionNotFound(string) => *string = new_string
        }
    }
}

/// begins to parse the line of text. Delegates the everything but the first word to other functions.
pub fn parse_line(line: &str, kontext: &mut FormelKontext) -> Result<ScriptAction, ScriptError> {
    //Key Word Match
    let mut iterator: SplitWhitespace = line.split_whitespace();
    let next = iterator.next();
    if next.is_none() {
        return Err(ScriptError::WrongSyntax(String::from(line)));
    }
    let result: Result<ScriptAction, ScriptError> = match next.unwrap() {
        "set" | "SET" => set(iterator, kontext),
        "print" | "PRINT" => print(iterator, kontext),
        "tabelle" | "TABELLE" => tabelle(iterator,kontext),
        s => Err(ScriptError::FunctionTypeNotImplemented(String::from(s))),
    };

    beatify_error(line, result)
}

/// replaces the Strings in Errors when they are empty with the line
fn beatify_error(
    line: &str,
    result: Result<ScriptAction, ScriptError>,
) -> Result<ScriptAction, ScriptError> {
    match result {
        Ok(_) => result,
        Err(mut error) => {
            if error.get_string().is_empty() {
                error.set_string(String::from(line));
            }
            Err(error)
        }
    }
}

fn get_rest(iterator: &mut SplitWhitespace) -> Result<String, ScriptError> {
    let next = iterator.next();
    if next.is_none() {
        return Err(ScriptError::WrongSyntax(String::new()));
    }
    let mut rest = String::from(next.unwrap());

    for ele in iterator {
        rest.push_str(ele);
    }
    Ok(rest)
}

mod set {
    use std::str::SplitWhitespace;

    use crate::aussagen::structures::FormelKontext;
    use crate::script::get_rest;

    use super::{ScriptAction, ScriptError};

    pub(super) fn set(
        mut iterator: SplitWhitespace,
        kontext: &mut FormelKontext,
    ) -> Result<ScriptAction, ScriptError> {
        let next = iterator.next();
        if next.is_none() {
            return Err(ScriptError::WrongSyntax(String::new()));
        }
        match next.unwrap() {
            "AUSSAGEN" => set_aussagen(iterator, kontext),
            s => Err(ScriptError::FunctionTypeNotImplemented(String::from(s))),
        }
    }

    fn set_aussagen(
        mut iterator: SplitWhitespace,
        kontext: &mut FormelKontext,
    ) -> Result<ScriptAction, ScriptError> {
        let next = iterator.next();
        if next.is_none() {
            return Err(ScriptError::WrongSyntax(String::new()));
        }
        let name = next.unwrap();



        let formel = get_rest(&mut iterator)?;

        match crate::aussagen::parsing::parse_function(formel.as_str()) {
            Ok(formel) => {
                kontext.funktionen.insert(String::from(name), *formel);
                Ok(ScriptAction::ParseFunction {
                    name: String::from(name),
                })
            }
            Err(parse_error) => Err(ScriptError::ParseNotPossible(
                String::from(formel),
                parse_error,
            )),
        }
    }


}

mod print {
    use std::collections::HashMap;
    use std::str::SplitWhitespace;
    use crate::aussagen::{get_belegung, is_aequivalent};

    use crate::aussagen::structures::FormelKontext;
    use crate::script::ScriptAction::Print;
    use crate::script::ScriptError::{FunktionNotFound, TabelleNotGenerated};

    use super::{ScriptAction, ScriptError};

    pub(super) fn print(
        mut iterator: SplitWhitespace,
        kontext: &mut FormelKontext,
    ) -> Result<ScriptAction, ScriptError> {
        let next = iterator.next();
        if next.is_none() {
            return Err(ScriptError::WrongSyntax(String::new()));
        }
        match next.unwrap() {
            "Formel-UTF" => print_formel_utf(iterator, kontext),
            "Formel-ASCII" => print_formel_ascii(iterator, kontext),
            "Tabelle" => print_tabelle(kontext),
            "Belegung" | "BELEGUNG" => print_belegung(iterator, kontext),
            "aequivalenz" | "AEQUIVALENZ" => print_aequivalenz(iterator,kontext),
            _ => return Err(ScriptError::WrongSyntax(String::new())),
        }
    }

    fn print_formel_utf(
        mut iterator: SplitWhitespace,
        kontext: &mut FormelKontext,
    ) -> Result<ScriptAction, ScriptError> {

        let next = iterator.next();
        if next.is_none() {
            return Err(ScriptError::WrongSyntax(String::new()));
        }
        let next = next.unwrap();

        let funktion = kontext.funktionen.get(&String::from(next));
        if funktion.is_none() {
            Err(ScriptError::WrongSyntax(String::from(next)))
        } else {
            Ok(Print {
                ausgabe: funktion.unwrap().to_utf_string(),
            })
        }
    }

    fn print_formel_ascii(
        mut iterator: SplitWhitespace,
        kontext: &mut FormelKontext,
    ) -> Result<ScriptAction, ScriptError> {

        let next = iterator.next();
        if next.is_none() {
            return Err(ScriptError::WrongSyntax(String::new()));
        }
        let next = next.unwrap();

        let funktion = kontext.funktionen.get(&String::from(next));
        if funktion.is_none() {
            Err(ScriptError::WrongSyntax(String::from(next)))
        } else {
            Ok(Print {
                ausgabe: funktion.unwrap().to_ascii_string(),
            })
        }
    }

    fn print_tabelle(
        kontext: &mut FormelKontext,
    ) -> Result<ScriptAction, ScriptError> {
        if  kontext.tabelle.is_none() {
            return Err(TabelleNotGenerated{string: String::new()});
        }
        let string = format!("{}", &kontext.tabelle.as_ref().unwrap());
        Ok(Print {ausgabe: string})
    }

    fn print_belegung(mut iterator: SplitWhitespace,
                      kontext: &mut FormelKontext,) -> Result<ScriptAction, ScriptError> {

        let  mut vec = Vec::new();
        let mut next = iterator.next();
        while next.is_some() {
            let name = next.unwrap();
            match name {
                "|" => break,
                name => {
                    let option = kontext.funktionen.get(&*String::from(name));
                    if  option.is_none() {
                        return Err(FunktionNotFound(String::from(name)))
                    }
                    vec.push(option.unwrap());
                }
            }
            next = iterator.next();
        }

        let mut werte = HashMap::new();
        for name in iterator {
            werte.insert(String::from(name), true);
        }

        let belegung = get_belegung(kontext, &vec, &werte);

        Ok(Print {ausgabe: format!("{}", belegung)})
    }

   fn print_aequivalenz(mut iterator: SplitWhitespace,
                                   kontext: &mut FormelKontext,) -> Result<ScriptAction, ScriptError> {
        let  mut vec = Vec::new();
        for name in iterator {
            let option = kontext.funktionen.get(&*String::from(name));
            if  option.is_none() {
                return Err(FunktionNotFound(String::from(name)))
            }
            vec.push(option.unwrap());
        }
        Ok(Print {ausgabe: format!("{}",  is_aequivalent(kontext, vec))})
    }
}

mod tabelle {
    use std::mem::transmute;
    use std::str::SplitWhitespace;
    use crate::aussagen::get_wahrheitstabelle;

    use crate::aussagen::structures::FormelKontext;
    use crate::script::ScriptAction::{GenerateTabelle, Print};
    use crate::script::{ScriptAction, ScriptError};
    use crate::script::ScriptError::FunktionNotFound;

    pub(super) fn tabelle(
        iterator: SplitWhitespace,
        kontext: &mut FormelKontext,
    ) -> Result<ScriptAction, ScriptError> {
        let  mut vec = Vec::new();
        for name in iterator {
            let option = kontext.funktionen.get(&*String::from(name));
            if  option.is_none() {
                return Err(FunktionNotFound(String::from(name)))
            }
            vec.push(option.unwrap());
        }
        let tabelle = get_wahrheitstabelle(kontext, vec);
        kontext.tabelle = Some(tabelle);
        Ok(GenerateTabelle())
    }
}
