use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlDocumentList, YamlSyntaxNode};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDocumentList;
impl FormatRule<YamlDocumentList> for FormatYamlDocumentList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlDocumentList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut previous_document: Option<YamlSyntaxNode> = None;

        for document in node.iter() {
            let has_blank_line_before = get_lines_before(document.syntax()) > 1
                || previous_document
                    .as_ref()
                    .is_some_and(|previous| {
                        get_lines_after(previous) > 1
                            || document.syntax().text_trimmed_range().start()
                                - previous.text_trimmed_range().end()
                                > 1.into()
                    });

            if previous_document.is_some() {
                write!(f, [hard_line_break()])?;
                if has_blank_line_before {
                    write!(f, [empty_line()])?;
                }
            }

            write!(f, [document.format()])?;

            previous_document = Some(document.into_syntax());
        }

        Ok(())
    }
}

fn get_lines_after(document: &YamlSyntaxNode) -> u32 {
    document
        .last_token()
        .map(|token| {
            token
                .trailing_trivia()
                .pieces()
                .filter(|piece| piece.is_newline())
                .count() as u32
        })
        .unwrap_or(0)
}
