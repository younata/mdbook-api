# mdbook-api

Experimental backend for generating an api for an mdbook.

## Endpoints

- `/api/v1/chapters` is a list of `JSONChapter`s. A `JSONChapter` is defined as: `{path: String, title: String, subchapters: [JSONChapter]}`.
