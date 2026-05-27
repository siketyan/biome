use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::{
    AnyYamlFlowNode, AnyYamlJsonContent, AnyYamlMappingImplicitKey, YamlFlowMapImplicitEntry,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapImplicitEntry;
impl FormatNodeRule<YamlFlowMapImplicitEntry> for FormatYamlFlowMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let key = node.key();

        if let (Some(key), Some(colon_token), Some(value)) =
            (&key, node.colon_token(), node.value())
        {
            if key.as_yaml_alias_node().is_some() {
                write!(
                    f,
                    [
                        key.format(),
                        space(),
                        format_trimmed_token(&colon_token),
                        space(),
                        value.format()
                    ]
                )?;

                return Ok(());
            }

            if is_flow_collection_key(key) {
                write!(
                    f,
                    [
                        token("?"),
                        space(),
                        indent(&key.format()),
                        hard_line_break(),
                        format_trimmed_token(&colon_token),
                        space(),
                        indent(&value.format())
                    ]
                )?;

                return Ok(());
            }
        }

        write!(f, [key.format()])?;

        if let Some(colon_token) = node.colon_token() {
            write!(f, [format_trimmed_token(&colon_token)])?;

            if let Some(value) = node.value() {
                if key.is_some() && is_flow_collection(&value) {
                    write!(
                        f,
                        [group(&indent(&format_args![
                            soft_line_break_or_space(),
                            value.format()
                        ]))]
                    )?;
                } else {
                    write!(f, [space(), indent(&value.format())])?;
                }
            } else if key.is_none() {
                write!(f, [space()])?;
            }
        }

        Ok(())
    }
}

fn is_flow_collection(node: &AnyYamlFlowNode) -> bool {
    let Some(json) = node.as_yaml_flow_json_node() else {
        return false;
    };

    matches!(
        json.content(),
        Ok(AnyYamlJsonContent::YamlFlowMapping(_)) | Ok(AnyYamlJsonContent::YamlFlowSequence(_))
    )
}

fn is_flow_collection_key(node: &AnyYamlMappingImplicitKey) -> bool {
    let Some(json) = node.as_yaml_flow_json_node() else {
        return false;
    };

    matches!(
        json.content(),
        Ok(AnyYamlJsonContent::YamlFlowMapping(_)) | Ok(AnyYamlJsonContent::YamlFlowSequence(_))
    )
}
