use crate::prelude::*;
use biome_formatter::comments::{Comments, SourceComment};
use biome_formatter::trivia::format_dangling_comments_from_slice;
use biome_formatter::{CstFormatContext, format_args, write};
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, AnyYamlBlockNode, YamlBlockMapExplicitEntry, YamlLanguage,
    YamlSyntaxToken,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapExplicitEntry;
impl FormatNodeRule<YamlBlockMapExplicitEntry> for FormatYamlBlockMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let question_mark = node.question_mark_token()?;

        if let (Some(key), Some(colon_token), Some(value)) =
            (node.key(), node.colon_token(), node.value())
        {
            if f.context()
                .comments()
                .dangling_comments(node.syntax())
                .is_empty()
                && can_inline_explicit_entry(&key, &value)
            {
                write!(
                    f,
                    [
                        format_removed(&question_mark),
                        key.format(),
                        format_removed(&colon_token),
                        token(":"),
                        space(),
                        value.format()
                    ]
                )?;
                return Ok(());
            }
        }

        write!(f, [format_trimmed_token(&question_mark)])?;

        if let Some(key) = node.key() {
            write!(f, [space(), key.format()])?;
        }

        if let Some(colon_token) = node.colon_token() {
            let comments = f.context().comments().clone();
            let (before_colon, _) = dangling_comments_around_colon(node, &comments, &colon_token);
            if !before_colon.is_empty() {
                write!(
                    f,
                    [indent(&format_args![
                        hard_line_break(),
                        format_dangling_comments_from_slice(before_colon)
                    ])]
                )?;
            }

            write!(
                f,
                [hard_line_break(), format_removed(&colon_token), token(":")]
            )?;

            if let Some(value) = node.value() {
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
                        )?;
                    }
                    _ if is_block_collection(&value) => {
                        write!(
                            f,
                            [indent(&format_args![hard_line_break(), value.format()])]
                        )?;
                    }
                    _ => {
                        write!(f, [space(), value.format()])?;
                    }
                }
            }
        }

        Ok(())
    }

    fn fmt_dangling_comments(
        &self,
        node: &YamlBlockMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let Some(colon_token) = node.colon_token() else {
            return format_dangling_comments(node.syntax())
                .with_soft_block_indent()
                .fmt(f);
        };

        let comments = f.context().comments().clone();
        let (_, after_colon) = dangling_comments_around_colon(node, &comments, &colon_token);
        let needs_empty_line = has_blank_line_after(after_colon);

        write!(
            f,
            [format_dangling_comments_from_slice(after_colon).with_soft_block_indent()]
        )?;

        if needs_empty_line {
            write!(f, [empty_line()])?;
        }

        Ok(())
    }
}

fn can_inline_explicit_entry(key: &AnyYamlBlockNode, value: &AnyYamlBlockNode) -> bool {
    matches!(key, AnyYamlBlockNode::YamlFlowInBlockNode(_))
        && matches!(value, AnyYamlBlockNode::YamlFlowInBlockNode(_))
        && !key.clone().into_syntax().has_leading_comments()
        && !key.clone().into_syntax().has_trailing_comments()
        && !value.clone().into_syntax().has_leading_comments()
        && !value.clone().into_syntax().has_trailing_comments()
        && !key
            .clone()
            .into_syntax()
            .text_trimmed()
            .to_string()
            .contains(['\n', '\r'])
        && !value
            .clone()
            .into_syntax()
            .text_trimmed()
            .to_string()
            .contains(['\n', '\r'])
}

fn has_blank_line_after(comments: &[SourceComment<YamlLanguage>]) -> bool {
    comments
        .last()
        .is_some_and(|comment| comment.lines_after() > 1)
}

fn dangling_comments_around_colon<'a>(
    node: &YamlBlockMapExplicitEntry,
    comments: &'a Comments<YamlLanguage>,
    colon_token: &YamlSyntaxToken,
) -> (
    &'a [SourceComment<YamlLanguage>],
    &'a [SourceComment<YamlLanguage>],
) {
    let dangling = comments.dangling_comments(node.syntax());
    let colon_start = colon_token.text_trimmed_range().start();
    let after_colon_start =
        dangling.partition_point(|comment| comment.piece().text_range().end() <= colon_start);

    dangling.split_at(after_colon_start)
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
