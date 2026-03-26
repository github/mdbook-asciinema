use mdbook_asciinema::PlaceholderPreprocessor;
use mdbook_driver::{MDBook, builtin_renderers::MarkdownRenderer};
use pathbuf::pathbuf;
use regex::Regex;
use soup::prelude::*;

macro_rules! resource_path {
    ($path:expr) => {
        pathbuf![env!("CARGO_MANIFEST_DIR"), "tests/resources", $path]
    };
}

#[test]
fn replace_placeholder() {
    let mut mdbook = MDBook::load(resource_path!("basic")).expect("failed to load book");

    let preprocessor = PlaceholderPreprocessor::new();
    mdbook.with_preprocessor(preprocessor);

    let renderer = MarkdownRenderer::new();
    let (book, _) = mdbook
        .preprocess_book(&renderer)
        .expect("failed to preprocess book");

    let chapter = book.chapters().next().expect("chapter is absent");

    let soup = Soup::new(&chapter.content);

    soup.tag("div")
        .attr_name(Regex::new(r#"b-[A-Za-z0-9]{10}"#).unwrap())
        .find()
        .expect("asciinema div element is absent");
}
