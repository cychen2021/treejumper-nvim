# TreeJumper-NeoVim

A powerful Neovim plugin for navigating code using tree-sitter Abstract Syntax Trees (AST). Jump through functions, classes, and other code structures with ease.

## Features

- 🚀 AST-based code navigation using tree-sitter
- 🔤 Multi-language support: C, C++, Python, Rust, and C#
- ⚡ Fast and efficient navigation through code structures
- 🎯 Jump to next/previous function, class, or definition
- 📍 Visual indicators for navigation points

## Installation

### Using a plugin manager (e.g., `lazy.nvim`)

```lua
{
    "cychen2021/treejumper-nvim",
    build = "cargo build --release",
    config = function()
        require("treejumper").setup()
    end
}
```

## Usage

### Basic Commands

```vim
" Jump to next function
:TJNext

" Jump to previous function
:TJPrevious

" Show current node info
:TJInfo
```

### Key Mappings (Example)

```lua
local tj = require("treejumper")
vim.keymap.set('n', '<C-k>', tj.jump_next, { noremap = true })
vim.keymap.set('n', '<C-j>', tj.jump_prev, { noremap = true })
```

## Supported Languages

- **C** (.c, .h)
- **C++** (.cpp, .cc, .cxx, .hpp, .h)
- **Python** (.py)
- **Rust** (.rs)
- **C#** (.cs)

## Architecture

TreeJumper is built with:

- **Backend**: Rust with tree-sitter for AST parsing and navigation
- **Frontend**: Lua/Neovim integration
- **Communication**: JSON-RPC interface between Lua and Rust

### Project Structure

```
treejumper-nvim/
├── src/                      # Rust backend
│   ├── lib.rs               # Library entry point
│   ├── error.rs             # Error types
│   ├── ast.rs               # AST representation
│   ├── parser.rs            # Tree-sitter parser wrapper
│   └── navigator.rs         # Navigation logic
├── lua/                      # Lua plugin interface
│   └── treejumper/
│       ├── init.lua         # Main plugin entry
│       └── api.lua          # API definitions
├── tests/                    # Integration tests
├── Cargo.toml               # Rust dependencies
└── README.md
```

## Development

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Development Setup

1. Clone the repository
2. Ensure you have Rust 1.70+ installed
3. Build with `cargo build --release`
4. Add to your Neovim config

## Roadmap

- [ ] Implement full tree-sitter integration
- [ ] Add Lua bindings for Neovim
- [ ] Implement code folding based on AST
- [ ] Add visual highlighting for navigation
- [ ] Support for more languages (Go, Java, TypeScript, etc.)
- [ ] Performance optimizations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT

## Author

[Your Name]
