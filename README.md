# LogikAnalyse

# how to build:
1. Rust und Cargo installieren: https://www.rust-lang.org/tools/install
2. npm installieren: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
3. tauri installieren: cargo install tauri-cli
4. npm packete installieren: (in ./logik_analyse_gui) npm install
5. Dev Build: (in ./logik_analyse_gui) cargo tauri dev
   Release Build: (in ./logik_analyse_gui) cargo tauri build 
   
Ansonsten siehe: https://tauri.app/v1/guides/getting-started/prerequisites

# Aufbau
LogikLib ist die Rust Libary die die Buiness Logik beinhaltet.

logik_analyse_gui ist das GUI. Es besteht aus einem NextJS Project und einem Rust Project in src-tauri.
Das Rust Projekt ist das Backend des GUI. Das Backend verwendet LogikLib.
