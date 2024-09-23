use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use crate::app::{App, AppMode, SearchResultType, EpisodeSortMethod, SortOrder, MovieSortMethod};

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
        AppMode::Help => draw_help_screen(f, layout_chunks[1]),
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

fn draw_search_tab<B: Backend>(f: &mut Frame<B>, app: &mut App, area: tui::layout::Rect) {
    let search_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ].as_ref())
        .split(area);

    let search_input = Paragraph::new(app.search_query.as_ref())
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default().borders(Borders::ALL).title("Search").style(Style::default().fg(Color::White)));
    f.render_widget(search_input, search_layout[0]);

    let results: Vec<ListItem> = app.search_results.iter()
        .map(|result| {
            let result_type = match result.result_type {
                SearchResultType::Episode(_, _) => "Episode",
                SearchResultType::Movie(_) => "Movie",
            };
            ListItem::new(vec![
                Spans::from(vec![
                    Span::styled(format!("[{}] ", result_type), Style::default().fg(Color::Green)),
                    Span::raw(&result.title),
                ]),
            ])
        })
        .collect();

    let results_list = List::new(results)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .highlight_style(Style::default().bg(Color::Yellow));

    f.render_stateful_widget(results_list, search_layout[1], &mut app.list_state);
}

fn draw_main_tabs<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let tab_titles = ["Episodes", "Movies", "Characters"];
    let spans: Vec<Spans> = tab_titles.iter().map(|&t| {
        Spans::from(vec![Span::styled(t, Style::default().fg(Color::White))])
    }).collect();

    let tabs = Tabs::new(spans)
        .block(Block::default().borders(Borders::BOTTOM).title("Main Tabs").style(Style::default().fg(Color::White)))
        .style(Style::default().bg(Color::Black).fg(Color::Gray))
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

        let sort_method = match app.episode_sort_method {
            EpisodeSortMethod::EpisodeNumber => "Ep#",
            EpisodeSortMethod::Title => "Title",
            EpisodeSortMethod::ReleaseDate => "Date",
        };
        let sort_order = match app.episode_sort_order {
            SortOrder::Ascending => "↑",
            SortOrder::Descending => "↓",
        };
        let sort_info = format!("[{} {}]", sort_method, sort_order);

        let title = Spans::from(vec![
            Span::styled("Episodes ", Style::default().fg(Color::LightCyan)),
            Span::styled(sort_info, Style::default().fg(Color::LightYellow)),
        ]);

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().bg(Color::Yellow));
        f.render_stateful_widget(list, area, &mut app.list_state);
    }
}

fn draw_episode_details<B: Backend>(f: &mut Frame<B>, app: &App, series_index: usize, episode_index: usize, area: tui::layout::Rect) {
    if let Some(series) = app.guide.get(series_index) {
        if let Some(episode) = series.episodes.get(episode_index) {
            let block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    format!(" Episode Details: {} ", episode.title),
                    Style::default().add_modifier(Modifier::BOLD)
                ))
                .border_style(Style::default().fg(Color::Cyan));

            let details = vec![
                Spans::from(vec![
                    Span::styled("Episode Number: ", Style::default().fg(Color::Yellow)),
                    Span::raw(episode.episode_number.to_string()),
                ]),
                Spans::from(vec![
                    Span::styled("Release Date: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&episode.release_date),
                ]),
                Spans::from(vec![
                    Span::styled("Duration: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&episode.duration),
                ]),
                Spans::from(vec![
                    Span::styled("Saga: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&episode.saga),
                ]),
                Spans::from(""),
                Spans::from(vec![
                    Span::styled("Description: ", Style::default().fg(Color::Yellow)),
                ]),
                Spans::from(Span::raw(&episode.description)),
            ];

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
    
    let sort_method = match app.movie_sort_method {
        MovieSortMethod::Number => "Num",
        MovieSortMethod::Title => "Title",
        MovieSortMethod::ReleaseDate => "Date",
    };
    let sort_order = match app.movie_sort_order {
        SortOrder::Ascending => "↑",
        SortOrder::Descending => "↓",
    };
    let sort_info = format!("[{} {}]", sort_method, sort_order);

    let title = Spans::from(vec![
        Span::styled("Movies ", Style::default().fg(Color::Magenta)),
        Span::styled(sort_info, Style::default().fg(Color::Yellow)),
    ]);

    let movies_list = List::new(movie_items)
        .block(Block::default().borders(Borders::ALL).title(title).style(Style::default().fg(Color::White)))
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));

    f.render_stateful_widget(movies_list, area, &mut app.list_state);
}

fn draw_movie_details<B: Backend>(f: &mut Frame<B>, app: &App, movie_index: usize, area: tui::layout::Rect) {
    if let Some(movie) = app.movies.get(movie_index) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                format!(" Movie Details: {} ", movie.title),
                Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)
            ))
            .border_style(Style::default().fg(Color::Gray));

        let details = vec![
            Spans::from(vec![
                Span::styled("Number: ", Style::default().fg(Color::Yellow)),
                Span::raw(movie.number.to_string()),
            ]),
            Spans::from(vec![
                Span::styled("Release Date: ", Style::default().fg(Color::Yellow)),
                Span::raw(&movie.release_date),
            ]),
            Spans::from(vec![
                Span::styled("Runtime: ", Style::default().fg(Color::Yellow)),
                Span::raw(&movie.runtime),
            ]),
            Spans::from(vec![
                Span::styled("Director: ", Style::default().fg(Color::Yellow)),
                Span::raw(&movie.director),
            ]),
            Spans::from(vec![
                Span::styled("Genres: ", Style::default().fg(Color::Yellow)),
                Span::raw(movie.genres.join(", ")),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("Description: ", Style::default().fg(Color::Yellow)),
            ]),
            Spans::from(Span::raw(&movie.description)),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("Trivia: ", Style::default().fg(Color::Yellow)),
            ]),
            Spans::from(Span::raw(&movie.trivia)),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("Plot Keywords: ", Style::default().fg(Color::Yellow)),
                Span::raw(movie.plot_keywords.join(", ")),
            ]),
        ];

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

fn draw_help_screen<B: Backend>(f: &mut Frame<B>, area: tui::layout::Rect) {
    let help_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ].as_ref())
        .split(area);

    let title = Paragraph::new("Help")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));

    f.render_widget(title, help_layout[0]);

    let help_items = vec![
        ("Navigation", vec![
            ("Tab", "Switch between main tabs"),
            ("Left/Right", "Navigate series tabs (in Episodes tab)"),
            ("Up/Down", "Navigate lists"),
            ("Enter", "View details of selected item"),
            ("Esc", "Go back / Exit search"),
        ]),
        ("Actions", vec![
            ("Q/q", "Quit the application"),
            ("H/h", "Toggle this help screen"),
            ("S/s", "Enter search mode"),
        ]),
        ("Sorting", vec![
            ("M/m", "Change sort method"),
            ("O/o", "Toggle sort order"),
        ]),
    ];

    let mut text = Vec::new();

    for (section, items) in help_items {
        text.push(Spans::from(Span::styled(
            section,
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        )));
        text.push(Spans::from(""));

        for (key, description) in items {
            text.push(Spans::from(vec![
                Span::styled(format!("{:<12}", key), Style::default().fg(Color::Green)),
                Span::raw(description),
            ]));
        }

        text.push(Spans::from(""));
    }

    let help_paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)))
        .wrap(tui::widgets::Wrap { trim: true });

    f.render_widget(help_paragraph, help_layout[1]);
}