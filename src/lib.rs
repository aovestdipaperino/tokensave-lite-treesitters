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
/// through `languages::tree_sitter_rust::LANGUAGE` keep working.
pub mod languages {
    macro_rules! grammar {
        ($name:ident, $module:ident) => {
            pub mod $name {
                pub use tree_sitter_lite_rs::$module::LANGUAGE;
            }
        };
    }
    grammar!(tree_sitter_c, c);
    grammar!(tree_sitter_go, go);
    grammar!(tree_sitter_java, java);
    grammar!(tree_sitter_javascript, javascript);
    grammar!(tree_sitter_python, python);
    grammar!(tree_sitter_rust, rust);

    pub mod tree_sitter_typescript {
        pub use tree_sitter_lite_rs::tsx::LANGUAGE as LANGUAGE_TSX;
        pub use tree_sitter_lite_rs::typescript::LANGUAGE_TYPESCRIPT;
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
}
