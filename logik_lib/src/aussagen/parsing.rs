use slab_tree::{NodeId, Tree, TreeBuilder};
use crate::aussagen;
use crate::aussagen::parsing::ParseError::{CurrentIsNotRoot, NoCurrent, NoParent, NoRoot, NoVariableToClose, ToNone, VariableAlreadyClosed};
use crate::aussagen::parsing::ParseOption::{AND, NOTHING, OR, UNSPECIFIED, VARIABLE};
use crate::aussagen::structures::AussagenFunktion;

#[derive(Debug)]
pub struct Parsed {
    pub option: ParseOption,
}

#[derive(Debug, PartialEq)]
pub enum ParseOption {
    UNSPECIFIED(),
    NOTHING(),
    VARIABLE(String, bool),
    TOP(),
    BOTTOM(),
    NOT(),
    AND(),
    OR(),
}

impl From<ParseError> for String {
    fn from(e: ParseError) -> Self {
        format!("{:?}",e)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ToNone,
    NoCurrent,
    NoParent,
    VariableAlreadyClosed,
    NoVariableToClose,
    NoRoot,
    CurrentIsNotRoot,
}

pub fn parse_function(eingabe: &str) -> Result<Box<AussagenFunktion>, ParseError> {
    let parent_funktion = Parsed {
        option: UNSPECIFIED(),
    };
    let mut tree: Tree<Parsed> = TreeBuilder::new().with_root(parent_funktion).build();

    let mut current_node_id = tree.root_id().unwrap();
    for x in eingabe.chars() {
        //Close and move up at end of Var
        match x {
            '|' | '⋁' | '&' | '⋀' | '(' | ')' | 't' | '⊤' | '-' | '¬' | ' ' => {
                if is_unclosed_variable(&mut tree, current_node_id)? {
                    close_var(&mut tree, current_node_id)?;
                    current_node_id = move_up(&mut tree, current_node_id)?;
                }
            }
            _ => {}
        }

        match x {
            '|' | '⋁' => {
                set_option(&mut tree, current_node_id, OR())?;
            }
            '&' | '⋀' => {
                set_option(&mut tree, current_node_id, AND())?;
            }
            ')' => {
                if !has_parent(&mut tree, current_node_id)? {
                    current_node_id = add_unspecified_root_and_move_up(&mut tree, current_node_id)?;
                } else {
                    current_node_id = move_up(&mut tree, current_node_id)?;
                }
            }
            '(' => {
                if !is_unspecified(&mut tree, current_node_id)? {
                    current_node_id = append_unspecified_and_move_down(&mut tree, current_node_id)?;
                }
                current_node_id = append_unspecified_and_move_down(&mut tree, current_node_id)?;
            }
            't' | '⊤' => {
                current_node_id =
                    set_or_append_option(&mut tree, current_node_id, ParseOption::TOP())?;
                if has_parent(&mut tree, current_node_id)? {
                    current_node_id = move_up(&mut tree, current_node_id)?;
                }
            }
            'f' | '⊥' => {
                current_node_id =
                    set_or_append_option(&mut tree, current_node_id, ParseOption::BOTTOM())?;
                if has_parent(&mut tree, current_node_id)? {
                    current_node_id = move_up(&mut tree, current_node_id)?;
                }
            }
            '-' | '¬' => {
                current_node_id =
                    set_or_append_option(&mut tree, current_node_id, ParseOption::NOT())?;
                current_node_id = append_unspecified_and_move_down(&mut tree, current_node_id)?;
            }
            ' ' => {}
            _ => match &tree.get_mut(current_node_id).unwrap().data().option {
                VARIABLE(name, _) => {
                    let mut neu_name = name.clone();
                    neu_name.push(x);
                    tree.get_mut(current_node_id).unwrap().data().option =
                        VARIABLE(neu_name, false);
                }
                UNSPECIFIED() => {
                    set_option(&mut tree, current_node_id, VARIABLE(String::from(x), false))?;
                }
                _ => {
                    current_node_id = append_and_move_down(
                        &mut tree,
                        current_node_id,
                        VARIABLE(String::from(x), false),
                    )?;
                }
            },
        }
        // print_tree(&tree, None);
        //println!(
        //  "after {} => {:?}",
        //x,
        //tree.get(current_node_id).unwrap().data().option
        //);
    }

    //print_tree(&tree, None);
    let stru = to_structures(&tree, tree.root_id().unwrap());
    if stru.is_none() {
        Err(ToNone)
    } else {
        Ok(stru.unwrap())
    }
}

fn append_unspecified_and_move_down(
    tree: &mut Tree<Parsed>,
    current_node_id: NodeId,
) -> Result<NodeId, ParseError> {
    append_and_move_down(tree, current_node_id, UNSPECIFIED())
}

fn append_and_move_down(
    tree: &mut Tree<Parsed>,
    current_node_id: NodeId,
    option: ParseOption,
) -> Result<NodeId, ParseError> {
    let current = tree.get_mut(current_node_id);
    if current.is_none() {
        return Err(NoCurrent);
    }
    let mut current = current.unwrap();
    Ok(current.append(Parsed { option: option }).node_id())
}

fn is_unspecified(tree: &mut Tree<Parsed>, current_node_id: NodeId) -> Result<bool, ParseError> {
    let current = tree.get(current_node_id);
    if current.is_none() {
        return Err(NoCurrent);
    }
    let current = current.unwrap();
    Ok(current.data().option == UNSPECIFIED())
}

fn is_unclosed_variable(
    tree: &mut Tree<Parsed>,
    current_node_id: NodeId,
) -> Result<bool, ParseError> {
    let current = tree.get(current_node_id);
    if current.is_none() {
        return Err(NoCurrent);
    }
    let current = current.unwrap();
    match current.data().option {
        VARIABLE(_, is_closed) => Ok(!is_closed),
        _ => Ok(false),
    }
}

fn close_var(tree: &mut Tree<Parsed>, current_node_id: NodeId) -> Result<(), ParseError> {
    let current = tree.get_mut(current_node_id);
    if current.is_none() {
        return Err(NoCurrent);
    }
    let mut current = current.unwrap();
    match &current.data().option {
        VARIABLE(name, is_closed) => {
            if *is_closed {
                return Err(VariableAlreadyClosed);
            }
            current.data().option = VARIABLE(name.clone(), true);
            Ok(())
        }
        _ => Err(NoVariableToClose),
    }
}

fn set_or_append_option(
    tree: &mut Tree<Parsed>,
    current_node_id: NodeId,
    option: ParseOption,
) -> Result<NodeId, ParseError> {
    let current = tree.get_mut(current_node_id);
    if current.is_none() {
        return Err(NoCurrent);
    }
    let mut current = current.unwrap();
    match current.data().option {
        UNSPECIFIED() => {
            set_option(tree, current_node_id, option)?;
            Ok(current_node_id)
        }
        _ => append_and_move_down(tree, current_node_id, option),
    }
}

fn set_option(
    tree: &mut Tree<Parsed>,
    current_node_id: NodeId,
    option: ParseOption,
) -> Result<(), ParseError> {
    let current = tree.get_mut(current_node_id);
    if current.is_none() {
        return Err(NoCurrent);
    }
    let mut current = current.unwrap();
    current.data().option = option;
    Ok(())
}

fn has_parent(tree: &mut Tree<Parsed>, current_node_id: NodeId) -> Result<bool, ParseError> {
    let current = tree.get(current_node_id);
    if current.is_none() {
        return Err(NoCurrent);
    }
    let current = current.unwrap();
    Ok(current.parent().is_some())
}

fn add_unspecified_root_and_move_up(
    tree: &mut Tree<Parsed>,
    current_node_id: NodeId,
) -> Result<NodeId, ParseError> {
    let root_id = tree.root_id();
    if root_id.is_none() {
        return Err(NoRoot);
    }
    let root_id = root_id.unwrap();
    if current_node_id != root_id {
        return Err(CurrentIsNotRoot);
    }

    let new_root = tree.set_root(Parsed {
        option: UNSPECIFIED(),
    });
    Ok(new_root)
}

mod test {
    use slab_tree::Tree;

