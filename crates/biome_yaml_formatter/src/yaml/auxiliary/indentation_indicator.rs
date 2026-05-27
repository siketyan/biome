use crate::prelude::*;
use biome_yaml_syntax::YamlIndentationIndicator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlIndentationIndicator;
impl FormatNodeRule<YamlIndentationIndicator> for FormatYamlIndentationIndicator {
    fn fmt_fields(
        &self,
        node: &YamlIndentationIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        format_trimmed_token(&node.indentation_indicator_token()?).fmt(f)
    }
}
