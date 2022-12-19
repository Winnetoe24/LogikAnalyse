use parsing::ParseError::*;
use parsing::ParseOption::{AND, NOTHING, OR, UNSPECIFIED, VARIABLE};
use slab_tree::{NodeId, Tree, TreeBuilder};

use std::collections::{HashMap, HashSet};
use std::ptr::{addr_of, addr_of_mut};
use parsing::{Parsed, ParseError, ParseOption};

use self::structures::{AussagenFunktion, Belegung, FormelKontext, Wahrheitstabelle};

pub mod structures;
pub mod parsing;

pub fn get_belegung(
    kontext: &FormelKontext,
    funktionen: &Vec<&AussagenFunktion>,
    werte: &HashMap<String, bool>,
) -> Belegung {
    let mut ergebnisse = HashMap::new();
    for aussagen_funktion in funktionen {
        ergebnisse.insert(
            kontext.get_key(&aussagen_funktion).unwrap(),
            aussagen_funktion.result(kontext, &werte, false),
        );
    }

    Belegung {
        werte: werte.clone(),
        ergebnisse: ergebnisse,
    }
}

fn get_all_keys<'a>(kontext: &'a FormelKontext, funktionen: &'a Vec<&AussagenFunktion>) -> Vec<&'a String> {
    let mut keys: HashSet<&String> = HashSet::new();
    for aussagen_funktionen in funktionen {
        let mut set = aussagen_funktionen.get_keys(kontext);
        set.extend(keys);
        keys = set;
    }
    let mut keys: Vec<&String> = Vec::from_iter(keys.into_iter());
    keys
}

fn call_for_every_belegung<T>(kontext: &FormelKontext,  keys: &mut Vec<&String>, funktionen: &Vec<&AussagenFunktion>, map: &mut HashMap<String, bool>, funktion: fn(Belegung) -> T, joiner: fn(T,T) -> T) -> T {
    let key = keys.pop();
    match key {
        Some(key) => {
            map.insert(key.clone(), false);
            let mut erstes_element: T = call_for_every_belegung(kontext, keys, funktionen, map, funktion, joiner);
            map.insert(key.clone(), true);
            let mut zweites_element: T = call_for_every_belegung(kontext, keys, funktionen, map, funktion, joiner);
            keys.push(key);
            joiner(erstes_element, zweites_element)
        }
        None =>  {
            let belegung = get_belegung(kontext, &funktionen, map);
            funktion(belegung)
        }
    }
}

pub fn get_wahrheitstabelle(
kontext: &FormelKontext,
funktionen: Vec<&AussagenFunktion>,
) -> Wahrheitstabelle {
    let mut keys = get_all_keys(kontext, &funktionen);
    let mut wahrheitstabelle = call_for_every_belegung(kontext, &mut keys, &funktionen, &mut HashMap::new(), to_tabelle, join_tabellen);
    wahrheitstabelle.reihenfolge = kontext.get_keys(&funktionen);
    wahrheitstabelle
}


fn to_tabelle(belegung: Belegung) -> Wahrheitstabelle {
    let mut belegungen = Vec::new();
    belegungen.push(belegung);

    Wahrheitstabelle {
        belegungen,
        reihenfolge: Vec::new(),
    }
}
fn join_tabellen(mut t1: Wahrheitstabelle, t2: Wahrheitstabelle) -> Wahrheitstabelle {
    t1.join(t2);
    t1
}

pub fn is_aequivalent(kontext: &FormelKontext, funktionen: Vec<&AussagenFunktion>) -> bool {
    call_for_every_belegung(kontext, &mut get_all_keys(kontext,&funktionen), &funktionen, &mut HashMap::new(), is_belegung_aequivalent, join_bool_and)
}

fn is_belegung_aequivalent(belegung: Belegung) -> bool {
    let mut iter = belegung.ergebnisse.into_iter();
    let option = iter.next();
    if option.is_none() {
        return true;
    }
    let value = option.unwrap().1;
    for tupel in iter {
        if  tupel.1 != value {
            return false;
        }
    }
    return true;
}

fn join_bool_and(a: bool, b: bool)-> bool {
    a && b
}
