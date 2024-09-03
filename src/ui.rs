use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use crate::app::{App, AppMode};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ].as_ref())
        .split(size);

    draw_main_tabs(f, app, layout_chunks[0]);

    match app.app_mode {
        AppMode::Search => draw_search_tab(f, app, layout_chunks[1]),
        _ => {
            match app.selected_tab {
                0 => draw_episodes_tab(f, app, layout_chunks[1]),
                1 => draw_movies_tab(f, app, layout_chunks[1]),
                2 => draw_characters_tab(f, app, layout_chunks[1]),
                _ => {}
            }
        }
    }
}

// Add this new function to draw the search tab
fn draw_search_tab<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let search_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ].as_ref())
        .split(area);

    let search_input = Paragraph::new(app.search_query.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(search_input, search_layout[0]);

    let results_text = "Search results will be displayed here.";
    let results = Paragraph::new(results_text)
        .block(Block::default().borders(Borders::ALL).title("Results"));
    f.render_widget(results, search_layout[1]);
}

fn draw_main_tabs<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let tab_titles = ["Episodes", "Movies", "Characters"];
    let spans: Vec<Spans> = tab_titles.iter().map(|&t| {
        Spans::from(vec![Span::styled(t, Style::default().fg(Color::White))])
    }).collect();

    let tabs = Tabs::new(spans)
        .block(Block::default().borders(Borders::BOTTOM).title("Main Tabs"))
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .divider(Span::raw(" | "))
        .select(app.selected_tab);

    f.render_widget(tabs, area);
}

fn draw_episodes_tab<B: Backend>(f: &mut Frame<B>, app: &mut App, area: tui::layout::Rect) {
    let layout_with_series_tabs = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ].as_ref())
        .split(area);

    draw_series_tabs(f, app, layout_with_series_tabs[0]);

    match app.app_mode {
        AppMode::EpisodesSeries(series_index) => {
            draw_episodes_list(f, app, series_index, layout_with_series_tabs[1]);
        }
        AppMode::Details(series_index, episode_index) => {
            draw_episode_details(f, app, series_index, episode_index, layout_with_series_tabs[1]);
        }
        _ => {}
    }
}

fn draw_series_tabs<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let series_names: Vec<String> = app.guide.iter()
        .map(|series| series.series.clone())
        .collect();

    let series_tabs: Vec<Spans> = series_names.iter().map(|name| {
        Spans::from(vec![Span::styled(name, Style::default().fg(Color::White))])
    }).collect();

    let series_tabs_widget = Tabs::new(series_tabs)
        .block(Block::default().borders(Borders::BOTTOM).title("Series Tabs"))
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .divider(Span::raw(" | "))
        .select(app.selected_series_tab);

    f.render_widget(series_tabs_widget, area);
}

fn draw_episodes_list<B: Backend>(f: &mut Frame<B>, app: &mut App, series_index: usize, area: tui::layout::Rect) {
    if let Some(series) = app.guide.get(series_index) {
        let items: Vec<_> = series.episodes.iter()
            .map(|ep| ListItem::new(format!(
                "{}: {}",
                ep.episode_number,
                ep.title
            )))
            .collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Episodes"))
            .highlight_style(Style::default().bg(Color::Yellow));
        f.render_stateful_widget(list, area, &mut app.list_state);
    }
}

fn draw_episode_details<B: Backend>(f: &mut Frame<B>, app: &App, series_index: usize, episode_index: usize, area: tui::layout::Rect) {
    if let Some(series) = app.guide.get(series_index) {
        if let Some(episode) = series.episodes.get(episode_index) {
            let block = Block::default()
                .borders(Borders::ALL)
                .title(format!("Episode Details: {}", episode.title));
            let details = format!(
                "Episode Number: {}\nDescription: {}\nRelease Date: {}\nDuration: {}\nSaga: {}",
                episode.episode_number,
                episode.description,
                episode.release_date,
                episode.duration,
                episode.saga
            );
            let paragraph = Paragraph::new(details)
                .block(block)
                .wrap(tui::widgets::Wrap { trim: true });
            f.render_widget(paragraph, area);
        }
    }
}

fn draw_movies_tab<B: Backend>(f: &mut Frame<B>, app: &mut App, area: tui::layout::Rect) {
    match app.app_mode {
        AppMode::MoviesList => {
            draw_movies_list(f, app, area);
        }
        AppMode::MovieDetails(movie_index) => {
            draw_movie_details(f, app, movie_index, area);
        }
        _ => {}
    }
}

fn draw_movies_list<B: Backend>(f: &mut Frame<B>, app: &mut App, area: tui::layout::Rect) {
    let movie_items: Vec<_> = app.movies.iter()
        .map(|movie| ListItem::new(format!(
            "{}: {} ",
            movie.number,
            movie.title,
        )))
        .collect();
    
    let movies_list = List::new(movie_items)
        .block(Block::default().borders(Borders::ALL).title("Movies"))
        .highlight_style(Style::default().bg(Color::Yellow));

    f.render_stateful_widget(movies_list, area, &mut app.list_state);
}

fn draw_movie_details<B: Backend>(f: &mut Frame<B>, app: &App, movie_index: usize, area: tui::layout::Rect) {
    if let Some(movie) = app.movies.get(movie_index) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Movie Details: {}", movie.title));
        let details = format!(
            "Number: {}\nRelease Date: {}\nRuntime: {}\nDescription: {}\nDirector: {}\nGenres: {}\nTrivia: {}\nPlot Keywords: {}",
            movie.number,
            movie.release_date,
            movie.runtime,
            movie.description,
            movie.director,
            movie.genres.join(", "),
            movie.trivia,
            movie.plot_keywords.join(", ")
        );
        let paragraph = Paragraph::new(details)
            .block(block)
            .wrap(tui::widgets::Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}

fn draw_characters_tab<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let characters_text = match app.app_mode {
        AppMode::Characters => "Characters details go here.",
        _ => "",
    };

    let paragraph = Paragraph::new(characters_text)
        .block(Block::default().borders(Borders::ALL).title("Characters"));
    f.render_widget(paragraph, area);
}