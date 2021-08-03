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
use std::fs::{File, create_dir_all};

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct JSONChapter {
    pub path: String,
    pub md: String,
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
    generate_html_json(context)
}

fn generate_html_json(context: &RenderContext) -> Result<(), Error> {
    let filename_regex = Regex::new(r"md$")?;

    let chapters = context.book.sections.iter().filter_map(|item| {
        if let BookItem::Chapter(ref chapter) = *item {
            Some(copy_chapter(chapter, &context.destination, &filename_regex))
        } else {
            None
        }
    }).collect::<Vec<JSONChapter>>();


    let title = &context.config.book.title.to_owned().unwrap_or(String::from(""));

    write_book(title.to_string(), chapters, &context.destination, "book.json")?;
    Ok(())
}

fn write_book(title: String, chapters: Vec<JSONChapter>, path: &PathBuf, filename: &str) -> Result<(), Error> {
    let mut book_api = File::create(path.join(filename))?;

    let book = JSONBook {
        title,
        chapters,
    };

    let content = serde_json::to_string(&book)?;
    writeln!(book_api, "{}", content)?;
    Ok(())
}

fn copy_chapter(chapter: &Chapter, prefix: &PathBuf, filename_regex: &Regex) -> JSONChapter {
    let chapter_path = chapter.path.as_ref().expect("chapter path not valid").to_str().expect("Chapter path not valid");

    let path = String::from("/") + &filename_regex.replace_all(chapter_path, "html");

    let md_path = String::from("/api/markdown/") + chapter_path;

    let md_chapter_path = prefix.join("markdown").join(chapter_path);

    let parent_directory = md_chapter_path.parent().expect("");
    create_dir_all(parent_directory).expect("Unable to create parent directory");

    let mut chapter_file = File::create(&md_chapter_path).expect(&format!("Unable to create chapter file at {}", md_chapter_path.to_str().expect("")));
    writeln!(chapter_file, "{}", chapter.content).expect("Unable to write chapter contents");

    JSONChapter {
        path,
        md: md_path,
        title: chapter.name.clone(),
        subchapters: chapter.sub_items.iter().filter_map(|item| {
            if let BookItem::Chapter(ref chapter) = *item {
                Some(copy_chapter(chapter, prefix, filename_regex))
            } else {
                None
            }
        }).collect()
    }
}
