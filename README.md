```
 _____ _   ___      ______ _____ _____ ______      _____   _____
|_   _| \ | \ \    / / __ \_   _/ ____|  ____|    |  __ \ / ____|
  | | |  \| |\ \  / / |  | || || |    | |__ ______| |__) | (___
  | | | . ` | \ \/ /| |  | || || |    |  __|______|  _  / \___ \
 _| |_| |\  |  \  / | |__| || || |____| |____     | | \ \ ____) |
|_____|_| \_|   \/   \____/_____\_____|______|    |_|  \_\_____/

```

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Ratatui](https://img.shields.io/badge/Ratatui-1a1a1a?style=for-the-badge)](https://ratatui.rs)
[![Crossterm](https://img.shields.io/badge/Crossterm-333333?style=for-the-badge)](https://crates.io/crates/crossterm)
[![License: MIT](https://img.shields.io/badge/License-MIT-purple?style=for-the-badge)](./LICENSE.md)
[![Status](https://img.shields.io/badge/Status-Active-success?style=for-the-badge)](#)

</div>

A terminal-based **Invoice Management System** with unique invoice number built in **Rust** using [Ratatui](https://github.com/ratatui-org/ratatui).

a TUI interface for managing invoices, and **PDF invoice generation** support.

### Features

- **Add / Edit / View Invoices** directly in your terminal
- **6 Random Invoice Numbers** excluding existing ones
- **Persistent storage** (SQLite planned)
- **PDF Invoice Generation** using `headless_chrome`
- Modular architecture with clear file separation
- 100% Rust — no external UI frameworks needed


### Project Structure

```

src/
├── main.rs         # App entry point
├── app.rs          # Application state and mode handling
├── ui.rs           # Main UI rendering logic
├── form.rs         # Add/Edit invoice form
├── splash.rs       # Animated splash screen
├── pdf.rs          # PDF invoice generation
└── utils.rs        # Utility functions for invoice generation

```

### Getting Started (For Development)

#### 1. Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- A terminal that supports colors

#### 2. Clone the Repository

```bash
git clone https://github.com/shayyz-code/invoice-rs.git
cd invoice-rs
````

#### 3. Run the App

```bash
cargo run
```

The app will start with an animated **INVOICE-RS** splash screen,
then open the invoice TUI.


### Controls

| Key     | Action                                          |
| ------- | ----------------------------------------------- |
| `Tab`   | Switch between input fields in the invoice form |
| `Enter` | Confirm and save invoice                        |
| `Esc`   | Cancel current action                           |
| `↑ / ↓` | Navigate invoice list                           |
| `q`     | Quit the app                                    |
| `n`     | New invoice                                     |
| `e`     | Edit invoice                                    |
| `d`     | Delete invoice                                  |
| `s`     | Save invoice                                    |
| `p`     | Export invoice pdf                              |


### Tech Stack

* **Language:** Rust
* **UI Framework:** [Ratatui](https://github.com/ratatui-org/ratatui)
* **Event Handling:** Crossterm
* **Random IDs:** Rand
* **PDF Generation:** genpdf
* **Database:** Sqlite / Serde


<div align="center">

**Contributions are always welcome <3**

[![Contributions Welcome](https://img.shields.io/badge/Contributions-Welcome-purple?style=for-the-badge)](../../issues)

[Open an issue](../../issues) or [create a pull request](../../pulls) to help improve **invoice-rs**

</div>



### License

MIT License © 2025 Aung Min Khant


// **invoice-rs** — where invoices meet terminal art.
