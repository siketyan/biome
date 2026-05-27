use crate::prelude::*;
use biome_formatter::{CstFormatContext, write};
use biome_yaml_syntax::YamlBlockSequence;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequence;
impl FormatNodeRule<YamlBlockSequence> for FormatYamlBlockSequence {
    fn fmt_fields(&self, node: &YamlBlockSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        write!(
            f,
            [
                format_removed(&node.sequence_start_token()?),
                node.entries().format(),
                format_removed(&node.sequence_end_token()?)
            ]
        )
    }

    fn fmt_trailing_comments(
        &self,
        node: &YamlBlockSequence,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        let trailing = comments.trailing_comments(node.syntax());
        let needs_empty_line = trailing
            .last()
            .is_some_and(|comment| comment.lines_after() > 1);

        write!(f, [format_trailing_comments(node.syntax())])?;

        if needs_empty_line {
            write!(f, [empty_line()])?;
        }

        Ok(())
    }
}
