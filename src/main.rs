use std::error::Error;

use comrak::{markdown_to_html, ComrakOptions};
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::error;
use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
struct Event {
    input: String,
    #[serde(with = "MdOptions", default)]
    options: ComrakOptions,
}

#[derive(Deserialize, Clone, Default)]
#[serde(remote = "ComrakOptions", default)]
struct MdOptions {
    hardbreaks: bool,
    smart: bool,
    github_pre_lang: bool,
    width: usize,
    default_info_string: Option<String>,
    unsafe_: bool,
    ext_strikethrough: bool,
    ext_tagfilter: bool,
    ext_table: bool,
    ext_autolink: bool,
    ext_tasklist: bool,
    ext_superscript: bool,
    ext_header_ids: Option<String>,
    ext_footnotes: bool,
    ext_description_lists: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: Event, c: Context) -> Result<String, HandlerError> {
    if e.input == "" {
        error!("Empty input in request {}", c.aws_request_id);
        return Err(c.new_error("Empty input"));
    }
    Ok(markdown_to_html(&e.input, &e.options))
}
