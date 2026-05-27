use crate::prelude::*;
use biome_yaml_syntax::YamlTagProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlTagProperty;
impl FormatNodeRule<YamlTagProperty> for FormatYamlTagProperty {
    fn fmt_fields(&self, node: &YamlTagProperty, f: &mut YamlFormatter) -> FormatResult<()> {
        format_trimmed_token(&node.value_token()?).fmt(f)
    }
}
