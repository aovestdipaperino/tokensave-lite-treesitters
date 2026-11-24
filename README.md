# tokensave-lite-treesitters

[![GitHub stars](https://img.shields.io/github/stars/aovestdipaperino/tokensave-lite-treesitters?logo=github&color=brightgreen)](https://github.com/aovestdipaperino/tokensave-lite-treesitters/stargazers)

[![Crates.io](https://img.shields.io/crates/v/tokensave-lite-treesitters.svg)](https://crates.io/crates/tokensave-lite-treesitters)
[![docs.rs](https://docs.rs/tokensave-lite-treesitters/badge.svg)](https://docs.rs/tokensave-lite-treesitters)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

Essential tree-sitter grammars for [tokensave](https://github.com/aovestdipaperino/tokensave). This is the **lite** tier, bundling the 7 most common languages.

## Languages

| Language | Grammar Source |
|----------|---------------|
| Rust | [tree-sitter/tree-sitter-rust](https://github.com/tree-sitter/tree-sitter-rust) |
| Python | [tree-sitter/tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python) |
| JavaScript | [tree-sitter/tree-sitter-javascript](https://github.com/tree-sitter/tree-sitter-javascript) |
| TypeScript / TSX | [tree-sitter/tree-sitter-typescript](https://github.com/tree-sitter/tree-sitter-typescript) |
| Go | [tree-sitter/tree-sitter-go](https://github.com/tree-sitter/tree-sitter-go) |
| Java | [tree-sitter/tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java) |
| C | [tree-sitter/tree-sitter-c](https://github.com/tree-sitter/tree-sitter-c) |

## Usage

```rust
use tokensave_lite_treesitters::all_languages;

for (name, language) in all_languages() {
    println!("{name}");
}
```

## Tiers

- **lite** (this crate) -- 7 languages
- **medium** ([tokensave-medium-treesitters](https://crates.io/crates/tokensave-medium-treesitters)) -- 17 languages, includes lite
- **large** ([tokensave-large-treesitters](https://crates.io/crates/tokensave-large-treesitters)) -- 22+ languages, includes medium

## Pure-Rust grammars (branch `pure-rust-grammars`)

This branch swaps the seven C-based grammar crates for [`tree-sitter-lite-rs`](https://github.com/aovestdipaperino/tree-sitter-lite-rs): parse tables and lexers are transpiled from the released parsers by [c2rs](https://github.com/aovestdipaperino/c2rs) and the external scanners are hand-ported, so no grammar C source is compiled. The public API is unchanged (`all_languages()` and `languages::tree_sitter_*::LANGUAGE`). tokensave's full integration suite passes against it.
