# Schemata for Zed

Language support for [Schemata](https://github.com/msbolton/Schemata) `.schemata` files in
[Zed](https://zed.dev): highlighting, the outline, and the Schemata language server, which gives
diagnostics as you type, go to definition, hover, find references, rename, and formatting.

## Install

1. Install the compiler, version 2.0.0 or later: `brew install msbolton/schemata/schemata`, or a
   binary from the [releases](https://github.com/msbolton/Schemata/releases). The language server
   is the `schemata lsp` command of that binary.
2. Clone this repository.
3. In Zed, run "zed: install dev extension" from the command palette and choose the clone. Zed
   builds the extension; this needs Rust installed through [rustup](https://rustup.rs).

## Settings

In Zed's `settings.json`:

```json
{
  "lsp": {
    "schemata": {
      "binary": { "path": "/path/to/schemata" },
      "initialization_options": { "roots": ["model"], "strict": true }
    }
  }
}
```

| Setting | Meaning |
|---|---|
| `binary.path` | The binary to run. Without it, `schemata` is looked up on `PATH`. |
| `initialization_options.roots` | Directories, relative to the project, whose whole subtree is one schema set. Without roots, each directory is its own set. |
| `initialization_options.strict` | Report implicit ordinals as errors. |
| `settings` | `lsp.schemata.settings` takes the same `roots` and `strict` and reaches the server without a restart. |

Zed formats `.schemata` files on save through the server by default; to turn that off, set
`"languages": { "Schemata": { "format_on_save": "off" } }`.

If the server does not start, open Zed's log ("zed: open log"). A compiler older than 0.8.0 has
no `lsp` command and exits at once. This extension highlights the 2.0 syntax; a 1.x compiler reports
every 2.0 file as a syntax error, and `schemata upgrade` rewrites a 1.x file to 2.0.

The compiler's guide describes what the server does, how schema sets work, and what happens
while a file does not parse.

## Develop

    cargo test                                    # the command-choice logic
    cargo build --target wasm32-wasip2 --release  # what Zed builds
    scripts/check-queries                         # the queries against the pinned grammar

The grammar lives in [tree-sitter-schemata](https://github.com/msbolton/tree-sitter-schemata);
`extension.toml` pins it by commit.

## Publishing to Zed's extension registry

Not done yet. It takes a pull request to
[zed-industries/extensions](https://github.com/zed-industries/extensions) that adds this
repository as a submodule under `extensions/schemata` and an entry in `extensions.toml` with the
version from `extension.toml`.
