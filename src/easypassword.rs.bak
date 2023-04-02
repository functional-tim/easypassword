/*
 * main.rs - GUI program to create the password.
 *
 * (C) 2020 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use eframe::egui;
use heck::ToTitleCase;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
//use std::path::{Path, PathBuf};
//use std::process::exit;
//use structopt::StructOpt;

mod password;

// Main program logic.
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "easypassword",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

// Auxiliary function to transform the input file into a Vector of single words.
// Input file has to be formatted in such a way that every word is on a single line.
/*fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>, (String, i32)> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err((String::from("no such file"), exitcode::NOINPUT)),
    };
    let buf = BufReader::new(file);
    match buf.lines().collect() {
        Ok(res) => Ok(res),
        Err(_) => Err((
            String::from("file contained invalid UTF-8"),
            exitcode::DATAERR,
        )),
    }
}*/

// Change the first letter of every word to uppercase.
fn transform(st: &mut Vec<String>) {
    for s in st {
        s.retain(|c| !c.is_whitespace());
        *s = s.to_title_case();
    }
}

// GUI implementations
struct MyApp {
    //wordlistpath: String,
    wordlist: Vec<String>,
    seperator1: String,
    seperator2: String,
    number: usize,
    password: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            /*            wordlistpath: {
                let wordlist: String = "../../12dicts/International/3of6game.txt".to_string();
                wordlist
            },
            wordlist: Vec::new(),*/
            wordlist: {
                let mut wordlist: Vec<String> =
                    include_str!("../12dicts/International/3of6game.txt")
                        .split('\n')
                        .map(|x| x.parse::<String>().unwrap())
                        .collect();
                transform(&mut wordlist);
                wordlist
            },
            seperator1: "%".to_owned(),
            seperator2: "5".to_owned(),
            number: 4,
            password: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            /*ui.horizontal(|ui| {
                ui.label("Wordlist");
                ui.text_edit_singleline(&mut self.wordlistpath);
            });*/
            ui.horizontal(|ui| {
                ui.label("Seperator 1");
                ui.text_edit_singleline(&mut self.seperator1);
            });
            ui.horizontal(|ui| {
                ui.label("Seperator 2");
                ui.text_edit_singleline(&mut self.seperator2);
            });
            ui.add(egui::Slider::new(&mut self.number, 4..=8).text("Number of words"));
            if ui.button("Click to generatre password").clicked() {
                /*self.wordlist = match lines_from_file(&mut self.wordlistpath) {
                    Ok(x) => x,
                    Err((err, _)) => {
                        eprintln!("Error: {}", err);
                        Vec::new()
                    }
                };
                transform(&mut self.wordlist);*/
                self.password = password::create_password(
                    &mut self.wordlist,
                    self.seperator1.to_owned(),
                    self.seperator2.to_owned(),
                    self.number,
                );
            }
            ui.horizontal(|_ui| "");
            ui.heading("Password");
            ui.horizontal(|ui| {
                //ui.label("Password");
                ui.text_edit_singleline(&mut self.password);
            });
        });
    }
}
