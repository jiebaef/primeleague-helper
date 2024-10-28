use scraper::ElementRef;

pub(crate) fn elementref_text(element: &ElementRef<'_>, join_by: Option<&str>) -> String {
    return element
        .text()
        .collect::<Vec<_>>()
        .join(join_by.unwrap_or(" "));
}
