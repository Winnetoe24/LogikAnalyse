#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{
    collections::HashMap,
    f32::consts::E,
    hash::Hash,
    sync::{Mutex, PoisonError, TryLockError},
};

use lazy_static::lazy_static;
use tauri::utils::resources::ResourcePaths;
use logik_lib::aussagen::*;
use logik_lib::aussagen::parsing::parse_function;
use logik_lib::aussagen::structures::{AussagenFunktion, FormelKontext};

struct MyState {
    kontext: FormelKontext,
}

impl MyState {
    fn insert(&mut self, name: String, funktion: Box<AussagenFunktion>) {
        self.kontext.funktionen.insert(name, *funktion);
    }

    fn get(&self, name: String) -> Option<&AussagenFunktion> {
        self.kontext.funktionen.get(&name)
    }
}

struct Mapping {
    name: String,
    funktion: Box<AussagenFunktion>,
}

fn main() {
    let mut state = MyState {
        kontext: FormelKontext::new(),
    };
    tauri::Builder::default()
        .manage(Mutex::from(state))
        .invoke_handler(tauri::generate_handler![
            greet,
            renderFormel,
            get_wahrheitstabelle_cmd,
            getFormel,
            check_formel,
            is_aequivalent
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hi, {}!", name)
}

#[tauri::command(rename_all = "snake_case")]
fn getFormel(state: tauri::State<'_, Mutex<MyState>>, name: &str, is_utf: bool) -> String {
    match state.lock() {
        Ok(state) => {
            let formel = state.get(String::from(name));
            if formel.is_none() {
                return String::new();
            }
            let formel = formel.unwrap();
            if is_utf {
                formel.to_utf_string()
            } else {
                formel.to_ascii_string()
            }
        }
        Err(_) => String::from(""),
    }
}

#[tauri::command]
async fn check_formel(
    mut state: tauri::State<'_, Mutex<MyState>>,
    input: &str,
) -> Result<String, String> {
    let funktion = parse_function(&String::from(input))?;
    let utf = funktion.to_utf_string();
    Ok(utf)
}
#[tauri::command]
async fn renderFormel(
    mut state: tauri::State<'_, Mutex<MyState>>,
    name: &str,
    input: &str,
) -> Result<String, String> {
    let funktion = parse_function(&String::from(input))?;
    let utf = funktion.to_utf_string();
    match state.lock() {
        Ok(mut state) => {
            state.insert(String::from(name), funktion);
        }
        Err(_) => return Err(String::from("Fehler bei Lock")),
    } // state.map.insert(String::from(name), funktion);
      // FUNKTIONEN.lock().into().insert(String::from(name), funktion);
    println!("{}", utf);

    Ok(utf)
}

#[tauri::command]
async fn get_wahrheitstabelle_cmd(
    mut state: tauri::State<'_, Mutex<MyState>>,
    namen: Vec<String>,
) -> Result<String, String> {
    let mut formeln = Vec::new();

    match state.lock() {
        Ok(state) => {
            for ele in namen {
                let func = state.get(ele);
                if func.is_some() {
                    let func = func.unwrap();
                    formeln.push(func);
                }
            }

            Ok(format!(
                "{}",
                get_wahrheitstabelle(&state.kontext, formeln)
            ))
        }
        Err(e) => {
            let r = e.get_ref();
            drop(r);
            return Err(e.to_string());
        }
    }
}

#[tauri::command]
async fn is_aequivalent(
    mut state: tauri::State<'_, Mutex<MyState>>,
    namen: Vec<String>,
) -> Result<String, String> {
    println!("is_aequivalent");
    let mut formeln = Vec::new();

    match state.lock() {
        Ok(state) => {
            let mut map = HashMap::new();
            for ele in &namen {
                let func = state.get(ele.clone());
                if func.is_some() {
                    let func = func.unwrap();
                    formeln.push(func);
                    for name2 in &namen {
                        let func = state.get(name2.clone());
                        if func.is_some() {
                                if !formeln.contains(&func.unwrap()) {
                                map.insert(format!("{} ??? {}", ele, name2), true);
                            }
                        }
                    }
                }
            }

            let tabelle = get_wahrheitstabelle(&state.kontext, formeln);
            for ele in tabelle.belegungen {
                for tupel in &ele.ergebnisse {
                    for tupel2 in &ele.ergebnisse {
                        if map.contains_key(&format!("{} ??? {}", tupel.0, tupel2.0)) {
                            if (tupel.1 != tupel2.1) {
                                map.insert(format!("{} ??? {}", tupel.0, tupel2.0), false);
                            }
                        }
                    }
                }
            }

            let mut s = String::new();
            for ele in map {
                if s.is_empty() {
                    s = format!("{}", ele.0.replace("???", {
                        if ele.1 {
                            "???"
                        }else {
                            "???"
                        }
                    }));
                } else {
                    s = format!("{}\n{}", s, ele.0.replace("???", {
                        if ele.1 {
                            "="
                        }else {
                            "???"
                        }
                    }));
                }
            }
            Ok(s)
        }
        Err(e) => {
            let r = e.get_ref();
            drop(r);
            return Err(e.to_string());
        }
    }
}
