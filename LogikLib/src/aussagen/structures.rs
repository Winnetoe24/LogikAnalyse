use std::collections::hash_map::RandomState;
use std::collections::hash_set::Union;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ptr::addr_of_mut;



#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AussagenFunktion {
    VARIABEL(String),
    TOP(),
    BOTTOM(),
    NOT(Box<AussagenFunktion>),
    AND(Box<AussagenFunktion>, Box<AussagenFunktion>),
    OR(Box<AussagenFunktion>, Box<AussagenFunktion>),
}

impl Display for AussagenFunktion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_utf_string())
    }
}

impl AussagenFunktion {
    pub fn get_keys<'a>(&'a self, kontext: &'a FormelKontext) -> HashSet<&String> {
        match self {
            AussagenFunktion::VARIABEL(key) => {
                if kontext.contains_funktion(key) {
                    return kontext.funktionen.get(key).unwrap().get_keys(kontext);
                }
                HashSet::from([key])
            },
            AussagenFunktion::TOP() | AussagenFunktion::BOTTOM() => HashSet::new(),
            AussagenFunktion::NOT(funktion) => funktion.get_keys(kontext),
            AussagenFunktion::AND(funktion, funktion2)
            | AussagenFunktion::OR(funktion, funktion2) => {
                let mut set = funktion.get_keys(kontext);
                set.extend(&funktion2.get_keys(kontext));
                set
            }
        }
    }

    pub fn result(&self, kontext: &FormelKontext, belegung: &HashMap<String, bool>, default: bool) -> bool {
        match self {
            AussagenFunktion::VARIABEL(key) => {
                if kontext.contains_funktion(key) {
                    kontext.funktionen.get(key).unwrap().result(kontext, belegung, default)
                }else{
                    *belegung.get(key).unwrap_or(&default)
                }
            },
            AussagenFunktion::TOP() => true,
            AussagenFunktion::BOTTOM() => false,
            AussagenFunktion::NOT(funktion) => !funktion.result(kontext,belegung, default),
            AussagenFunktion::AND(funktion, funktion2) => {
                funktion.result(kontext,belegung, default) & funktion2.result(kontext,belegung, default)
            }
            AussagenFunktion::OR(funktion, funktion2) => {
                funktion.result(kontext,belegung, default) || funktion2.result(kontext,belegung, default)
            }
        }
    }

    pub fn to_ascii_string(&self) -> String {
        match self {
            AussagenFunktion::VARIABEL(key) => key.clone(),
            AussagenFunktion::TOP() => String::from("true"),
            AussagenFunktion::BOTTOM() => String::from("false"),
            AussagenFunktion::NOT(funktion) => format!("-{}", funktion.to_ascii_string()),
            AussagenFunktion::AND(funktion, funktion2) => format!(
                "({} & {})",
                funktion.to_ascii_string(),
                funktion2.to_ascii_string()
            ),
            AussagenFunktion::OR(funktion, funktion2) => format!(
                "({} | {})",
                funktion.to_ascii_string(),
                funktion2.to_ascii_string()
            ),
        }
    }
    pub fn to_utf_string(&self) -> String {
        match self {
            AussagenFunktion::VARIABEL(key) => key.clone(),
            AussagenFunktion::TOP() => String::from("⊤"),
            AussagenFunktion::BOTTOM() => String::from("⊥"),
            AussagenFunktion::NOT(funktion) => format!("¬{}", funktion.to_utf_string()),
            AussagenFunktion::AND(funktion, funktion2) => format!(
                "({} ⋀ {})",
                funktion.to_utf_string(),
                funktion2.to_utf_string()
            ),
            AussagenFunktion::OR(funktion, funktion2) => format!(
                "({} ⋁ {})",
                funktion.to_utf_string(),
                funktion2.to_utf_string()
            ),
        }
    }
}

impl Clone for AussagenFunktion {
    fn clone(&self) -> Self {
        match self {
            Self::VARIABEL(arg0) => Self::VARIABEL(arg0.clone()),
            Self::TOP() => Self::TOP(),
            Self::BOTTOM() => Self::BOTTOM(),
            Self::NOT(arg0) => Self::NOT(arg0.clone()),
            Self::AND(arg0, arg1) => Self::AND(arg0.clone(), arg1.clone()),
            Self::OR(arg0, arg1) => Self::OR(arg0.clone(), arg1.clone()),
        }
    }
}

#[derive(Debug)]
pub struct FormelKontext {
    pub funktionen: HashMap<String, AussagenFunktion>,
    pub belegung: Vec<Belegung>,
}

impl FormelKontext {
    pub fn contains_funktion(&self, key: &String) -> bool {
        self.funktionen.contains_key(key)
    }

    pub fn new() -> FormelKontext {
        FormelKontext { funktionen: HashMap::new(), belegung: Vec::new() }
    }

    pub fn get_key(&self, value: &AussagenFunktion) -> Option<String> {
        for ele in &self.funktionen {
            if (ele.1.eq(value)) {return Some(ele.0.clone());}
        }
        None
    }

    pub fn get_keys(&self) -> Vec<String> {
        let mut v = Vec::new();
        for ele in &self.funktionen {
            v.push(ele.0.clone());
        }
        v
    }
}

#[derive(Debug)]
pub struct Belegung {
    pub werte: HashMap<String, bool>,
    pub ergebnisse: HashMap<String, bool>,
}

#[derive(Debug)]
pub struct Wahrheitstabelle {
    pub belegungen: Vec<Belegung>,
    pub reihenfolge: Vec<String>,
}

impl Wahrheitstabelle {
    pub fn join(&mut self, tabelle: Wahrheitstabelle) {
        for belegung in tabelle.belegungen {
            self.belegungen.push(belegung);
        }
    }

}

impl Display for Wahrheitstabelle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let belegung = self.belegungen.get(0);
        if belegung.is_none() {
            ()
        }
        let belegung = belegung.unwrap();
        println!("belegung: {:?}", belegung);
        for ele in &belegung.werte {
            write!(f, "  {}  |", ele.0)?;
        }

        let mut pattern_map:HashMap<String, String> = HashMap::new();
        for ele in &self.reihenfolge {
            let uft_string = ele;
            write!(f, " {} |", uft_string)?;
            let len = uft_string.len();
            let mut pattern = String::with_capacity(len);
            let spaces_len = (len - 2) / 2;
            for x in 0..spaces_len {
                pattern.push(' ');
            }
            pattern.push_str("{}");
            for x in 0..spaces_len {
                pattern.push(' ');
            }
            pattern.push('|');
            pattern_map.insert(ele.clone(), pattern);
        }
        writeln!(f, "")?;


        let def = String::from(" {} |");
        
        for ele in &self.belegungen {
            for ele in &ele.werte {
                if *ele.1 {
                    write!(f, "  1  |")?;
                } else {
                    write!(f, "  0  |")?;
                }
            }
            for erg in &self.reihenfolge {
                let pattern = pattern_map.get(erg).unwrap_or(&def);

                let filled_pattern;
                if *(&ele.ergebnisse).get(erg).unwrap() {
                    filled_pattern = pattern.replace("{}", "1");
                }else {
                    filled_pattern = pattern.replace("{}", "0");
                }
                write!(f, "{}", filled_pattern)?;
            }
            writeln!(f,"")?;
        }

        Ok(())
    }
}
