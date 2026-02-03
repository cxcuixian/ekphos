use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::Tabs,
    Frame,
};

use crate::app::App;

pub fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    if app.open_tabs.is_empty() {
        return;
    }

    let titles: Vec<Line> = app
        .open_tabs
        .iter()
        .map(|path| {
            let title = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            Line::from(title)
        })
        .collect();

    let current_path = app
        .notes
        .get(app.selected_note)
        .and_then(|n| n.file_path.as_ref());
    let selected_tab = current_path
        .and_then(|p| app.open_tabs.iter().position(|tp| tp == p))
        .unwrap_or(0);

    let tabs = Tabs::new(titles)
        .select(selected_tab)
        .style(Style::default().fg(app.theme.muted))
        .highlight_style(
            Style::default()
                .fg(app.theme.primary)
                .add_modifier(Modifier::BOLD),
        )
        .divider("|");

    f.render_widget(tabs, area);
}
