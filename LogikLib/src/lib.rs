pub mod aussagen;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use slab_tree::Tree;
    use std::collections::HashMap;

    use crate::aussagen::{parseFunktion, get_wahrheitstabelle};
    use crate::aussagen::structures::AussagenFunktion::{*, self};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        assert!(
            VARIABEL(String::from("A")).result(&HashMap::from([(String::from("A"), true)]), false)
        );
        assert!(TOP().result(&HashMap::new(), false));
        assert!(!BOTTOM().result(&HashMap::new(), false));

        assert!(AND(Box::new(TOP()), Box::new(TOP())).result(&HashMap::new(), false));
        assert!(OR(Box::new(TOP()), Box::new(TOP())).result(&HashMap::new(), false));
        assert!(NOT(Box::new(BOTTOM())).result(&HashMap::new(), false));

        assert!(AND(
            Box::new(OR(Box::new(BOTTOM()), Box::new(NOT(Box::new(BOTTOM()))))),
            Box::new(TOP())
        )
        .result(&HashMap::new(), false));

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
        let funktion = parseFunktion(&String::from(formel));
        assert_eq!(funktion.to_utf_string(), formel);
    }

    fn test_parse_ascii(formel: &str) {
        let funktion = parseFunktion(&String::from(formel));
        assert_eq!(funktion.to_ascii_string(), formel);
    }

    #[test]
    fn wahrheitstabelle() {
        let funktion:AussagenFunktion = *parseFunktion(&String::from("(A & ( B | C))")).to_owned();
        let tabelle = get_wahrheitstabelle(vec![funktion]);
        println!("{:?}", tabelle);
        println!("{}", tabelle);
    }

    
}
