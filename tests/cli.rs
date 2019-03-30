use std::process::Command;
use assert_cmd::prelude::*; // Add methods on commands
use std::fs::read_to_string;

use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq)]
struct JSONChapter {
    path: String,
    title: String,
    subchapters: Vec<JSONChapter>
}

impl PartialEq for JSONChapter {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path &&
            self.title == other.title &&
            self.subchapters == other.subchapters
    }
}

#[test]
fn chapter_json() -> Result<(), Box<std::error::Error>> {
    let expected_chapters_path = Path::new("tests/expected_chapters.json");

    assert_eq!(expected_chapters_path.exists(), true);

    Command::new("mdbook")
        .arg("build")
        .arg("tests/example_book")
        .assert()
        .success();

    let received_chapters: Vec<JSONChapter> = serde_json::from_str(
        &read_to_string("tests/example_book/book/api/chapters.json")
            .expect("failed to read chapters.json")
    ).expect("Failed to parse chapters.json");

    let expected_chapters: Vec<JSONChapter> = serde_json::from_str(
        &read_to_string(expected_chapters_path)
            .expect("failed to read expected_chapters.json")
    ).expect("Failed to parse expected_chapters.json");

    assert_eq!(received_chapters, expected_chapters);

    Ok(())
}