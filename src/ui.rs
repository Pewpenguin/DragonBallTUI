use crate::app::{App, AppMode, EpisodeSortMethod, MovieSortMethod, SearchResultType, SortOrder};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(size);

    draw_main_tabs(f, app, layout_chunks[0]);

    match app.app_mode {
        AppMode::Help => draw_help_screen(f, layout_chunks[1]),
        AppMode::Search => draw_search_tab(f, app, layout_chunks[1]),
        AppMode::CharacterDetails(character_index) => {
            draw_character_details(f, app, character_index, layout_chunks[1])
        }
        _ => match app.selected_tab {
            0 => draw_episodes_tab(f, app, layout_chunks[1]),
            1 => draw_movies_tab(f, app, layout_chunks[1]),
            2 => draw_characters_tab(f, app, layout_chunks[1]),
            _ => {}
        },
    }
}

fn draw_search_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let search_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);

    let search_input = Paragraph::new(app.search_query.as_str())
        .style(Style::default().fg(Color::LightCyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(Span::styled(
                    " Search ",
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(search_input, search_layout[0]);

    // Update search pagination total items
    app.search_pagination.total_items = app.search_results.len();

    // Get visible search results for current page
    let visible_results = app.search_pagination.get_visible_items(&app.search_results);

    let is_searching = app.search_query.len() > 0 && app.search_results.is_empty();
    let spinner_symbols = vec!["-", "\\", "|", "/"];
    let spinner_index = (app.search_query.len() % spinner_symbols.len()) as usize;
    let spinner = if is_searching {
        spinner_symbols[spinner_index]
    } else {
        ""
    };

    let results: Vec<ListItem> = visible_results
        .iter()
        .map(|result| {
            let result_type = match result.result_type {
                SearchResultType::Episode(_, _) => "[Episode]",
                SearchResultType::Movie(_) => "[Movie]",
                SearchResultType::Character(_) => "[Character]",
            };
            ListItem::new(vec![Line::from(vec![
                Span::styled(
                    format!("{} ", result_type),
                    Style::default().fg(app.config.get_color("secondary")),
                ),
                Span::raw(&result.title),
            ])])
        })
        .collect();

    let page_info = app.search_pagination.page_info();
    let title = Line::from(vec![
        Span::styled(
            "Results ",
            Style::default().fg(app.config.get_color("primary")),
        ),
        Span::styled(
            format!("{} {}", spinner, page_info),
            Style::default().fg(app.config.get_color("accent")),
        ),
    ]);

    let results_list = List::new(results)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(title),
        )
        .highlight_style(Style::default().bg(app.config.get_color("highlight")));

    f.render_stateful_widget(results_list, search_layout[1], &mut app.list_state);

    if !app.search_query.is_empty() {
        let status_text = if is_searching {
            format!("Searching for '{}' {}", app.search_query, spinner)
        } else {
            let pagination_help = if app.search_results.len() > app.search_pagination.items_per_page
            {
                " (Use Ctrl+P/N/F/L for pagination)"
            } else {
                ""
            };
            format!(
                "Found {} results for '{}'{}",
                app.search_results.len(),
                app.search_query,
                pagination_help
            )
        };

        let search_status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title("Status"),
            )
            .alignment(Alignment::Center);

        f.render_widget(search_status, search_layout[2]);
    }
}

fn draw_main_tabs(f: &mut Frame, app: &App, area: Rect) {
    let tab_titles = ["Episodes", "Movies", "Characters"];
    let spans: Vec<Line> = tab_titles
        .iter()
        .map(|&t| Line::from(vec![Span::styled(t, Style::default().fg(Color::White))]))
        .collect();

    let tabs = Tabs::new(spans)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title("Main Tabs")
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().bg(Color::Black).fg(Color::Gray))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "))
        .select(app.selected_tab);

    f.render_widget(tabs, area);
}