    use crate::aussagen::UNSPECIFIED;
    use crate::aussagen::parsing::{append_unspecified_and_move_down, is_unspecified, move_up, Parsed};
    use crate::aussagen::parsing::ParseOption::VARIABLE;

    #[test]
    fn move_up_test() {
        //Given
        let mut tree: Tree<Parsed> = Tree::new();
        let root_id = tree.set_root(Parsed {
            option: VARIABLE(String::from("ROOT"), false),
        });
        let current_node_id = tree
            .get_mut(root_id)
            .unwrap()
            .append(Parsed {
                option: VARIABLE(String::from("Var"), false),
            })
            .node_id();

        //When
        let ret = move_up(&mut tree, current_node_id).expect("Fehler bei move up");

        //Then
        assert_eq!(root_id, ret);
    }

    #[test]
    fn append_unspecified_and_move_down_test() {
        //Given
        let mut tree: Tree<Parsed> = Tree::new();
        let root_id = tree.set_root(Parsed {
            option: VARIABLE(String::from("ROOT"), false),
        });
        let current_node_id = tree
            .get_mut(root_id)
            .unwrap()
            .append(Parsed {
                option: VARIABLE(String::from("Var"), false),
            })
            .node_id();

        //When
        let ret = append_unspecified_and_move_down(&mut tree, current_node_id)
            .expect("Fehler bei append und move down");

        //Then
        assert_ne!(root_id, ret);
        assert_ne!(current_node_id, ret);
        let ret_node = tree.get(ret).expect("Keine Node");
        assert_eq!(ret_node.data().option, UNSPECIFIED());
        assert_eq!(
            ret_node.parent().expect("Kein Parent").data().option,
            VARIABLE(String::from("Var"), false)
        );
    }

