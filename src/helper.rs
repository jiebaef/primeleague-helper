use scraper::{ElementRef, Selector};

use crate::Selectors;

pub(crate) fn elementref_text(element: &ElementRef<'_>, join_by: Option<&str>) -> String {
    return element
        .text()
        .collect::<Vec<_>>()
        .join(join_by.unwrap_or(" "));
}

pub(crate) fn init_selectors() -> Selectors {
    Selectors {
        logs: Selector::parse("section.league-match-logs > div > div > div > table.table.table-flex.table-responsive.table-static > tbody > tr").expect("Could not create logs_selector"),
        action_span: Selector::parse("td > span").expect("Could not create action_span_selector"),
        split_link: Selector::parse("div.page-header-content > div > ul > li.breadcrumbs-item:nth-child(2) > a",).expect("Could not create split_selector"),
        team_names: Selector::parse("div.content-match-head-team > div > div > a > h2").expect("Could not create team_names_selector"),
        team_links: Selector::parse("div.content-match-head-team-titles > a").expect("Could not create team_names_selector"),
        team_participants: Selector::parse("div").expect("Could not create team_names_selector"),
        game_account: Selector::parse("ul.quick-info > li > span[title*=\"League of Legends » LoL Summoner Name\"]",).expect("could not create game account selector"),
    }
}
