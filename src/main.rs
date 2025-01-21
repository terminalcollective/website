use std::io;

use gridlife::{CellState, Grid};
use ratzilla::ratatui::layout::{Constraint, Flex, Layout, Offset, Rect};
use ratzilla::ratatui::style::{Style, Stylize};
use ratzilla::ratatui::text::{Line, Text};
use ratzilla::ratatui::widgets::{BorderType, Clear, Wrap};
use ratzilla::ratatui::Frame;
use ratzilla::ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Block, Paragraph},
    Terminal,
};
use ratzilla::widgets::Hyperlink;
use ratzilla::{DomBackend, RenderOnWeb};

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
];

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let mut grid = Grid::new_random(size.width.into(), size.height.into());

    terminal.render_on_web(move |frame| {
        render_game_of_life(&mut grid, frame);

        let vertical = Layout::vertical([Constraint::Percentage(80)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

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

        render_background(frame, area, &constraints);

        let [banner_area, description_area, meetups_area, links_area] =
            Layout::vertical(constraints).areas(area);

        render_banner(frame, banner_area);
        render_description(frame, description, description_area);
        render_meetups(frame, meetups_area);
        render_links(frame, links_area);
    });

    Ok(())
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
    for (i, (_, url)) in LINKS.iter().enumerate() {
        let link = Hyperlink::new(*url);
        frame.render_widget(
            link,
            links_area.offset(Offset {
                x: 1,
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

fn render_background(frame: &mut Frame<'_>, area: Rect, constraints: &[Constraint]) {
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
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(73, 222, 128))
        .style(
            Style::default()
                .fg(Color::Rgb(73, 222, 128))
                .bg(Color::Rgb(16, 24, 39)),
        )
        .title_bottom("|Website built with Ratzilla|".bold())
        .title_alignment(Alignment::Right);
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
}
