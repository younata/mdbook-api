extern crate mdbook;
extern crate regex;

use mdbook::renderer::RenderContext;
use mdbook::book::Chapter;
use mdbook::BookItem;
use regex::Regex;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct JSONChapter {
    path: String,
    title: String,
    subchapters: Vec<JSONChapter>
}

fn main() {
    let mut stdin = io::stdin();
    let context = RenderContext::from_json(&mut stdin).expect("Unable to create RenderContext.");

    let filename_regex = Regex::new(r"md$").expect("Unable to create regex");

    let chapters = context.book.sections.iter().filter_map(|item| {
        if let BookItem::Chapter(ref chapter) = *item {
            Some(parse_chapter(chapter, &filename_regex))
        } else {
            None
        }
    }).collect::<Vec<JSONChapter>>();

    write_chapters(chapters, context.destination);
}

fn write_chapters(chapters: Vec<JSONChapter>, path: PathBuf) {
    let mut chapter_api = File::create(path.join("chapters.json"))
        .expect("Unable to create chapters.json");

    let content = serde_json::to_string(&chapters)
        .expect("Unable to serialize chapters to json");
    writeln!(chapter_api, "{}", content).expect("Unable to write chapter api");
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