use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use tempfile::NamedTempFile;

use crate::request::{NoteSource, Request};
use crate::utils;

pub fn create_note(write_date: bool, note_body: &String) -> String {
    if write_date {
        return format!("{}\n{}\n\n", utils::get_date_time_string(), note_body);
    } else {
        return format!("{}\n\n", note_body);
    }
}

pub fn write_note(path: &PathBuf, note: &String) {
    utils::create_file(path);

    let mut file = OpenOptions::new().append(true).open(path).unwrap();

    if let Err(_) = file.write_all(note.as_bytes()) {
        eprintln!("Could not write to the file. Exiting...");
        process::exit(1);
    }
}

pub fn get_note_body(request: &Request, template_file_path: &PathBuf) -> String {
    let note_body;

    if let Some(v) = &request.note_body {
        note_body = v.to_string();
    } else if let NoteSource::Editor = request.note_source {
        note_body = get_file_note(request, template_file_path);
    } else {
        note_body = get_stdin_note();
    }

    return note_body.trim().to_owned();
}

fn get_file_note(request: &Request, template_file_path: &PathBuf) -> String {
    let mut file = NamedTempFile::new().unwrap();

    if request.use_template {
        let template_contents = fs::read_to_string(template_file_path).unwrap();
        file.write_all(template_contents.as_bytes()).unwrap();
    }

    let temp_path = file.into_temp_path();
    let status = utils::open_editor(
        &request.editor_name.as_ref().unwrap(),
        &temp_path.to_path_buf(),
    );

    if status.success() {
        let buffer = fs::read_to_string(&temp_path).unwrap();
        return buffer;
    } else {
        eprintln!("Child process failed. Exiting...");
        process::exit(1);
    }
}

fn get_stdin_note() -> String {
    let mut stdout = io::stdout();
    write!(&mut stdout, "Enter note: ").unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input;
}
