//! Essential tree-sitter grammars for tokensave.
//!
//! Tier: **lite** — 7 core languages.
//!
//! Languages: Rust, Python, JavaScript, TypeScript, Go, Java, C

pub use tree_sitter;

pub mod languages {
    pub use tree_sitter_c;
    pub use tree_sitter_go;
    pub use tree_sitter_java;
    pub use tree_sitter_javascript;
    pub use tree_sitter_python;
    pub use tree_sitter_rust;
    pub use tree_sitter_typescript;
}

/// Returns (name, language_fn) pairs for all lite-tier languages.
pub fn all_languages() -> Vec<(&'static str, tree_sitter_language::LanguageFn)> {
    vec![
        ("rust", tree_sitter_rust::LANGUAGE),
        ("python", tree_sitter_python::LANGUAGE),
        ("javascript", tree_sitter_javascript::LANGUAGE),
        ("typescript", tree_sitter_typescript::LANGUAGE_TYPESCRIPT),
        ("go", tree_sitter_go::LANGUAGE),
        ("java", tree_sitter_java::LANGUAGE),
        ("c", tree_sitter_c::LANGUAGE),
    ]
}
