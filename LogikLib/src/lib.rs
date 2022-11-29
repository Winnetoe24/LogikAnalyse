pub mod aussagen;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use slab_tree::Tree;
    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::aussagen::structures::{FormelKontext, Belegung};
    use crate::aussagen::ParseOption::{VARIABLE};
    use crate::aussagen::{parseFunktion, get_wahrheitstabelle, Parsed, ParseOption};
    use crate::aussagen::structures::AussagenFunktion::{*, self};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let kontext = FormelKontext::new();

        assert!(
            VARIABEL(String::from("A")).result(&kontext, &HashMap::from([(String::from("A"), true)]), false)
        );
        assert!(TOP().result(&kontext, &HashMap::new(), false));
        assert!(!BOTTOM().result(&kontext, &HashMap::new(), false));

        assert!(AND(vec![Box::new(TOP()), Box::new(TOP())]).result(&kontext, &HashMap::new(), false));
        assert!(OR(vec![Box::new(TOP()), Box::new(TOP())]).result(&kontext, &HashMap::new(), false));
        assert!(NOT(Box::new(BOTTOM())).result(&kontext, &HashMap::new(), false));

        assert!(AND(
            vec![
                Box::new(OR(vec![
                    Box::new(BOTTOM()),
                    Box::new(NOT(Box::new(BOTTOM())))
                ]
                )),
                Box::new(TOP())
            ]
        )
        .result(&kontext, &HashMap::new(), false));

        let mut belegung_map = HashMap::new();
        belegung_map.insert(String::from("A"), false);
        belegung_map.insert(String::from("B"), false);
        assert!(!parseFunktion(&String::from("(A & B)")).expect("parse1").result(&kontext, &belegung_map, false));
        belegung_map.insert(String::from("A"), true);
        belegung_map.insert(String::from("B"), false);
        assert!(!parseFunktion(&String::from("(A & B)")).expect("parse2").result(&kontext, &belegung_map, false));
        belegung_map.insert(String::from("A"), false);
        belegung_map.insert(String::from("B"), true);
        assert!(!parseFunktion(&String::from("(A & B)")).expect("parse3").result(&kontext, &belegung_map, false));
        belegung_map.insert(String::from("A"), true);
        belegung_map.insert(String::from("B"), true);
        assert!(parseFunktion(&String::from("(A & B)")).expect("parse4").result(&kontext, &belegung_map, false));


        test_parse("(F ⋀ C)");
        test_parse("(F ⋁ (phi1 ⋀ phi2))");
        test_parse_ascii("(F & C)");
        test_parse_ascii("-F");
        test_parse_ascii("(F & -(phi1 | phi2))");
    }

    #[test]
    fn ascii1() {
        test_parse_ascii("(F & (-phi1 & phi2))");
    }

    fn test_parse(formel: &str) {
        let funktion = parseFunktion(&String::from(formel)).expect("couldnt parse");
        assert_eq!(funktion.to_utf_string(), formel);
    }

    fn test_parse_ascii(formel: &str) {
        let funktion = parseFunktion(&String::from(formel)).expect("couldnt parse");
        assert_eq!(funktion.to_ascii_string(), formel);
    }

    #[test]
    fn test_funktion() {
        let belegung = HashMap::from([(String::from("A"), true), (String::from("B"), false), (String::from("C"), false)]) ;
        let funktion:AussagenFunktion = *parseFunktion(&String::from("(A & ( B | C))")).expect("couldnt parse").to_owned();
        let kontext = FormelKontext { funktionen : HashMap::from([(String::from("phi1"), funktion.clone())]), belegung: vec![] };
        assert!(!funktion.result(&kontext, &belegung, false))
    }

    #[test]
    fn wahrheitstabelle() {
        let mut kontext = FormelKontext::new();
        
        let funktion:AussagenFunktion = *parseFunktion(&String::from("(A | (B))")).expect("couldnt parse").to_owned();
        kontext.funktionen.insert(String::from("phi1"), funktion.clone());

        let tabelle = get_wahrheitstabelle(&kontext, vec![&funktion]);
        println!("{:?}", tabelle);
        println!("{}", tabelle);
    }

    #[test]
    fn test_simple_functions() {
        test_parse_ascii("A");
        test_parse("⊥");
        test_parse("⊤");
    }

    #[test]
    fn swich_parent() {
        let mut tree:Tree<Parsed> = Tree::new();
        tree.set_root(Parsed {option: VARIABLE(String::from("1"), false) });

        let mut current = tree.root_mut().unwrap();
        let new_id = &current
            .append(Parsed {
                option: VARIABLE(String::from("2"), false)
            })
            .node_id();
        current.data().option = ParseOption::AND();
        

        assert_eq!(tree.root().unwrap().data().option, ParseOption::AND());
        assert_eq!(tree.root().unwrap().first_child().unwrap().data().option, VARIABLE(String::from("2"), false));
    }

    
}
