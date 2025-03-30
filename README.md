# DragonBallTUI
 
A terminal-based application to explore the Dragon Ball series, including episodes, movies, and characters.

## Features

- Browse episodes by series (Dragon Ball, Dragon Ball Z, etc.)
- View detailed information about each episode
- Browse and view details of Dragon Ball movies
- Browse and view details of Dragon Ball characters
- User-friendly terminal interface with keyboard navigation
- Search functionality for episodes, movies, and characters
- Sorting options for episodes, movies, and characters
- **Pagination** for navigating large datasets efficiently
- **Fuzzy search** to find content even with typos
- **Customizable themes** with multiple color schemes
- **Configuration system** for user preferences

## Installation

1. Make sure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository:
   ```
   git clone https://github.com/Pewpenguin/DragonBallTUI
   cd dragon-ball-guide
   ```

3. Build the application:
   ```
   cargo build --release
   ```

## Usage

Run the application using:

```
cargo run --release
```

### Navigation

- Use `Tab` to switch between Episodes, Movies, and Characters tabs
- Use `Left` and `Right` arrow keys to navigate between different series in the Episodes tab
- Use `Up` and `Down` arrow keys to navigate through episodes, movies, or characters
- Press `Enter` to view details of a selected episode, movie, or character
- Press `Esc` to go back from details view to list view
- Press `q` to quit the application
- Press `s` to enter search mode
- Press `m` to change sort method
- Press `o` to toggle sort order
- Press `h` to view help screen
- Press `t` to cycle through themes

### Pagination Controls

- Press `n` or `N` to go to the next page
- Press `p` or `P` to go to the previous page
- Press `f` or `F` to go to the first page
- Press `l` or `L` to go to the last page

### Search

The application features a powerful fuzzy search capability:

1. Press `s` to enter search mode
2. Type your search query (fuzzy matching will find results even with typos)
3. Use pagination controls (`n`, `p`, `f`, `l`) to navigate through search results
4. Press `Enter` to select a search result
5. Press `Esc` to exit search mode

Search results are highlighted to show matching characters, making it easy to see why a particular item was included in the results.

### Sorting

- Press `m` to cycle through sort methods (Number/Title/Release Date)
- Press `o` to toggle sort order (Ascending/Descending)

## Configuration

The application uses a `config.json` file for customization. You can modify the following settings:

- `theme`: Current theme name (default, dark, light)
- `themes`: Color configurations for each theme
- `items_per_page`: Number of items to display per page
- `show_keyboard_shortcuts`: Whether to display keyboard shortcuts
- `enable_animations`: Enable/disable UI animations
- `default_sort_method`: Default method for sorting items
- `default_sort_order`: Default order for sorting (ascending/descending)
- `enable_fuzzy_search`: Enable/disable fuzzy search capability

## Data Files

The application uses three JSON files to store data:

- `episodes.json`: Contains information about all episodes
- `movies.json`: Contains information about all movies
- `characters.json`: Contains information about all characters

If these files don't exist, the application will create them with default data.

## Customization

You can modify the `episodes.json`, `movies.json`, and `characters.json` files to add, remove, or update information about episodes, movies, and characters.

## Dependencies

- tui: Terminal user interface library
- crossterm: Terminal manipulation library
- serde: Serialization and deserialization library for JSON
- fuzzy_matcher: Library for fuzzy text matching
- chrono: Date and time library

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## TODO

- [x] Implement the Characters tab functionality
- [x] Add search functionality 
- [x] Implement sorting options
- [x] Add pagination controls
- [x] Implement fuzzy search
