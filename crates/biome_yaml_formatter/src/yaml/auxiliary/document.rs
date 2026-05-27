use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlDocument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDocument;
impl FormatNodeRule<YamlDocument> for FormatYamlDocument {
    fn fmt_fields(&self, node: &YamlDocument, f: &mut YamlFormatter) -> FormatResult<()> {
        let directives = node.directives();
        write!(f, [node.bom_token().format()])?;

        if !directives.is_empty() {
            write!(f, [directives.format(), hard_line_break()])?;
        }

        if let Some(dashdashdash_token) = node.dashdashdash_token() {
            write!(f, [dashdashdash_token.format()])?;
            if node.node().is_some() {
                write!(f, [hard_line_break()])?;
            }
        }

        write!(f, [node.node().format()])?;

        if let Some(dotdotdot_token) = node.dotdotdot_token() {
            if dotdotdot_token.has_trailing_comments()
                || dotdotdot_token.has_leading_comments()
                || is_followed_by_bare_document(node)
            {
                if node.node().is_some() {
                    write!(f, [hard_line_break()])?;
                }
                write!(f, [dotdotdot_token.format()])?;
            } else {
                write!(f, [format_removed(&dotdotdot_token)])?;
            }
        }

        Ok(())
    }
}

fn is_followed_by_bare_document(node: &YamlDocument) -> bool {
    node.syntax()
        .next_sibling()
        .and_then(YamlDocument::cast)
        .is_some_and(|next| next.dashdashdash_token().is_none() && next.node().is_some())
}
