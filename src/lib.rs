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

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct JSONBook {
    pub title: String,
    pub chapters: Vec<JSONChapter>
}

impl PartialEq for JSONBook {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.chapters == other.chapters
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


    let title = &context.config.book.title.to_owned().unwrap_or(String::from(""));

    write_book(title.to_string(), chapters, &context.destination)?;
    Ok(())
}

fn write_book(title: String, chapters: Vec<JSONChapter>, path: &PathBuf) -> Result<(), Error> {
    let mut book_api = File::create(path.join("book.json"))?;

    let book = JSONBook {
        title,
        chapters,
    };

    let content = serde_json::to_string(&book)?;
    writeln!(book_api, "{}", content)?;
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