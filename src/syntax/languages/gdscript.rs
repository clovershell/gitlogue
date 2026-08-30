pub fn language() -> tree_sitter::Language {
    tree_sitter_gdscript::LANGUAGE.into()
}

pub const HIGHLIGHT_QUERY: &str = include_str!("queries/gdscript_highlights.scm");