fn draw_episodes_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let layout_with_series_tabs = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    draw_series_tabs(f, app, layout_with_series_tabs[0]);

    match app.app_mode {
        AppMode::EpisodesSeries(series_index) => {
            draw_episodes_list(f, app, series_index, layout_with_series_tabs[1]);
        }
        AppMode::Details(series_index, episode_index) => {
            draw_episode_details(
                f,
                app,
                series_index,
                episode_index,
                layout_with_series_tabs[1],
            );
        }
        _ => {}
    }
}

fn draw_series_tabs(f: &mut Frame, app: &App, area: Rect) {
    let series_names: Vec<String> = app
        .guide
        .iter()
        .map(|series| series.series.clone())
        .collect();

    let series_tabs: Vec<Line> = series_names
        .iter()
        .map(|name| Line::from(vec![Span::styled(name, Style::default().fg(Color::White))]))
        .collect();

    let series_tabs_widget = Tabs::new(series_tabs)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title("Series Tabs"),
        )
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "))
        .select(app.selected_series_tab);

    f.render_widget(series_tabs_widget, area);
}

fn draw_episodes_list(
    f: &mut Frame,
    app: &mut App,
    series_index: usize,
    area: Rect,
) {
    if let Some(series) = app.guide.get(series_index) {
        // Update pagination total items
        app.episodes_pagination.total_items = series.episodes.len();

        // Get visible episodes for current page
        let visible_episodes = app.episodes_pagination.get_visible_items(&series.episodes);

        let items: Vec<_> = visible_episodes
            .iter()
            .map(|ep| ListItem::new(format!("{}: {}", ep.episode_number, ep.title)))
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
        let page_info = app.episodes_pagination.page_info();

        let title = Line::from(vec![
            Span::styled(
                "Episodes ",
                Style::default().fg(app.config.get_color("primary")),
            ),
            Span::styled(
                sort_info,
                Style::default().fg(app.config.get_color("secondary")),
            ),
            Span::styled(
                format!(" | {}", page_info),
                Style::default().fg(app.config.get_color("accent")),
            ),
        ]);

        if app.show_charts {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .split(area);

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(Style::default().bg(app.config.get_color("highlight")));
            f.render_stateful_widget(list, chunks[0], &mut app.list_state);
        } else {
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(Style::default().bg(app.config.get_color("highlight")));
            f.render_stateful_widget(list, area, &mut app.list_state);
        }
    }
}

