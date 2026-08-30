pub fn language() -> tree_sitter::Language {
    tree_sitter_gdshader::LANGUAGE.into()
}

pub const HIGHLIGHT_QUERY: &str = tree_sitter_gdshader::HIGHLIGHTS_QUERY;
