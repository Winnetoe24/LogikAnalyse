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
use LogikLib::aussagen::{
    parseFunktion,
    structures::{AussagenFunktion, Wahrheitstabelle},
};
use LogikLib::aussagen;
struct MyState {
    vector: Vec<Mapping>,
}

impl MyState {
    fn insert(&mut self, name: String, funktion: Box<AussagenFunktion>) {
        for i in 0..self.vector.len() {
            let ele = self.vector.get(i).unwrap();
            if (ele.name.eq(&name)) {
                self.vector.remove(i);
            }
        }
        self.vector.push(Mapping {
            name: name,
            funktion: funktion,
        });
    }

    fn get(&self, name: String) -> Option<Box<AussagenFunktion>> {
        for ele in &self.vector {
            if (ele.name.eq(&name)) {
                return Some(ele.funktion.clone());
            }
        }
        None
    }
}

struct Mapping {
    name: String,
    funktion: Box<AussagenFunktion>,
}

fn main() {
    let mut state = MyState { vector: Vec::new() };
    tauri::Builder::default()
        .manage(Mutex::from(state))
        .invoke_handler(tauri::generate_handler![greet, renderFormel, get_wahrheitstabelle_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hi, {}!", name)
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
               formeln.push(*func);
            }
           }
           
           Ok(format!("{}",aussagen::get_wahrheitstabelle(formeln)))
        },
        Err(e) => {
          let r = e.get_ref();
          drop(r);
          return Err(e.to_string())}
          ,
    }
    
}
