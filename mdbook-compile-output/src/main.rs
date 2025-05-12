//! This is a demonstration of an mdBook preprocessor which parses markdown
//! and replaces compile placeholders with custom output.

use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use std::io;
use std::process::Command;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // This preprocessor supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    if let Err(e) = handle_preprocessing() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

pub struct CompileOutputPreprocessor;

impl Preprocessor for CompileOutputPreprocessor {
    fn name(&self) -> &str {
        "compile-output-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ch) = item {
                if ch.is_draft_chapter() {
                    return;
                }
                // Process the chapter content to replace compile placeholders
                ch.content = process_compile(&ch.content);
            }
        });
        Ok(book)
    }
}

fn process_compile(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    for line in content.lines() {
        if let Some(step) = extract_step_name(line) {
            // Call the user-implemented compile function
            result.push_str(&compile(&step));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

fn extract_step_name(line: &str) -> Option<String> {
    let prefix = "{{#compile_output:";
    if line.trim_start().starts_with(prefix) {
        // Strip prefix and suffix
        let after = line.trim_start().strip_prefix(prefix)?;
        let step = after.strip_suffix("}}")?.trim();
        Some(step.to_string())
    } else {
        None
    }
}

/// User-implemented compile function stub. Replace with desired logic.
fn compile(step: &str) -> String {
    let path = format!("rust_stages/{}", step.trim());  // Use the step name

    // Run the `cargo test` command in the given directory
    let output = Command::new("cargo")
        .arg("test").arg("--release")
        .current_dir(path)
        .output()
        .expect("Failed to execute cargo test");

    // Get the standard output and error output as strings
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Format the output into a Markdown code block
    if output.status.success() {
        // If the test passed, wrap the standard output in a code block
        format!(
            "```text\n{}\n```",
            stdout // Include only the standard output
        )
    } else {
        // If the test failed, wrap the error output in a code block
        format!(
            "```text\n{}\n```",
            stderr // Include only the error output
        )
    }
}

pub fn handle_preprocessing() -> Result<(), Error> {
    let pre = CompileOutputPreprocessor;
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    let processed = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}
