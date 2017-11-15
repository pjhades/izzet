use error::Result;
use pulldown_cmark::{Parser, html};

pub fn markdown_to_html(md: &str) -> Result<String> {
    let parser = Parser::new(md);
    let mut ret = String::new();
    html::push_html(&mut ret, parser);

    Ok(ret)
}
