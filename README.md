# DragonBallTUI
 
A terminal-based application to explore the Dragon Ball series, including episodes and movies.

## Features

- Browse episodes by series (Dragon Ball, Dragon Ball Z, etc.)
- View detailed information about each episode
- Browse and view details of Dragon Ball movies
- User-friendly terminal interface with keyboard navigation

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
- Use `Up` and `Down` arrow keys to navigate through episodes or movies
- Press `Enter` to view details of a selected episode or movie
- Press `Esc` to go back from details view to list view
- Press `q` to quit the application

## Data Files

The application uses two JSON files to store data:

- `episodes.json`: Contains information about all episodes
- `movies.json`: Contains information about all movies

If these files don't exist, the application will create them with default data.

## Customization

You can modify the `episodes.json` and `movies.json` files to add, remove, or update information about episodes and movies.

## Dependencies

- tui: Terminal user interface library
- crossterm: Terminal manipulation library
- serde: Serialization and deserialization library for JSON

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## TODO

- [ ] Implement the Characters tab functionality
- [x] Add search functionality 
- [ ] Implement sorting options 
