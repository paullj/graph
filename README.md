# graph (better name pending)

> A graph layout engine and language

### Example:

```
graph down
a --> b
b --> d
a --> c
c --> d

```

Turns into:

> Put a picture here when it looks less terrible

### Goals of this project:

- Language that is easy to read and write by humans or machines
- Parser that can report errors clearly
- Expressive enough to represent many graphs
- Renders graphs that look nice
- Fast enough to render large graphs
- Run as a CLI, rust library, python library, or in a web browser

### Packages:

- `graph_core`: The parser and layout engine
- `graph_cli`: A command line interface for rendering graphs
- `graph_api`: API for getting SVGs from graphs
- `graph_editor`: Web app for graph_core (TODO)
- `graph_py`: A python library for graph_core (TODO)

### Development

You will need the following tools to build and run this project:

- [just](https://github.com/casey/just) - A command runner for project specific commands
- [rust](https://www.rust-lang.org/tools/install) - The rust programming language
- [node](https://nodejs.org/en/download/) - The node.js runtime
- [pnpm](https://pnpm.io/installation) - A package manager for node.js

# TODO:

- [x] Render edges
- [ ] Render markers on edges
- [ ] Render edge types
- [ ] Make edges not overlap nodes
- [ ] Add colours to nodes and edges
- [ ] Think of a better name
- [ ] Maybe split parser and layout engine into separate crates?
