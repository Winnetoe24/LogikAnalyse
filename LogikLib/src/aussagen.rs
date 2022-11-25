use crate::aussagen::ParseOption::{AND, NOTHING, OR, UNSPECIFIED, VARIABLE};
use slab_tree::{NodeId, Tree, TreeBuilder};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::env::current_exe;
use std::f32::consts::E;
use std::fs::remove_file;
use std::mem::{needs_drop, transmute};
use std::ops::Add;
use std::process::Termination;
use std::ptr::addr_of_mut;

use self::structures::{AussagenFunktion, Belegung, Wahrheitstabelle, FormelKontext};

pub mod structures;

#[derive(Debug)]
struct Parsed {
    option: ParseOption,
}

#[derive(Debug, PartialEq)]
enum ParseOption {
    UNSPECIFIED(),
    NOTHING(),
    VARIABLE(String),
    TOP(),
    BOTTOM(),
    NOT(),
    AND(),
    OR(),
}

pub fn parseFunktion(eingabe: &String) -> Box<AussagenFunktion> {
    let mut parent_funktion = Parsed { option: NOTHING() };
    let mut tree: Tree<Parsed> = TreeBuilder::new().with_root(parent_funktion).build();

    let mut current_node_id = tree.root_id().unwrap();
    for x in eingabe.chars() {
        match x {
            '|' | '⋁' => {
                current_node_id = specify_parent(&mut tree, current_node_id, OR());
            }
            '&' | '⋀' => {
                current_node_id = specify_parent(&mut tree, current_node_id, AND());
            }
            ')' => {
                current_node_id = tree
                    .get(current_node_id)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .node_id();
            }
            '(' => {
                current_node_id = add_value(&mut tree, current_node_id, UNSPECIFIED());
            }
            't' | '⊤' => {
                current_node_id = add_value(&mut tree, current_node_id, ParseOption::TOP());
            }
            'f' | '⊥' => {
                current_node_id = add_value(&mut tree, current_node_id, ParseOption::BOTTOM());
            }
            '-' | '¬' => {
                println!("NOT");
                current_node_id = add_value(&mut tree, current_node_id, ParseOption::NOT());
            }
            ' ' => {}
            _ => match &tree.get_mut(current_node_id).unwrap().data().option {
                VARIABLE(name) => {
                    let mut neu_name = name.clone();
                    neu_name.push(x);
                    tree.get_mut(current_node_id).unwrap().data().option = VARIABLE(neu_name);
                }
                _ => {
                    current_node_id = tree
                        .get_mut(current_node_id)
                        .unwrap()
                        .append(Parsed {
                            option: VARIABLE(String::from(x)),
                        })
                        .node_id();
                }
            },
        }
        println!(
            "after {} => {:?}",
            x,
            tree.get(current_node_id).unwrap().data().option
        );
    }

    printTree(&tree, None);
    to_structures(&tree, tree.root_id().unwrap())
}

fn add_value(tree: &mut Tree<Parsed>, current_node_id: NodeId, option: ParseOption) -> NodeId {
    if tree.get(current_node_id).unwrap().data().option != NOTHING() {
        println!("append");
        tree.get_mut(current_node_id)
            .unwrap()
            .append(Parsed { option })
            .node_id()
    } else {
        tree.get_mut(current_node_id).unwrap().data().option = option;
        current_node_id
    }
}

fn specify_parent(tree: &mut Tree<Parsed>, current_node_id: NodeId, option: ParseOption) -> NodeId {
    let parent_id = tree
        .get(current_node_id)
        .unwrap()
        .parent()
        .unwrap()
        .node_id();
    let mut parent = tree.get_mut(parent_id).unwrap();
    if parent.data().option == UNSPECIFIED() {
        parent.data().option = option;
        parent_id
    } else {
        specify_parent(tree, parent_id, option)
    }
}

