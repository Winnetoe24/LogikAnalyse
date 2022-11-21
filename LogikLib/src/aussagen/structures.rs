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
    pub fn get_keys(&self) -> HashSet<&String> {
        match self {
            AussagenFunktion::VARIABEL(key) => HashSet::from([key]),
            AussagenFunktion::TOP() | AussagenFunktion::BOTTOM() => HashSet::new(),
            AussagenFunktion::NOT(funktion) => funktion.get_keys(),
            AussagenFunktion::AND(funktion, funktion2)
            | AussagenFunktion::OR(funktion, funktion2) => {
                let mut set = funktion.get_keys();
                set.extend(&funktion2.get_keys());
                set
            }
        }
    }
    pub fn result(&self, belegung: &HashMap<String, bool>, default: bool) -> bool {
        match self {
            AussagenFunktion::VARIABEL(key) => *belegung.get(key).unwrap_or(&default),
            AussagenFunktion::TOP() => true,
            AussagenFunktion::BOTTOM() => false,
            AussagenFunktion::NOT(funktion) => !funktion.result(belegung, default),
            AussagenFunktion::AND(funktion, funktion2) => {
                funktion.result(belegung, default) & funktion2.result(belegung, default)
            }
            AussagenFunktion::OR(funktion, funktion2) => {
                funktion.result(belegung, default) || funktion2.result(belegung, default)
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

    // pub fn clone(&self) -> AussagenFunktion {
    //     match self {
    //         AussagenFunktion::VARIABEL(key) => AussagenFunktion::VARIABEL(key.clone()),
    //         AussagenFunktion::TOP() => AussagenFunktion::TOP(),
    //         AussagenFunktion::BOTTOM() => AussagenFunktion::BOTTOM(),
    //         AussagenFunktion::NOT(funk) => AussagenFunktion::NOT(Box::new(funk.clone())),
    //         AussagenFunktion::AND(funk, funk2) => todo!(),
    //         AussagenFunktion::OR(funk, funk2) => todo!(),
    //     }
    // }

    // fn is_equivalent(&self, belegbar: &dyn Funktion) -> bool {
    //
    //     let union = self.get_keys()
    //         .union(&belegbar.get_keys())
    //         .collect();
    //     self.is_equivalent(belegbar,union, &mut HashMap::new())
    // }
    // fn is_equivalent_internal(&self, belegbar: &dyn Funktion, restliche_keys: &mut HashSet<&String, RandomState>, map: &mut HashMap<&String, bool>) -> bool {
    //     let mut iter = restliche_keys.iter();
    //     let option = iter.next();
    //     match option {
    //         None => {
    //             self.result(map, false) == belegbar.result(map, false)
    //         }
    //         Some(key) => {
    //             restliche_keys.remove(key);
    //             if !self.is_equivalent(belegbar, restliche_keys, map) {
    //                 false
    //             }
    //             map.insert(key,true);
    //             self.is_equivalent(belegbar, restliche_keys, map)
    //         }
    //     }
    // }
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
pub struct Belegung {
    pub funktionen: Vec<AussagenFunktion>,
    pub werte: HashMap<String, bool>,
    pub ergebnis: Vec<bool>,
}

#[derive(Debug)]
pub struct Wahrheitstabelle {
    pub belegungen: Vec<Belegung>,
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

        let mut pattern_map:HashMap<&AussagenFunktion, String> = HashMap::new();
        for ele in &belegung.funktionen {
            let uft_string = ele.to_utf_string();
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
            pattern_map.insert(ele, pattern);
        }
        writeln!(f, "")?;

        for ele in &self.belegungen {
            for ele in &ele.werte {
                if *ele.1 {
                    write!(f, "  1  |")?;
                } else {
                    write!(f, "  0  |")?;
                }
            }
            let mut iterator = ele.ergebnis.iter();
            for ele in &ele.funktionen {
                match &iterator.next() {
                    Some(erg) =>{
                        let pattern = pattern_map.get(&ele).unwrap().as_str();
                        let filled_pattern;
                        if **erg {
                            filled_pattern = pattern.replace("{}", "1");
                        }else {
                            filled_pattern = pattern.replace("{}", "0");
                        }
                        write!(f, "{}", filled_pattern)?;
                    },
                    None => {
                        let pattern = pattern_map.get(&ele).unwrap().as_str();
                        write!(f,"{}",pattern.replace("{}", "?"))?;
                    },
                }
            }
            writeln!(f,"")?;
        }

        Ok(())
    }
}
