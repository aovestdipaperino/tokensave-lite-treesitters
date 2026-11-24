//! Essential tree-sitter grammars for tokensave.
//!
//! Tier: **lite** — 7 core languages.
//!
//! Languages: Rust, Python, JavaScript, TypeScript, Go, Java, C
//!
//! The grammars are pure Rust (`tree-sitter-lite-rs`): parse tables and lexers are transpiled from
//! the upstream generated parsers and the external scanners are ported by hand, so no grammar C
//! source is compiled.

pub use tree_sitter;

/// Modules named and shaped like the upstream `tree-sitter-*` crates, so callers that reach
/// through `languages::tree_sitter_rust::LANGUAGE` (or `NODE_TYPES`, or a query constant) keep working.
pub mod languages {
    pub mod tree_sitter_c {
        pub use tree_sitter_lite_rs::c::{HIGHLIGHT_QUERY, LANGUAGE, NODE_TYPES, TAGS_QUERY};
    }
    pub mod tree_sitter_go {
        pub use tree_sitter_lite_rs::go::{HIGHLIGHTS_QUERY, LANGUAGE, NODE_TYPES, TAGS_QUERY};
    }
    pub mod tree_sitter_java {
        pub use tree_sitter_lite_rs::java::{HIGHLIGHTS_QUERY, LANGUAGE, NODE_TYPES, TAGS_QUERY};
    }
    pub mod tree_sitter_javascript {
        pub use tree_sitter_lite_rs::javascript::{
            HIGHLIGHT_QUERY, INJECTIONS_QUERY, JSX_HIGHLIGHT_QUERY, LANGUAGE, LOCALS_QUERY,
            NODE_TYPES, TAGS_QUERY,
        };
    }
    pub mod tree_sitter_python {
        pub use tree_sitter_lite_rs::python::{HIGHLIGHTS_QUERY, LANGUAGE, NODE_TYPES, TAGS_QUERY};
    }
    pub mod tree_sitter_rust {
        pub use tree_sitter_lite_rs::rust::{
            HIGHLIGHTS_QUERY, INJECTIONS_QUERY, LANGUAGE, NODE_TYPES, TAGS_QUERY,
        };
    }
    pub mod tree_sitter_typescript {
        pub use tree_sitter_lite_rs::tsx::{LANGUAGE_TSX, TSX_NODE_TYPES};
        pub use tree_sitter_lite_rs::typescript::{
            HIGHLIGHTS_QUERY, LANGUAGE_TYPESCRIPT, LOCALS_QUERY, TAGS_QUERY, TYPESCRIPT_NODE_TYPES,
        };
    }
}

/// Returns (name, language_fn) pairs for all lite-tier languages.
pub fn all_languages() -> Vec<(&'static str, tree_sitter_language::LanguageFn)> {
    vec![
        ("rust", languages::tree_sitter_rust::LANGUAGE),
        ("python", languages::tree_sitter_python::LANGUAGE),
        ("javascript", languages::tree_sitter_javascript::LANGUAGE),
        (
            "typescript",
            languages::tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        ),
        ("go", languages::tree_sitter_go::LANGUAGE),
        ("java", languages::tree_sitter_java::LANGUAGE),
        ("c", languages::tree_sitter_c::LANGUAGE),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_seven_languages_parse() {
        let langs = all_languages();
        let names: Vec<_> = langs.iter().map(|(n, _)| *n).collect();
        assert_eq!(
            names,
            [
                "rust",
                "python",
                "javascript",
                "typescript",
                "go",
                "java",
                "c"
            ]
        );
        let samples = [
            "fn main() {}",
            "def f():\n    pass\n",
            "const a = 1;",
            "let a: number = 1;",
            "package main\nfunc main() {}",
            "class A {}",
            "int main(void) { return 0; }",
        ];
        for ((name, lang), src) in langs.into_iter().zip(samples) {
            let mut p = tree_sitter::Parser::new();
            p.set_language(&lang.into()).unwrap();
            let tree = p.parse(src, None).unwrap();
            assert!(!tree.root_node().has_error(), "{name}: {src:?}");
        }
    }

    #[test]
    fn tsx_is_reachable_through_the_typescript_module() {
        let mut p = tree_sitter::Parser::new();
        p.set_language(&languages::tree_sitter_typescript::LANGUAGE_TSX.into())
            .unwrap();
        assert!(
            !p.parse("const e = <a>{b}</a>;", None)
                .unwrap()
                .root_node()
                .has_error()
        );
    }

    #[test]
    fn upstream_constants_come_through_the_shim() {
        use languages::*;
        for (name, data) in [
            ("c", tree_sitter_c::NODE_TYPES),
            ("go", tree_sitter_go::NODE_TYPES),
            ("java", tree_sitter_java::NODE_TYPES),
            ("javascript", tree_sitter_javascript::NODE_TYPES),
            ("python", tree_sitter_python::NODE_TYPES),
            ("rust", tree_sitter_rust::NODE_TYPES),
            ("typescript", tree_sitter_typescript::TYPESCRIPT_NODE_TYPES),
            ("tsx", tree_sitter_typescript::TSX_NODE_TYPES),
        ] {
            assert!(data.trim_start().starts_with('['), "{name}: node types");
        }
        // a query constant compiles against its grammar through the shim
        let lang: tree_sitter::Language = tree_sitter_rust::LANGUAGE.into();
        tree_sitter::Query::new(&lang, tree_sitter_rust::HIGHLIGHTS_QUERY).unwrap();
        let ts: tree_sitter::Language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
        tree_sitter::Query::new(&ts, tree_sitter_typescript::TAGS_QUERY).unwrap();
    }
}
