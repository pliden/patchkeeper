use crate::repo::CommitUtils;
use anyhow::Result;
use crossterm::cursor;
use crossterm::event;
use crossterm::execute;
use crossterm::terminal;
use git2::Commit;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::List;
use ratatui::widgets::ListState;
use ratatui::Terminal;
use ratatui::TerminalOptions;
use ratatui::Viewport;
use std::io::stdout;
use std::io::Stdout;

const MAX_HEIGHT_PERCENT: u16 = 75;
const MAX_WIDTH_PERCENT: u16 = 50;

const TITLE_FG: Color = Color::Rgb(0xf8, 0xfb, 0xfe);
const TITLE_BG: Color = Color::Rgb(0x00, 0x72, 0xbf);

const LIST_FG: Color = Color::Rgb(0xf8, 0xfb, 0xfe);
const LIST_BG: Color = Color::Rgb(0x43, 0x45, 0x5f);

const HIGHLIGHT_FG: Color = Color::Rgb(0xf8, 0xfb, 0xfe);
const HIGHLIGHT_BG: Color = Color::Rgb(0xca, 0x43, 0x35);

struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    columns: u16,
    cursor_restore_row: u16,
}

fn init(nitems: usize) -> Result<UI> {
    let (columns, rows) = terminal::size()?;
    let (_, cursor_row) = cursor::position()?;

    let title_height = 1;
    let list_height = title_height + nitems;
    let viewport_height_max = ((rows * MAX_HEIGHT_PERCENT) / 100) as usize;
    let viewport_height = list_height.min(viewport_height_max) as u16;
    let last_row = cursor_row + viewport_height;
    let scroll_rows = if last_row > rows { last_row - rows } else { 0 };
    let cursor_restore_row = cursor_row - scroll_rows;

    let viewport = Viewport::Inline(viewport_height);
    let terminal = ratatui::init_with_options(TerminalOptions { viewport });

    Ok(UI {
        terminal,
        columns,
        cursor_restore_row,
    })
}

fn fini(ui: UI) -> Result<()> {
    ratatui::restore();
    Ok(execute!(
        stdout(),
        cursor::MoveTo(0, ui.cursor_restore_row)
    )?)
}

fn format_commits(commits: &[Commit]) -> Vec<String> {
    commits
        .iter()
        .map(|commit| {
            let id = commit.short_id().unwrap_or("<invalid id>".to_string());
            let summary = commit.summary().unwrap_or("<empty>");
            format!("{} {}", id, summary)
        })
        .collect::<Vec<_>>()
}

pub fn select_commit(title: &str, commits: &[Commit], select_top: bool) -> Result<Option<usize>> {
    let items = format_commits(commits);
    select_item(title, &items, select_top)
}

fn format_item(item: &str, padding: usize, width: usize) -> String {
    let pad = " ";
    let dots = "...";
    let width_max = width - (padding * 2);

    if item.len() > width_max {
        let item = &item[..width_max - dots.len()];
        format!("{:>padding$}{}{}{:<padding$}", pad, item, dots, pad)
    } else {
        format!("{:>padding$}{}{:<padding$}", pad, item, pad)
    }
}

fn format_items(items: &[String], padding: usize, width: usize) -> Vec<String> {
    items
        .iter()
        .map(|item| format_item(item, padding, width))
        .collect::<Vec<_>>()
}

fn format_width(title: &str, items: &[String], width_min: usize, width_max: usize) -> usize {
    items
        .iter()
        .map(String::len)
        .max()
        .unwrap_or(0)
        .max(title.len())
        .clamp(width_min, width_max)
}

fn select_item(title: &str, items: &[String], select_top: bool) -> Result<Option<usize>> {
    let mut ui = init(items.len())?;

    let selected = if select_top { 0 } else { items.len() - 1 };
    let mut state = ListState::default().with_selected(Some(selected));

    let padding = 1;
    let width_max = ui.columns as usize;
    let width_min = ((ui.columns * MAX_WIDTH_PERCENT) / 100) as usize;

    let title = format_item(title, padding, width_max);
    let items = format_items(items, padding, width_max);
    let width = format_width(&title, &items, width_min, width_max) as u16;

    let title_style = Style::default().bg(TITLE_BG).fg(TITLE_FG);
    let list_style = Style::default().bg(LIST_BG).fg(LIST_FG);
    let highlight_style = Style::default().bg(HIGHLIGHT_BG).fg(HIGHLIGHT_FG);

    while state.selected().is_some() {
        ui.terminal.draw(|frame| {
            let items = items.iter().map(String::as_str).collect::<Vec<_>>();
            let list = List::new(items)
                .block(Block::default().title(title.as_str()).style(title_style))
                .highlight_style(highlight_style)
                .style(list_style);

            let area = frame.area();
            let horizontal = Layout::horizontal([Constraint::Length(width)]).split(area);
            frame.render_stateful_widget(list, horizontal[0], &mut state);
        })?;

        match event::read()? {
            event::Event::Key(key) => match key.code {
                event::KeyCode::Home => state.select_first(),
                event::KeyCode::End => state.select_last(),
                event::KeyCode::Down => state.select_next(),
                event::KeyCode::PageDown => state.scroll_down_by(4),
                event::KeyCode::Up => state.select_previous(),
                event::KeyCode::PageUp => state.scroll_up_by(4),
                event::KeyCode::Esc | event::KeyCode::Char('q') => state.select(None),
                event::KeyCode::Enter => break,
                _ => {}
            },
            event::Event::Resize(_, _) => ui.terminal.autoresize()?,
            _ => {}
        };
    }

    ui.terminal.draw(|frame| {
        let area = frame.area();
        let block = Block::new();
        frame.render_widget(block, area);
    })?;

    fini(ui)?;

    Ok(state.selected())
}