fn draw_episode_details(
    f: &mut Frame,
    app: &App,
    series_index: usize,
    episode_index: usize,
    area: Rect,
) {
    if let Some(series) = app.guide.get(series_index) {
        if let Some(episode) = series.episodes.get(episode_index) {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(Span::styled(
                    format!(" Episode Details: {} ", episode.title),
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .border_style(Style::default().fg(Color::Cyan));

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(area);

            let details = vec![
                Line::from(vec![
                    Span::styled("Episode Number: ", Style::default().fg(Color::Yellow)),
                    Span::raw(episode.episode_number.to_string()),
                ]),
                Line::from(vec![
                    Span::styled("Release Date: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&episode.release_date),
                ]),
                Line::from(vec![
                    Span::styled("Duration: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&episode.duration),
                ]),
                Line::from(vec![
                    Span::styled("Saga: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&episode.saga),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    "Description: ",
                    Style::default().fg(Color::Yellow),
                )]),
                Line::from(Span::raw(&episode.description)),
            ];

            let paragraph = Paragraph::new(details)
                .block(block)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunks[0]);

            let additional_info = Paragraph::new(vec![Line::from(vec![Span::styled(
                "Press 'Esc' to go back to episodes list",
                Style::default().fg(Color::Gray),
            )])])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Navigation"),
            )
            .alignment(Alignment::Center);

            f.render_widget(additional_info, chunks[1]);
        }
    }
}

fn draw_movies_tab(f: &mut Frame, app: &mut App, area: Rect) {
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

fn draw_movies_list(f: &mut Frame, app: &mut App, area: Rect) {
    // Update pagination total items
    app.movies_pagination.total_items = app.movies.len();

    // Get visible movies for current page
    let visible_movies = app.movies_pagination.get_visible_items(&app.movies);

    let movie_items: Vec<_> = visible_movies
        .iter()
        .map(|movie| ListItem::new(format!("{}: {} ", movie.number, movie.title,)))
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
    let page_info = app.movies_pagination.page_info();

    let title = Line::from(vec![
        Span::styled(
            "Movies ",
            Style::default().fg(app.config.get_color("primary")),
        ),
        Span::styled(
            sort_info,
            Style::default().fg(app.config.get_color("secondary")),
        ),
        Span::styled(
            format!(" | {}", page_info),
            Style::default().fg(app.config.get_color("accent")),
        ),
    ]);

    // If charts are enabled, split the area
    if app.show_charts {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(area);

        let movies_list = List::new(movie_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .style(Style::default().fg(app.config.get_color("text"))),
            )
            .highlight_style(Style::default().bg(app.config.get_color("highlight")));

        f.render_stateful_widget(movies_list, chunks[0], &mut app.list_state);
    } else {
        let movies_list = List::new(movie_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .style(Style::default().fg(app.config.get_color("text"))),
            )
            .highlight_style(Style::default().bg(app.config.get_color("highlight")));

        f.render_stateful_widget(movies_list, area, &mut app.list_state);
    }
}

fn draw_movie_details(
    f: &mut Frame,
    app: &App,
    movie_index: usize,
    area: Rect,
) {
    if let Some(movie) = app.movies.get(movie_index) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(Span::styled(
                format!(" Movie Details: {} ", movie.title),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightCyan),
            ))
            .border_style(Style::default().fg(Color::Magenta));

        let details = vec![
            Line::from(vec![
                Span::styled("Number: ", Style::default().fg(Color::Yellow)),
                Span::raw(movie.number.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Release Date: ", Style::default().fg(Color::Yellow)),
                Span::raw(&movie.release_date),
            ]),
            Line::from(vec![
                Span::styled("Runtime: ", Style::default().fg(Color::Yellow)),
                Span::raw(&movie.runtime),
            ]),
            Line::from(vec![
                Span::styled("Director: ", Style::default().fg(Color::Yellow)),
                Span::raw(&movie.director),
            ]),
            Line::from(vec![
                Span::styled("Genres: ", Style::default().fg(Color::Yellow)),
                Span::raw(movie.genres.join(", ")),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Description: ",
                Style::default().fg(Color::Yellow),
            )]),
            Line::from(Span::raw(&movie.description)),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Trivia: ",
                Style::default().fg(Color::Yellow),
            )]),
            Line::from(Span::raw(&movie.trivia)),
            Line::from(""),
            Line::from(vec![
                Span::styled("Plot Keywords: ", Style::default().fg(Color::Yellow)),
                Span::raw(movie.plot_keywords.join(", ")),
            ]),
        ];

        let paragraph = Paragraph::new(details)
            .block(block)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);

        let additional_info = Paragraph::new(vec![Line::from(vec![Span::styled(
            "Press 'Esc' to go back to movies list",
            Style::default().fg(Color::Gray),
        )])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title("Navigation"),
        )
        .alignment(Alignment::Center);

        f.render_widget(additional_info, chunks[1]);
    }
}

fn draw_characters_tab(f: &mut Frame, app: &mut App, area: Rect) {
    // Update pagination total items
    app.characters_pagination.total_items = app.characters.len();

    // Get visible characters for current page
    let visible_characters = app.characters_pagination.get_visible_items(&app.characters);

    let character_items: Vec<_> = visible_characters
        .iter()
        .enumerate()
        .map(|(i, character)| {
            let index = i + app.characters_pagination.current_page
                * app.characters_pagination.items_per_page;
            ListItem::new(format!("{}. {}", index + 1, character.name))
        })
        .collect();

    let sort_order = match app.character_sort_order {
        SortOrder::Ascending => "↑",
        SortOrder::Descending => "↓",
    };
    let sort_info = format!("[Name {}]", sort_order);

    let page_info = app.characters_pagination.page_info();
    let title = Line::from(vec![
        Span::styled(
            "Characters ",
            Style::default().fg(app.config.get_color("primary")),
        ),
        Span::styled(
            sort_info,
            Style::default().fg(app.config.get_color("secondary")),
        ),
        Span::styled(
            format!(" | {}", page_info),
            Style::default().fg(app.config.get_color("accent")),
        ),
    ]);

    let characters_list = List::new(character_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black));

    f.render_stateful_widget(characters_list, area, &mut app.list_state);
}

