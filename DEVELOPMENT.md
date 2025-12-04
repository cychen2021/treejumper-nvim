<!-- This file contains useful templates and development notes -->

## Development Notes

### Next Steps

1. **Implement Tree-Sitter Integration**
   - [ ] Complete the `Parser::parse()` method for each language
   - [ ] Extract function, class, and struct definitions
   - [ ] Handle nested scopes properly

2. **Create Neovim FFI Layer**
   - [ ] Design the communication protocol between Lua and Rust
   - [ ] Implement JSON-RPC or direct function calls
   - [ ] Handle memory management and lifetimes

3. **Implement Navigation Commands**
   - [ ] Add Neovim commands for jumping between nodes
   - [ ] Implement visual indicators
   - [ ] Add configuration options

4. **Testing & Documentation**
   - [ ] Write comprehensive tests for each language
   - [ ] Document the architecture
   - [ ] Create usage examples

### Architecture Decisions

#### AST vs Direct Parsing
We use tree-sitter for AST generation because it's:
- Language-agnostic
- Incremental (can update on edits)
- Efficient for large files

#### Rust Backend
- Handles complex parsing logic
- Better performance than Lua
- Compiled to a native library

#### Lua Frontend
- Direct integration with Neovim
- Easy to configure and extend
- User-friendly API

### Performance Considerations

- Cache parsed ASTs when possible
- Use incremental parsing for edits
- Implement lazy loading for large files
- Monitor memory usage

### Future Language Support

Languages to consider adding:
- Go
- Java
- TypeScript/JavaScript
- PHP
- Ruby
- Swift

## Useful Resources

- [Tree-sitter Documentation](https://tree-sitter.github.io/)
- [Neovim Lua Plugin Development](https://neovim.io/doc/user/lua.html)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
