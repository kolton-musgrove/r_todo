# r_todo

A minimalist TUI-based ToDo application written in Rust, focusing on simplicity and usability.

## Overview

r_todo is a terminal-based todo application that cuts through the noise. No unnecessary frills, no confusing icons - just a straightforward tool to manage your tasks efficiently within your terminal.

## Features

- **Simple Interface**: Clean TUI that's easy to navigate and understand
- **Task Management**: Create, edit, and delete todos with ease
- **Priority Levels**: Assign High, Medium, or Low priority to your tasks
- **Context-Sensitive Help**: A mode-dependent help menu that shows only what you need
- **Terminal-Based**: Works entirely in your terminal - no need for a GUI

## Screenshots

<img width="1325" alt="image" src="https://github.com/user-attachments/assets/c4302ace-fb99-4a19-8985-ba171dd21665" />


## Technologies

- **Rust**: For performance and reliability
- **[ratatui](https://github.com/ratatui-org/ratatui)**: Terminal UI framework for Rust

## Installation

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/r_todo.git

# Navigate to the project directory
cd r_todo

# Build the release version
cargo install --path .

# The binary will be available via `r_todo`
```

## Usage

```
# Launch the application
r_todo

# Basic keybindings
n - Create new todo
e - Edit selected todo
d - Delete selected todo
h, j, k, l - Navigation
? - Toggle help menu
q - Quit
```

## Motivation

r_todo was born from a desire to have a free-forever todo application that focuses on functionality rather than flashy features that often confuse users.

---

*This project is under active development.*
