use minify_html::{minify, Cfg};
use std::fs::write;

fn main() {
    if std::env::args().len() < 3 {
        eprintln!("Usage: {} <path_to_html_file> <output_path>", std::env::args().next().unwrap());
        return;
    }

    let input_path = std::env::args().nth(1).expect("Expected a path to an HTML file");
    let output_path = std::env::args().nth(2).expect("Expected an output file path");

    let code = match std::fs::read(input_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut cfg = Cfg::new();
    cfg.do_not_minify_doctype = true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    cfg.keep_closing_tags = true;
    cfg.keep_html_and_head_opening_tags = true;
    cfg.keep_spaces_between_attributes = true;
    cfg.keep_comments = false;
    cfg.keep_input_type_text_attr = true;
    cfg.keep_ssi_comments = false;
    // cfg.preserve_brace_template_syntax = false;
    // cfg.preserve_chevron_percent_template_syntax = false;
    cfg.minify_css = true;
    cfg.minify_js = true;
    cfg.remove_bangs = false;
    cfg.remove_processing_instructions = false;

    let minified = minify(&code, &cfg);

    if let Err(e) = write(&output_path, minified) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Minified HTML written to {}", output_path);
    }
}