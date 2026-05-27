use crate::prelude::*;
use biome_yaml_syntax::YamlAnchorProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlAnchorProperty;
impl FormatNodeRule<YamlAnchorProperty> for FormatYamlAnchorProperty {
    fn fmt_fields(&self, node: &YamlAnchorProperty, f: &mut YamlFormatter) -> FormatResult<()> {
        format_trimmed_token(&node.value_token()?).fmt(f)
    }
}
