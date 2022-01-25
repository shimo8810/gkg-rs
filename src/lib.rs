pub mod args;
pub mod tui;
#[derive(Debug)]
pub struct Item {
    name: String,
    desc: Option<String>,
    about: Option<String>,
    url: String,
}

impl Item {
    pub fn new(name: String, desc: Option<String>, about: Option<String>, url: String) -> Self {
        Self {
            name,
            desc,
            about,
            url,
        }
    }
}
