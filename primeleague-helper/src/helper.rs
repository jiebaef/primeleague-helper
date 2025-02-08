use scraper::{ElementRef, Selector};

#[derive(Clone)]
pub struct Selectors {
    pub(crate) team_tag: Selector,
}

pub(crate) fn elementref_text(element: &ElementRef<'_>, join_by: Option<&str>) -> String {
    return element
        .text()
        .collect::<Vec<_>>()
        .join(join_by.unwrap_or(" "));
}

pub fn init_selectors() -> Selectors {
    fn new(selector_name: &str, selector: &str) -> Selector {
        Selector::parse(selector)
            .expect(&format!("could not create selector \"{}\"", selector_name))
    }
    Selectors {
        team_tag: new("team_name", "div#page-container > header.page-block.page-header > div.block-content > div.page-title > h1"),
    }
}
