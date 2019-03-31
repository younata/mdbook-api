extern crate failure;
extern crate mdbook;
extern crate mdbook_api;
extern crate tempdir;

use failure::{Error, SyncFailure};
use mdbook::MDBook;
use mdbook::renderer::RenderContext;
use tempdir::TempDir;
use serde::{Serialize, Deserialize};
use std::fs::read_to_string;
use std::path::Path;

fn create_dummy_book() -> Result<(RenderContext, MDBook, TempDir), Error> {
    let temp = TempDir::new("mdbook-api")?;

    let dummy_book = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("example_book");

    let md = MDBook::load(dummy_book).map_err(SyncFailure::new)?;

    let ctx = RenderContext::new(
        md.root.clone(),
        md.book.clone(),
        md.config.clone(),
        temp.path().to_path_buf(),
    );

    Ok((ctx, md, temp))
}

#[test]
fn chapter_json() {
    let (ctx, _md, temp) = create_dummy_book().unwrap();
    mdbook_api::generate(&ctx).unwrap();

    let received_chapters_path = temp.path().join("chapters.json");
    assert_eq!(received_chapters_path.exists(), true);
    let expected_chapters_path = Path::new("tests/expected_chapters.json");
    assert_eq!(expected_chapters_path.exists(), true);

    let received_chapters: Vec<mdbook_api::JSONChapter> = serde_json::from_str(
        &read_to_string(received_chapters_path)
            .expect("failed to read chapters.json")
    ).expect("Failed to parse chapters.json");

    let expected_chapters: Vec<mdbook_api::JSONChapter> = serde_json::from_str(
        &read_to_string(expected_chapters_path)
            .expect("failed to read expected_chapters.json")
    ).expect("Failed to parse expected_chapters.json");

    assert_eq!(received_chapters, expected_chapters);
}