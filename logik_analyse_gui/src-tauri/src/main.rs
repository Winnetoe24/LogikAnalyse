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
use serde_json::Map;
use tauri::utils::resources::ResourcePaths;
use LogikLib::aussagen::{self, structures::FormelKontext};
use LogikLib::aussagen::{
    parseFunktion,
    structures::{AussagenFunktion, Wahrheitstabelle},
};
struct MyState {
    kontext: FormelKontext
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
    let mut state = MyState { kontext: FormelKontext::new() };
    tauri::Builder::default()
        .manage(Mutex::from(state))
        .invoke_handler(tauri::generate_handler![
            greet,
            renderFormel,
            get_wahrheitstabelle_cmd,
            getFormel,
            check_formel
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
    let funktion = parseFunktion(&String::from(input));
    let utf = funktion.to_utf_string();
    Ok(utf)
}
#[tauri::command]
async fn renderFormel(
    mut state: tauri::State<'_, Mutex<MyState>>,
    name: &str,
    input: &str,
) -> Result<String, String> {
    let funktion = parseFunktion(&String::from(input));
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

            Ok(format!("{}", aussagen::get_wahrheitstabelle(&state.kontext, formeln)))
        }
        Err(e) => {
            let r = e.get_ref();
            drop(r);
            return Err(e.to_string());
        }
    }
}

