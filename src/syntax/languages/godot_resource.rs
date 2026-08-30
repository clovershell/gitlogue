pub fn language() -> tree_sitter::Language {
    tree_sitter_godot_resource::LANGUAGE.into()
}

pub const HIGHLIGHT_QUERY: &str = include_str!("queries/godot_resource_highlights.scm");
