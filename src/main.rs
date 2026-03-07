use std::io;

use gridlife::{CellState, Grid};
use ratzilla::ratatui::layout::{Constraint, Flex, Layout, Offset, Rect};
use ratzilla::ratatui::style::{Style, Stylize};
use ratzilla::ratatui::text::{Line, Span, Text};
use ratzilla::ratatui::widgets::{BorderType, Clear, Wrap};
use ratzilla::ratatui::Frame;
use ratzilla::ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Block, Paragraph},
    Terminal,
};
use ratzilla::utils::is_mobile;
use ratzilla::widgets::Hyperlink;
use ratzilla::{DomBackend, WebRenderer};

const BANNER: &str = r#"
  _______                  _             _    _____      _ _           _   _           
 |__   __|                (_)           | |  / ____|    | | |         | | (_)          
    | | ___ _ __ _ __ ___  _ _ __   __ _| | | |     ___ | | | ___  ___| |_ ___   _____ 
    | |/ _ \ '__| '_ ` _ \| | '_ \ / _` | | | |    / _ \| | |/ _ \/ __| __| \ \ / / _ \
    | |  __/ |  | | | | | | | | | | (_| | | | |___| (_) | | |  __/ (__| |_| |\ V /  __/
    |_|\___|_|  |_| |_| |_|_|_| |_|\__,_|_|  \_____\___/|_|_|\___|\___|\__|_| \_/ \___|
"#;

const DESCRIPTION: &str = r#"
>_ Terminal Collective is a community for open-source terminal software enthusiasts.

We bring together developers of terminal software and users who share a passion for the terminal and its ecosystem.

Our goal is to create a space where people can share their work, learn from each other, and collaborate on terminal-related projects.
"#;

const LINKS: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/terminalcollective"),
    ("Discord", "https://discord.gg/6EUERBrAMs"),
    ("Twitter", "https://www.youtube.com/@TerminalCollectiveOrg"),
    ("RSS", "https://terminalcollective.org/feed.xml"),
];

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let mut grid = Grid::new_random(size.width.into(), size.height.into());

    terminal.draw_web(move |frame| {
        render_game_of_life(&mut grid, frame);

        let (vert_perc, hori_perc) = if is_mobile() { (30, 80) } else { (80, 60) };

        let vertical = Layout::vertical([Constraint::Percentage(vert_perc)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(hori_perc)]).flex(Flex::Center);
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        if is_mobile() {
            render_mobile(area, frame);
        } else {
            render_desktop(area, frame);
        }
    });

    Ok(())
}

fn render_mobile(area: Rect, frame: &mut Frame) {
    let constraints = [
        Constraint::Length(3),
        Constraint::Length(LINKS.len() as u16 + 2),
    ];
    render_background(
        frame,
        area,
        Some("Terminal Collective".to_string()),
        &constraints,
    );
    let [meetups_area, links_area] = Layout::vertical(constraints).areas(area);
    render_meetups(frame, meetups_area);
    render_links(frame, links_area);
}

fn render_desktop(area: Rect, frame: &mut Frame) {
    let description = textwrap::wrap(DESCRIPTION.trim(), area.width as usize - 15)
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    let constraints = [
        Constraint::Length(BANNER.lines().count() as u16 + 1),
        Constraint::Length(description.lines().count() as u16 + 2),
        Constraint::Length(3),
        Constraint::Length(LINKS.len() as u16 + 2),
    ];
    render_background(frame, area, None, &constraints);
    let [banner_area, description_area, meetups_area, links_area] =
        Layout::vertical(constraints).areas(area);
    render_banner(frame, banner_area);
    render_description(frame, description, description_area);
    render_meetups(frame, meetups_area);
    render_links(frame, links_area);
}

fn render_game_of_life(grid: &mut Grid<CellState>, frame: &mut Frame<'_>) {
    grid.update_states();
    let grid_out = grid.to_string();
    let lines: Vec<Line> = grid_out.lines().map(Line::from).collect();
    let grid_text = Text::from(lines).fg(Color::Rgb(100, 100, 100));
    frame.render_widget(Paragraph::new(grid_text), frame.area());
}

fn render_links(frame: &mut Frame<'_>, links_area: Rect) {
    frame.render_widget(Block::bordered().title("Links".bold()), links_area);
    for (i, (text, url)) in LINKS.iter().enumerate() {
        let label = Span::raw(*text);
        let link = Hyperlink::new(*url);

        frame.render_widget(
            label,
            links_area.offset(Offset {
                x: 1,
                y: i as i32 + 1,
            }),
        );

        frame.render_widget(
            link,
            links_area.offset(Offset {
                x: 10,
                y: i as i32 + 1,
            }),
        );
    }
}

fn render_meetups(frame: &mut Frame<'_>, meetups_area: Rect) {
    frame.render_widget(
        Paragraph::new("Coming soon!").block(Block::bordered().title("Meetups".bold())),
        meetups_area,
    );
}

fn render_description(frame: &mut Frame<'_>, description: String, description_area: Rect) {
    frame.render_widget(
        Paragraph::new(description)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(Block::bordered()),
        description_area,
    );
}

fn render_banner(frame: &mut Frame<'_>, banner_area: Rect) {
    frame.render_widget(
        Paragraph::new(BANNER).alignment(Alignment::Center),
        banner_area,
    );
}

fn render_background(
    frame: &mut Frame<'_>,
    area: Rect,
    title: Option<String>,
    constraints: &[Constraint],
) {
    let mut area = Rect::new(
        area.x - 2,
        area.y - 1,
        area.width + 4,
        constraints
            .iter()
            .map(|c| match *c {
                Constraint::Min(v) | Constraint::Max(v) | Constraint::Length(v) => v,
                _ => 0,
            })
            .sum::<u16>()
            + 3,
    );
    area = area.clamp(frame.area());
    let mut block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(73, 222, 128))
        .style(
            Style::default()
                .fg(Color::Rgb(73, 222, 128))
                .bg(Color::Rgb(16, 24, 39)),
        )
        .title_bottom("|built with Ratzilla|")
        .title_alignment(Alignment::Right);
    if let Some(title) = title {
        block = block.title_top(Line::from(title).alignment(Alignment::Center).bold());
    }
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
}
