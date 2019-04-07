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
fn book_json() {
    let (ctx, _md, temp) = create_dummy_book().unwrap();
    mdbook_api::generate(&ctx).unwrap();

    let received_chapters_path = temp.path().join("book.json");
    assert_eq!(received_chapters_path.exists(), true);
    let expected_chapters_path = Path::new("tests/expected_book.json");
    assert_eq!(expected_chapters_path.exists(), true);

    let received_book: mdbook_api::JSONBook = serde_json::from_str(
        &read_to_string(received_chapters_path)
            .expect("failed to read book.json")
    ).expect("Failed to parse book.json");

    let expected_book: mdbook_api::JSONBook = serde_json::from_str(
        &read_to_string(expected_chapters_path)
            .expect("failed to read expected_book.json")
    ).expect("Failed to parse expected_book.json");

    assert_eq!(received_book, expected_book);
}