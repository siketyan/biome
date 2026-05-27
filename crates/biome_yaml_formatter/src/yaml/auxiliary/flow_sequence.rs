use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::YamlFlowSequence;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequence;
impl FormatNodeRule<YamlFlowSequence> for FormatYamlFlowSequence {
    fn fmt_fields(&self, node: &YamlFlowSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        let entries = node.entries();

        if entries.is_empty() {
            return write!(
                f,
                [
                    format_trimmed_token(&node.l_brack_token()?),
                    format_trimmed_token(&node.r_brack_token()?)
                ]
            );
        }

        write!(
            f,
            [group(&format_args![
                format_trimmed_token(&node.l_brack_token()?),
                indent(&format_args![soft_line_break(), entries.format()]),
                soft_line_break(),
                format_trimmed_token(&node.r_brack_token()?)
            ])]
        )
    }
}
