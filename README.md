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

> Put a picture here

### Goals of this project:

- Language that is easy to read and write by humans or machines
- Parser that can report errors clearly
- Expressive enough to represent many graphs
- Renders graphs that look nice
- Fast enough to render large graphs
- Portable enough to run as a CLI, rust library, python library, or in a web browser

### Packages:

- `graph-core`: The parser and layout engine
- `graph-cli`: A command line interface for rendering graphs
- `graph-py`: A python library for rendering graphs

### Development

You will need the following tools to build and run this project:

- [just](https://github.com/casey/just) - A command runner for project specific commands
- [rust](https://www.rust-lang.org/tools/install) - The rust programming language

# TODO:

- [x] Render edges
- [ ] Render markers on edges
- [ ] Render edge types
- [ ] Truncate edges to node size
- [ ] Add colours to nodes and edges
- [ ] Think of a better name
- [ ] Semantic release?
- [ ] Setup CI to build cli and python packages
- [ ] Add a python library
- [ ] Maybe split parser and layout engine into separate crates?
