use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{
    AnyYamlJsonContent, AnyYamlMappingImplicitKey, YamlFlowMapExplicitEntry, YamlFlowMapping,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapExplicitEntry;
impl FormatNodeRule<YamlFlowMapExplicitEntry> for FormatYamlFlowMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        if node.colon_token().is_none() && is_entry_in_flow_mapping(node) {
            write!(
                f,
                [
                    format_removed(&node.question_mark_token()?),
                    node.key().format()
                ]
            )?;
            return Ok(());
        }

        write!(f, [format_trimmed_token(&node.question_mark_token()?)])?;

        if let Some(key) = node.key() {
            write!(f, [space(), indent(&key.format())])?;

            if is_flow_collection(&key) {
                if let (Some(colon_token), Some(value)) = (node.colon_token(), node.value()) {
                    write!(
                        f,
                        [
                            hard_line_break(),
                            format_trimmed_token(&colon_token),
                            space(),
                            indent(&value.format())
                        ]
                    )?;

                    return Ok(());
                }
            }
        }

        if let Some(colon_token) = node.colon_token() {
            write!(f, [space(), format_trimmed_token(&colon_token)])?;

            if let Some(value) = node.value() {
                write!(f, [space(), indent(&value.format())])?;
            }
        }

        Ok(())
    }
}

fn is_entry_in_flow_mapping(node: &YamlFlowMapExplicitEntry) -> bool {
    node.syntax()
        .parent()
        .and_then(|list| list.parent())
        .and_then(YamlFlowMapping::cast)
        .is_some()
}

fn is_flow_collection(node: &AnyYamlMappingImplicitKey) -> bool {
    let Some(json) = node.as_yaml_flow_json_node() else {
        return false;
    };

    matches!(
        json.content(),
        Ok(AnyYamlJsonContent::YamlFlowMapping(_)) | Ok(AnyYamlJsonContent::YamlFlowSequence(_))
    )
}
