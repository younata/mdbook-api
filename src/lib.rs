extern crate mdbook;
extern crate regex;
extern crate failure;

use failure::Error;
use mdbook::renderer::RenderContext;
use mdbook::book::Chapter;
use mdbook::BookItem;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct JSONChapter {
    pub path: String,
    pub title: String,
    pub subchapters: Vec<JSONChapter>
}


impl PartialEq for JSONChapter {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path &&
            self.title == other.title &&
            self.subchapters == other.subchapters
    }
}

pub fn generate(context: &RenderContext) -> Result<(), Error> {
    let filename_regex = Regex::new(r"md$")?;

    let chapters = context.book.sections.iter().filter_map(|item| {
        if let BookItem::Chapter(ref chapter) = *item {
            Some(parse_chapter(chapter, &filename_regex))
        } else {
            None
        }
    }).collect::<Vec<JSONChapter>>();

    write_chapters(chapters, &context.destination)?;
    Ok(())
}

fn write_chapters(chapters: Vec<JSONChapter>, path: &PathBuf) -> Result<(), Error> {
    let mut chapter_api = File::create(path.join("chapters.json"))?;

    let content = serde_json::to_string(&chapters)?;
    writeln!(chapter_api, "{}", content)?;
    Ok(())
}

fn parse_chapter(chapter: &Chapter, filename_regex: &Regex) -> JSONChapter {
    let chapter_path = chapter.path.to_str().expect("Chapter path not valid");
    let path = String::from("/") + &filename_regex.replace_all(chapter_path, "html");
    JSONChapter {
        path,
        title: chapter.name.clone(),
        subchapters: chapter.sub_items.iter().filter_map(|item| {
            if let BookItem::Chapter(ref chapter) = *item {
                Some(parse_chapter(chapter, filename_regex))
            } else {
                None
            }
        }).collect()
    }
}