use crate::prelude::*;
use biome_yaml_syntax::YamlDirectiveList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDirectiveList;
impl FormatRule<YamlDirectiveList> for FormatYamlDirectiveList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlDirectiveList, f: &mut YamlFormatter) -> FormatResult<()> {
        f.join_with(hard_line_break())
            .entries(node.iter().formatted())
            .finish()
    }
}