    #[test]
    fn is_unspecified_test() {
        //Given
        let mut tree: Tree<Parsed> = Tree::new();
        let root_id = tree.set_root(Parsed {
            option: VARIABLE(String::from("ROOT"), false),
        });
        let var_id = tree
            .get_mut(root_id)
            .unwrap()
            .append(Parsed {
                option: VARIABLE(String::from("Var"), false),
            })
            .node_id();
        let current_node_id = tree
            .get_mut(root_id)
            .unwrap()
            .append(Parsed {
                option: UNSPECIFIED(),
            })
            .node_id();

        //When
        assert!(is_unspecified(&mut tree, current_node_id).expect("Fehler unspecified"));
        assert!(!is_unspecified(&mut tree, var_id).expect("Fehler unspecified 2"));
    }
}

fn to_structures(slab_tree: &Tree<Parsed>, node_id: NodeId) -> Option<Box<AussagenFunktion>> {
    match &slab_tree.get(node_id).unwrap().data().option {
        VARIABLE(name, _) => Some(Box::new(AussagenFunktion::VARIABEL(name.clone()))),
        ParseOption::TOP() => Some(Box::new(AussagenFunktion::TOP())),
        ParseOption::BOTTOM() => Some(Box::new(AussagenFunktion::BOTTOM())),
        ParseOption::NOT() => {
            let children_id = slab_tree
                .get(node_id)
                .unwrap()
                .children()
                .next()
                .unwrap()
                .node_id();
            let child = to_structures(slab_tree, children_id);
            if child.is_none() {
                None
            } else {
                Some(Box::new(AussagenFunktion::NOT(child.unwrap())))
            }
        }
        AND() => {
            let siblings = slab_tree.get(node_id).unwrap().children();
            let mut vector = vec![];
            for ele in siblings {
                let option = to_structures(slab_tree, ele.node_id());
                if option.is_some() {
                    vector.push(option.unwrap());
                }
            }

            Some(Box::new(AussagenFunktion::AND(vector)))
        }
        OR() => {
            let siblings = slab_tree.get(node_id).unwrap().children();
            let mut vector = vec![];
            for ele in siblings {
                let option = to_structures(slab_tree, ele.node_id());
                if option.is_some() {
                    vector.push(option.unwrap());
                }
            }

            Some(Box::new(AussagenFunktion::OR(vector)))
        }
        NOTHING() | UNSPECIFIED() => {
            if slab_tree.get(node_id).unwrap().children().next().is_none() {
                return None;
            }
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

fn print_tree(slab_tree: &Tree<Parsed>, node_id_option: Option<NodeId>) {
    match node_id_option {
        None => {
            println!();
            let id = slab_tree.root_id();
            print_tree(slab_tree, id);
        }
        Some(node_id) => {
            let node_ref = slab_tree.get(node_id).unwrap();
            println!("{:?} -> ", slab_tree.get(node_id).unwrap().data().option);
            for node in node_ref.children() {
                println!("{:?}", node.data().option);
            }

            println!("-----------");
            for node in node_ref.children() {
                print_tree(slab_tree, Some(node.node_id()))
            }
        }
    }
}

fn move_up(tree: &mut Tree<Parsed>, current_node_id: NodeId) -> Result<NodeId, ParseError> {
    let current = tree.get(current_node_id);
    if current.is_none() {
        return Err(NoCurrent);
    }
    let current = current.unwrap();
    let parent = current.parent();
    if parent.is_none() {
        return Err(NoParent);
    }
    let parent = parent.unwrap();
    //println!("parent {:?}", parent.data().option);

    match parent.data().option {
        ParseOption::NOT() => {
            let parent_id = parent.node_id();
            match move_up(tree, parent_id.clone()) {
                Ok(node_id) => Ok(node_id),
                Err(_) => Ok(parent_id),
            }
        }
        _ => Ok(parent.node_id()),
    }
}
