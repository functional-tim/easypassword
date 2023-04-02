/*
 * main.rs - GUI program to create the password.
 *
 * (C) 2020 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
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
    let app = Application::builder().application_id("org.example.HelloWorld").build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
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

