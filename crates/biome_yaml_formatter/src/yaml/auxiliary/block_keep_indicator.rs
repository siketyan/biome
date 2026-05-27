use crate::prelude::*;
use biome_yaml_syntax::YamlBlockKeepIndicator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockKeepIndicator;
impl FormatNodeRule<YamlBlockKeepIndicator> for FormatYamlBlockKeepIndicator {
    fn fmt_fields(&self, node: &YamlBlockKeepIndicator, f: &mut YamlFormatter) -> FormatResult<()> {
        node.plus_token().format().fmt(f)
    }
}
