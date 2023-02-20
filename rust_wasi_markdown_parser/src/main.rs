use pulldown_cmark::{html, Parser};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rust_wasi_markdown_parser",
    about = "Markdown to html render cli, ,written by Rust and Wasi"
)]
struct Options {
    #[structopt(parse(from_os_str))]
    filename: std::path::PathBuf,
}

// Our entrypoint into our wasi module
fn main() {
    // Get the passed Cli args
    let options = Options::from_args();

    // Read the markdown file to string
    let contents = std::fs::read_to_string(options.filename).expect("Failed to read file");

    // Run our parsing function to get back a html string
    let html = render_markdown(contents);

    println!("{}", html);
}

fn render_markdown(contents: String) -> String {
    let mut file_buf = String::new();
    let parser = Parser::new(&contents[..]);
    html::push_html(&mut file_buf, parser);
    file_buf
}
