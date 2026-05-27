use crate::prelude::*;
use biome_yaml_syntax::YamlDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDirective;
impl FormatNodeRule<YamlDirective> for FormatYamlDirective {
    fn fmt_fields(&self, node: &YamlDirective, f: &mut YamlFormatter) -> FormatResult<()> {
        format_trimmed_token(&node.value_token()?).fmt(f)
    }
}
