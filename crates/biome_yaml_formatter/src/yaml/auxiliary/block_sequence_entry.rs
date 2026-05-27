use crate::prelude::*;
use biome_formatter::{CstFormatContext, format_args, write};
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, AnyYamlBlockNode, YamlBlockSequenceEntry,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequenceEntry;
impl FormatNodeRule<YamlBlockSequenceEntry> for FormatYamlBlockSequenceEntry {
    fn fmt_fields(&self, node: &YamlBlockSequenceEntry, f: &mut YamlFormatter) -> FormatResult<()> {
        write!(f, [format_trimmed_token(&node.minus_token()?)])?;

        if let Some(value) = node.value() {
            if is_block_scalar(&value) {
                write!(f, [space(), value.format()])?;
            } else {
                write!(f, [space(), align(2, &format_args![value.format()])])?;
            }
        }

        Ok(())
    }

    fn fmt_dangling_comments(
        &self,
        node: &YamlBlockSequenceEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        let dangling = comments.dangling_comments(node.syntax());
        let needs_leading_empty_line = dangling
            .first()
            .is_some_and(|comment| comment.lines_before() > 1);
        let needs_empty_line = dangling
            .last()
            .is_some_and(|comment| comment.lines_after() > 1);

        write!(
            f,
            [indent(&format_args![
                if needs_leading_empty_line {
                    empty_line()
                } else {
                    hard_line_break()
                },
                format_yaml_dangling_comments(dangling)
            ])]
        )?;

        if needs_empty_line {
            write!(f, [empty_line()])?;
        }

        Ok(())
    }
}

fn is_block_scalar(value: &AnyYamlBlockNode) -> bool {
    matches!(
        value,
        AnyYamlBlockNode::YamlBlockInBlockNode(node)
            if matches!(
                node.content(),
                Ok(
                    AnyYamlBlockInBlockContent::YamlFoldedScalar(_)
                        | AnyYamlBlockInBlockContent::YamlLiteralScalar(_)
                )
            )
    )
}
