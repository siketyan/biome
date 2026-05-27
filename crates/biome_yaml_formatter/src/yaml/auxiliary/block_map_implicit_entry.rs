use crate::prelude::*;
use biome_formatter::comments::{Comments, SourceComment};
use biome_formatter::trivia::format_dangling_comments_from_slice;
use biome_formatter::{CstFormatContext, FormatOptions, format_args, write};
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, AnyYamlBlockNode, AnyYamlJsonContent, AnyYamlMappingImplicitKey,
    YamlBlockMapImplicitEntry, YamlLanguage,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapImplicitEntry;
impl FormatNodeRule<YamlBlockMapImplicitEntry> for FormatYamlBlockMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let key = node.key();
        write!(f, [key.format()])?;

        if matches!(key, Some(AnyYamlMappingImplicitKey::YamlAliasNode(_))) {
            write!(f, [space()])?;
        }

        write!(f, [format_trimmed_token(&node.colon_token()?)])?;

        if let Some(value) = node.value() {
            let comments = f.context().comments().clone();
            let (before_value, _) = dangling_comments_around_value(node, &comments, &value);
            if !before_value.is_empty() {
                write!(
                    f,
                    [
                        space(),
                        format_dangling_comments_from_slice(before_value),
                        indent(&format_args![hard_line_break(), value.format()])
                    ]
                )?;
                return Ok(());
            }

            if should_break_flow_collection(&value, f) {
                write!(f, [indent(&format_args![hard_line_break(), value.format()])])?;
                return Ok(());
            }

            match &value {
                AnyYamlBlockNode::YamlBlockInBlockNode(block)
                    if is_block_collection(&value) && !block.properties().is_empty() =>
                {
                    f.context()
                        .comments()
                        .mark_suppression_checked(block.syntax());
                    write!(
                        f,
                        [
                            space(),
                            block.properties().format(),
                            indent(&format_args![hard_line_break(), block.content().format()])
                        ]
                    )
                }
                _ if is_block_collection(&value) => {
                    write!(
                        f,
                        [indent(&format_args![hard_line_break(), value.format()])]
                    )
                }
                _ if is_block_scalar(&value) => write!(f, [space(), value.format()]),
                _ => write!(f, [space(), align(2, &format_args![value.format()])]),
            }
        } else {
            let comments = f.context().comments().clone();
            let dangling = comments.dangling_comments(node.syntax());
            if !dangling.is_empty() {
                write!(
                    f,
                    [indent(&format_args![
                        hard_line_break(),
                        format_dangling_comments_from_slice(dangling)
                    ])]
                )?;
            }

            Ok(())
        }
    }

    fn fmt_dangling_comments(
        &self,
        node: &YamlBlockMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let Some(value) = node.value() else {
            return Ok(());
        };

        let comments = f.context().comments().clone();
        let (_, after_value) = dangling_comments_around_value(node, &comments, &value);
        let needs_leading_empty_line = after_value
            .first()
            .is_some_and(|comment| comment.lines_before() > 1);
        let needs_empty_line = has_blank_line_after(after_value);

        write!(
            f,
            [indent(&format_args![
                if needs_leading_empty_line {
                    empty_line()
                } else {
                    hard_line_break()
                },
                format_yaml_dangling_comments(after_value)
            ])]
        )?;

        if needs_empty_line {
            write!(f, [empty_line()])?;
        }

        Ok(())
    }
}

fn has_blank_line_after(comments: &[SourceComment<YamlLanguage>]) -> bool {
    comments
        .last()
        .is_some_and(|comment| comment.lines_after() > 1)
}

fn dangling_comments_around_value<'a>(
    node: &YamlBlockMapImplicitEntry,
    comments: &'a Comments<YamlLanguage>,
    value: &AnyYamlBlockNode,
) -> (
    &'a [SourceComment<YamlLanguage>],
    &'a [SourceComment<YamlLanguage>],
) {
    let dangling = comments.dangling_comments(node.syntax());
    let value_start = value.clone().into_syntax().text_trimmed_range().start();
    let after_before_value =
        dangling.partition_point(|comment| comment.piece().text_range().end() <= value_start);

    dangling.split_at(after_before_value)
}

fn is_block_collection(value: &AnyYamlBlockNode) -> bool {
    matches!(
        value,
        AnyYamlBlockNode::YamlBlockInBlockNode(node)
            if matches!(
                node.content(),
                Ok(
                    AnyYamlBlockInBlockContent::YamlBlockMapping(_)
                        | AnyYamlBlockInBlockContent::YamlBlockSequence(_)
                )
            )
    )
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

fn should_break_flow_collection(value: &AnyYamlBlockNode, f: &YamlFormatter) -> bool {
    let AnyYamlBlockNode::YamlFlowInBlockNode(node) = value else {
        return false;
    };

    let text_len = usize::from(value.clone().into_syntax().text_trimmed_range().len());

    matches!(
        node.flow(),
        Ok(flow)
            if matches!(
                flow.as_yaml_flow_json_node().and_then(|node| node.content().ok()),
                Some(
                    AnyYamlJsonContent::YamlFlowMapping(_)
                        | AnyYamlJsonContent::YamlFlowSequence(_)
                )
            )
                && text_len > usize::from(f.options().line_width().value())
                && node.syntax().tokens().any(|token| {
                    token.leading_trivia().pieces().any(|piece| piece.is_newline())
                        || token
                            .trailing_trivia()
                            .pieces()
                            .any(|piece| piece.is_newline())
                })
    )
}