fn draw_character_details(
    f: &mut Frame,
    app: &App,
    character_index: usize,
    area: Rect,
) {
    if let Some(character) = app.characters.get(character_index) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .title(Span::styled(
                format!(" Character Details: {} ", character.name),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightCyan),
            ))
            .border_style(Style::default().fg(Color::Yellow));

        let details = vec![
            Line::from(vec![
                Span::styled("Aliases: ", Style::default().fg(Color::Yellow)),
                Span::raw(character.aliases.join(", ")),
            ]),
            Line::from(vec![
                Span::styled("Race: ", Style::default().fg(Color::Yellow)),
                Span::raw(&character.race),
            ]),
            Line::from(vec![
                Span::styled("Powers: ", Style::default().fg(Color::Yellow)),
                Span::raw(character.powers.join(", ")),
            ]),
            Line::from(vec![
                Span::styled("Occupation: ", Style::default().fg(Color::Yellow)),
                Span::raw(&character.occupation),
            ]),
            Line::from(vec![Span::styled(
                "Description: ",
                Style::default().fg(Color::Yellow),
            )]),
            Line::from(Span::raw(&character.description)),
            Line::from(""),
            Line::from(vec![
                Span::styled("Key Events: ", Style::default().fg(Color::Yellow)),
                Span::raw(character.key_events.join(", ")),
            ]),
        ];

        let paragraph = Paragraph::new(details)
            .block(block)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);

        let additional_info = Paragraph::new(vec![Line::from(vec![Span::styled(
            "Press 'Esc' to go back to characters list",
            Style::default().fg(Color::Gray),
        )])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title("Navigation"),
        )
        .alignment(Alignment::Center);

        f.render_widget(additional_info, chunks[1]);
    }
}

fn draw_help_screen(f: &mut Frame, area: Rect) {
    let help_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    let title = Paragraph::new("Help")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(title, help_layout[0]);

    let help_items = vec![
        (
            "Navigation",
            vec![
                ("Tab", "Switch between main tabs"),
                ("Left/Right", "Navigate series tabs (in Episodes tab)"),
                ("Up/Down", "Navigate lists"),
                ("Enter", "View details of selected item"),
                ("Esc", "Go back / Exit search"),
                ("P/p", "Previous page"),
                ("N/n", "Next page"),
                ("F/f", "First page"),
                ("L/l", "Last page"),
                ("Ctrl+P", "Previous page (in search mode)"),
                ("Ctrl+N", "Next page (in search mode)"),
                ("Ctrl+F", "First page (in search mode)"),
                ("Ctrl+L", "Last page (in search mode)"),
            ],
        ),
        (
            "Actions",
            vec![
                ("Q/q", "Quit the application"),
                ("H/h", "Toggle this help screen"),
                ("S/s", "Enter search mode"),
                ("C/c", "Toggle charts view"),
                ("T/t", "Cycle through themes"),
            ],
        ),
        (
            "Sorting",
            vec![("M/m", "Change sort method"), ("O/o", "Toggle sort order")],
        ),
        (
            "Features",
            vec![
                ("Themes", "Multiple color themes available"),
                ("Charts", "Visual data representation"),
                ("Pagination", "Navigate through large datasets"),
                ("Fuzzy Search", "Find content even with typos"),
            ],
        ),
    ];

    let mut text = Vec::new();

    for (section, items) in help_items {
        text.push(Line::from(Span::styled(
            section,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        text.push(Line::from(""));

        for (key, description) in items {
            text.push(Line::from(vec![
                Span::styled(format!("{:<12}", key), Style::default().fg(Color::Green)),
                Span::raw(description),
            ]));
        }

        text.push(Line::from(""));
    }

    let help_paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, help_layout[1]);
}
