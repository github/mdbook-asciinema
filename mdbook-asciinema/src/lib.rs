mod types;

use crate::types::Asset;
use anyhow::anyhow;
use mdbook_preprocessor::book::{Book, Chapter};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use rand::distr::Alphanumeric;
use rand::prelude::*;
use regex::{CaptureMatches, Captures};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use tracing::{error, info, warn};

const ESCAPE_CHAR: char = '\\';

const ASSET_CFG: &[(&str, &str)] = &[
    ("asciinema-player.min.js", "lib/asciinema-player"),
    ("asciinema-player.css", "lib/asciinema-player"),
];

trait TransformMarkdown {
    fn transform(&mut self, ctx: &PreprocessorContext);
}

/// A preprocessor for expanding helpers in a chapter. Supported helpers are:
///
/// - `{{ #asciinema }}` - Reference an external asciinema cast file.
#[derive(Default)]
#[non_exhaustive]
pub struct PlaceholderPreprocessor;

impl PlaceholderPreprocessor {
    /// Name of this preprocessor.
    pub const NAME: &'static str = "placeholder";

    /// Create a new `PlaceholderPreprocessor`.
    pub fn new() -> Self {
        PlaceholderPreprocessor
    }
}

impl Preprocessor for PlaceholderPreprocessor {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        create_assets(ctx).map_err(|_| anyhow!("failed to create assets"))?;

        book.for_each_chapter_mut(|chapter| chapter.transform(ctx));

        Ok(book)
    }
}

fn create_assets(ctx: &PreprocessorContext) -> Result<()> {
    let src_dir = ctx.root.join(&ctx.config.book.src);

    // create embedded assets
    for &(asset_filename, dest_dir_str) in ASSET_CFG {
        let dest_dir = src_dir.join(dest_dir_str);

        if !dest_dir.exists() {
            fs::create_dir_all(&dest_dir).map_err(|_| anyhow!("failed to create {dest_dir:?}"))?;
        }

        if !dest_dir.is_dir() {
            return Err(anyhow!("{dest_dir:?} is not a directory"));
        }

        let asset_path = dest_dir.join(asset_filename);

        let asset_contents = Asset::get(asset_filename)
            .ok_or_else(|| anyhow!("missing {asset_filename}"))?
            .data;

        if asset_path.exists() {
            let existing = fs::read(&asset_path).unwrap_or_default();
            if existing == asset_contents.as_ref() {
                continue;
            }
        }

        fs::write(&asset_path, asset_contents)
            .map(|_| anyhow!("failed to create {asset_path:?}"))?;

        info!("created {asset_path:?}");
    }

    Ok(())
}

impl TransformMarkdown for Chapter {
    fn transform(&mut self, ctx: &PreprocessorContext) {
        let src_dir = ctx.root.join(&ctx.config.book.src);

        let mut previous_end_index = 0;
        let mut replaced = String::new();

        replaced.push_str(
            r#"<link rel="stylesheet" href="lib/asciinema-player/asciinema-player.css">"#,
        );
        replaced.push_str("\n\n");
        replaced
            .push_str(r#"<script src="lib/asciinema-player/asciinema-player.min.js"></script>"#);
        replaced.push_str("\n\n");

        for placeholder in find_placeholders(&self.content) {
            replaced.push_str(&self.content[previous_end_index..placeholder.start_index]);

            match placeholder.render(&src_dir) {
                Ok(new_content) => {
                    replaced.push_str(&new_content);
                    previous_end_index = placeholder.end_index;
                }
                Err(e) => {
                    error!("Error updating \"{}\", {}", placeholder.placeholder_text, e);

                    for cause in e.chain().skip(1) {
                        warn!("Caused By: {}", cause);
                    }

                    previous_end_index = placeholder.start_index;
                }
            }
        }

        replaced.push_str(&self.content[previous_end_index..]);
        self.content = replaced
    }
}

#[derive(PartialEq, Debug, Clone)]
enum PlaceholderType {
    Asciinema(String, HashMap<String, String>),
    Escaped,
}

#[derive(PartialEq, Debug, Clone)]
struct Placeholder {
    start_index: usize,
    end_index: usize,
    placeholder_type: PlaceholderType,
    placeholder_text: String,
}

impl Placeholder {
    fn from_capture(cap: Captures) -> Option<Self> {
        let placeholder_type = match (cap.get(0), cap.get(1), cap.get(2)) {
            (_, Some(typ), Some(rest)) => {
                let mut args = rest.as_str().split_whitespace();

                let uri_arg = args.next();

                let props = args
                    .filter_map(|path| path.split_once('='))
                    .map(|pairs| (pairs.0.to_string(), pairs.1.to_string()))
                    .collect::<HashMap<String, String>>();

                match (typ.as_str(), uri_arg) {
                    ("asciinema", Some(uri)) => Some(PlaceholderType::Asciinema(uri.into(), props)),
                    _ => None,
                }
            }
            (Some(mat), None, None) if mat.as_str().starts_with(ESCAPE_CHAR) => {
                Some(PlaceholderType::Escaped)
            }
            _ => None,
        };

        placeholder_type.and_then(|placeholder_type| {
            cap.get(0).map(|mat| Self {
                start_index: mat.start(),
                end_index: mat.end(),
                placeholder_type,
                placeholder_text: mat.as_str().to_string(),
            })
        })
    }