fn to_structures(slab_tree: &Tree<Parsed>, node_id: NodeId) -> Box<AussagenFunktion> {
    match &slab_tree.get(node_id).unwrap().data().option {
        UNSPECIFIED() => {
            panic!("Parse nicht erfolgreich")
        }
        VARIABLE(name) => Box::new(AussagenFunktion::VARIABEL(name.clone())),
        ParseOption::TOP() => Box::new(AussagenFunktion::TOP()),
        ParseOption::BOTTOM() => Box::new(AussagenFunktion::BOTTOM()),
        ParseOption::NOT() => {
            let children_id = slab_tree
                .get(node_id)
                .unwrap()
                .children()
                .next()
                .unwrap()
                .node_id();
            Box::new(AussagenFunktion::NOT(to_structures(slab_tree, children_id)))
        }
        AND() => {
            let mut siblings = slab_tree.get(node_id).unwrap().children();
            let children_id = (&mut siblings).next().unwrap().node_id();
            let children_id_2 = siblings.next().unwrap().node_id();
            Box::new(AussagenFunktion::AND(
                to_structures(slab_tree, children_id),
                to_structures(slab_tree, children_id_2),
            ))
        }
        OR() => {
            let mut siblings = slab_tree.get(node_id).unwrap().children();
            let children_id = (&mut siblings).next().unwrap().node_id();
            let children_id_2 = siblings.next().unwrap().node_id();
            Box::new(AussagenFunktion::OR(
                to_structures(slab_tree, children_id),
                to_structures(slab_tree, children_id_2),
            ))
        }
        NOTHING() => {
            let children_id = slab_tree
                .get(node_id)
                .unwrap()
                .children()
                .next()
                .unwrap()
                .node_id();
            to_structures(slab_tree, children_id)
        }
    }
}

fn printTree(slab_tree: &Tree<Parsed>, node_id_option: Option<NodeId>) {
    match node_id_option {
        None => {
            println!();
            let id = slab_tree.root_id();
            println!("{:?}", slab_tree.get(id.unwrap()).unwrap().data().option);
            println!("-----------");
            printTree(slab_tree, id);
        }
        Some(node_id) => {
            let node_ref = slab_tree.get(node_id).unwrap();
            for node in node_ref.children() {
                println!("{:?}", node.data().option);
            }

            println!("-----------");
            for node in node_ref.children() {
                printTree(slab_tree, Some(node.node_id()))
            }
        }
    }
}

pub fn get_belegung(kontext: &FormelKontext, funktionen: &Vec<&AussagenFunktion>, werte: &HashMap<String, bool>) -> Belegung {
    let mut ergebnisse = HashMap::new();
    for aussagen_funktion in funktionen {
        ergebnisse.insert(kontext.get_key(&aussagen_funktion).unwrap(), aussagen_funktion.result(kontext, &werte, false));
    }

    Belegung {
        werte: werte.clone(),
        ergebnisse: ergebnisse,
    }
}



pub fn get_wahrheitstabelle(kontext: &FormelKontext,funktionen: Vec<&AussagenFunktion>) -> Wahrheitstabelle {
    let mut keys: HashSet<&String> = HashSet::new();
    for aussagen_funktionen in &funktionen {
        let mut set = aussagen_funktionen.get_keys(kontext);
         set.extend(keys);
         keys = set;

    }
    let mut keys: Vec<&String> = Vec::from_iter(keys.into_iter());
    

    get_wahrheitstabelle_reku(kontext, &mut keys, &funktionen, &mut HashMap::new())

}

fn get_wahrheitstabelle_reku(kontext: &FormelKontext, keys: &mut Vec<&String>, funktionen: &Vec<&AussagenFunktion>, map: &mut HashMap<String, bool>) -> Wahrheitstabelle {
    let key = keys.pop();
    match key {
        Some(key) => {
            map.insert(key.clone(), false);
            let mut tabelle = get_wahrheitstabelle_reku(kontext, keys, funktionen, map);
            map.insert(key.clone(), true);
            let tabelle2 = get_wahrheitstabelle_reku(kontext, keys, &funktionen, map);
            tabelle.join(tabelle2);
            keys.push(key);
            tabelle
        },
        None => {
            let mut belegungen = Vec::new();
            belegungen.push(get_belegung(kontext,&funktionen, map));
            
            Wahrheitstabelle {
                belegungen,
                reihenfolge: kontext.get_keys(&funktionen)
            }
        },
    }
    
}


