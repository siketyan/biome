use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::YamlFlowMapping;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapping;
impl FormatNodeRule<YamlFlowMapping> for FormatYamlFlowMapping {
    fn fmt_fields(&self, node: &YamlFlowMapping, f: &mut YamlFormatter) -> FormatResult<()> {
        let entries = node.entries();

        if entries.is_empty() {
            return write!(
                f,
                [
                    format_trimmed_token(&node.l_curly_token()?),
                    format_trimmed_token(&node.r_curly_token()?)
                ]
            );
        }

        write!(
            f,
            [group(&format_args![
                format_trimmed_token(&node.l_curly_token()?),
                indent(&format_args![soft_line_break_or_space(), entries.format()]),
                soft_line_break_or_space(),
                format_trimmed_token(&node.r_curly_token()?)
            ])]
        )
    }
}
