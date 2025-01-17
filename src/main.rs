use std::io;

use ratzilla::ratatui::layout::{Constraint, Flex, Layout, Offset, Rect};
use ratzilla::ratatui::style::{Style, Stylize};
use ratzilla::ratatui::widgets::{BorderType, Wrap};
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
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.render_on_web(move |frame| {
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

        let background_area = Rect::new(
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
        frame.render_widget(block, background_area);

        let [banner_area, description_area, meetups_area, links_area] =
            Layout::vertical(constraints).areas(area);

        frame.render_widget(
            Paragraph::new(BANNER).alignment(Alignment::Center),
            banner_area,
        );
        frame.render_widget(
            Paragraph::new(description)
                .wrap(Wrap { trim: true })
                .left_aligned()
                .block(Block::bordered()),
            description_area,
        );
        frame.render_widget(
            Paragraph::new("Coming soon!").block(Block::bordered().title("Meetups".bold())),
            meetups_area,
        );
        frame.render_widget(Block::bordered().title("Links".bold()), links_area);
        for (i, (_, url)) in LINKS.iter().enumerate() {
            let link = Hyperlink::new(url);
            frame.render_widget(
                link,
                links_area.offset(Offset {
                    x: 1,
                    y: i as i32 + 1,
                }),
            );
        }
    });

    Ok(())
}