    fn render(&self, from_path: &Path) -> Result<String> {
        static SCOPE: LazyLock<regex::Regex> =
            LazyLock::new(|| regex::Regex::new(r#"^[A-Za-z0-9]{5,10}$"#).unwrap());

        match self.placeholder_type {
            // omit the escape char
            PlaceholderType::Escaped => Ok(self.placeholder_text[1..].to_owned()),
            PlaceholderType::Asciinema(ref uri, ref props) => {
                let mut html = String::new();

                // references:
                //   - https://learn.microsoft.com/en-us/aspnet/core/blazor/components/css-isolation?view=aspnetcore-10.0#css-isolation-bundling
                //   - https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector

                let maybe_scope = props.get("scope");

                let scope = match maybe_scope {
                    Some(value) if SCOPE.is_match(&value) => value.clone(),
                    _ => {
                        rand::rng()
                            .sample_iter(&Alphanumeric)
                            .take(10)
                            .map(char::from)
                            .collect::<String>()
                    }
                };

                let css_scope = format!("b-{scope}");

                html.push_str(format!(r#"<div {css_scope}></div>"#).as_str());
                html.push_str(
                    format!(
                        r#"<script {css_scope}>const player_{scope} = AsciinemaPlayer.create('{uri}', document.querySelector('div[{css_scope}]')"#
                    )
                    .as_str(),
                );

                // Opts is the 3rd argument to AsciinemaPlayer.create()
                if let Some(opts_path_txt) = props.get("opts") {
                    let opts_path = PathBuf::from(opts_path_txt);

                    if !opts_path.is_relative() {
                        return Err(anyhow!(
                            "opts must be a relative path, from the 'src' directory"
                        ));
                    }

                    let opts_full_path = from_path.join(opts_path);

                    let opts_txt = fs::read_to_string(&opts_full_path)
                        .map_err(|_| anyhow!("{opts_full_path:?} is not readable"))?;

                    let opts = serde_json::from_str::<Value>(opts_txt.as_str())
                        .map_err(|_| anyhow!("{opts_full_path:?} is not a valid json file"))?;

                    let opts = opts.as_object().ok_or_else(|| {
                        anyhow!("{opts_full_path:?} should contain a single json object")
                    })?;

                    let opts_json = serde_json::to_string(opts)
                        .map_err(|_| anyhow!("opts cannot be serialized as json"))?;

                    html.push_str(format!(", {opts_json}").as_str());
                }

                html.push_str(");</script>");

                Ok(html.to_string())
            }
        }
    }
}

struct PlaceholderIter<'a>(CaptureMatches<'a, 'a>);

impl<'a> Iterator for PlaceholderIter<'a> {
    type Item = Placeholder;
    fn next(&mut self) -> Option<Self::Item> {
        for cap in &mut self.0 {
            if let Some(inc) = Self::Item::from_capture(cap) {
                return Some(inc);
            }
        }
        None
    }
}

fn find_placeholders(contents: &str) -> PlaceholderIter<'_> {
    static PLACEHOLDER: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(
            r"(?x)              # insignificant whitespace mode
            \\\{\{\#.*\}\}      # match escaped link
            |                   # or
            \{\{\s*             # link opening parens and whitespace
            \#([a-zA-Z0-9_]+)   # link type
            \s+                 # separating whitespace
            ([^}]+)             # link target path and space separated properties
            \}\}                # link closing parens",
        )
        .unwrap()
    });

    PlaceholderIter(PLACEHOLDER.captures_iter(contents))
}
