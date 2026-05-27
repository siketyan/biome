use crate::prelude::*;
use biome_yaml_syntax::YamlBlockStripIndicator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockStripIndicator;
impl FormatNodeRule<YamlBlockStripIndicator> for FormatYamlBlockStripIndicator {
    fn fmt_fields(
        &self,
        node: &YamlBlockStripIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        node.minus_token().format().fmt(f)
    }
}
