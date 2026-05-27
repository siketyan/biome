use crate::prelude::*;
use biome_formatter::format_args;
use biome_yaml_syntax::{AnyYamlProperty, YamlPropertyList};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlPropertyList;
impl FormatRule<YamlPropertyList> for FormatYamlPropertyList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlPropertyList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut join = f.join_with(space());

        for property in node
            .iter()
            .filter(|property| matches!(property, AnyYamlProperty::YamlTagProperty(_)))
        {
            join.entry(&format_args![property.format()]);
        }

        for property in node
            .iter()
            .filter(|property| matches!(property, AnyYamlProperty::YamlAnchorProperty(_)))
        {
            join.entry(&format_args![property.format()]);
        }

        join.finish()
    }
}
