pub fn language() -> tree_sitter::Language {
    tree_sitter_yaml::LANGUAGE.into()
}

pub const HIGHLIGHT_QUERY: &str = tree_sitter_yaml::HIGHLIGHTS_QUERY;
