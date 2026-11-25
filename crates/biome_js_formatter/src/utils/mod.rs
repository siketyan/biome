pub(crate) mod array;
mod assignment_like;
mod conditional;
mod format_binary_like_expression;
pub mod string_utils;

pub(crate) mod format_class;
pub(crate) mod format_modifiers;
pub(crate) mod format_node_without_comments;
pub(crate) mod function_body;
pub mod jsx;
pub(crate) mod member_chain;
pub(crate) mod object;
mod object_like;
mod object_pattern_like;
#[cfg(test)]
mod quickcheck_utils;
pub(crate) mod test_each_template;
mod typescript;

use crate::context::Semicolons;
use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
pub(crate) use assignment_like::{
    AnyJsAssignmentLike, AssignmentLikeLayout, with_assignment_layout,
};
use biome_formatter::{Buffer, format_args, write};
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsCallExpression, JsInitializerClause, JsLanguage, Modifier,
};
use biome_js_syntax::{JsSyntaxKind, JsSyntaxToken};
use biome_rowan::{AstNode, AstNodeList};
use biome_text_size::TextSize;
pub(crate) use conditional::{AnyJsConditional, ConditionalJsxChain};
pub(crate) use object_like::JsObjectLike;
pub(crate) use object_pattern_like::JsObjectPatternLike;
use smallvec::SmallVec;
pub(crate) use string_utils::*;
pub(crate) use typescript::{is_object_like_type, should_hug_type};

/// Tests if expression is a long curried call
///
/// ```javascript
/// `connect(a, b, c)(d)`
/// ```
pub(crate) fn is_long_curried_call(expression: Option<&JsCallExpression>) -> bool {
    if let Some(expression) = expression
        && let Some(parent_call) = expression.parent::<JsCallExpression>()
        && let (Ok(arguments), Ok(parent_arguments)) =
            (expression.arguments(), parent_call.arguments())
    {
        let is_callee = matches!(
            parent_call.syntax().kind(),
            JsSyntaxKind::JS_CALL_EXPRESSION | JsSyntaxKind::JS_NEW_EXPRESSION
        );
        return is_callee
            && arguments.args().len() > parent_arguments.args().len()
            && !parent_arguments.args().is_empty();
    }

    false
}

/// Utility function to format the separators of the nodes that belong to the unions
/// of [biome_js_syntax::TsAnyTypeMember].
///
/// We can have two kind of separators: `,`, `;` or ASI.
/// Because of how the grammar crafts the nodes, the parent will add the separator to the node.
/// So here, we create - on purpose - an empty node.
pub(crate) struct FormatTypeMemberSeparator<'a> {
    token: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatTypeMemberSeparator<'a> {
    pub fn new(token: Option<&'a JsSyntaxToken>) -> Self {
        Self { token }
    }
}

impl Format<JsFormatContext> for FormatTypeMemberSeparator<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(separator) = self.token {
            format_removed(separator).fmt(f)
        } else {
            Ok(())
        }
    }
}

/// Utility function to format the node [biome_js_syntax::JsInitializerClause]
pub(crate) struct FormatInitializerClause<'a> {
    initializer: Option<&'a JsInitializerClause>,
}

impl<'a> FormatInitializerClause<'a> {
    pub fn new(initializer: Option<&'a JsInitializerClause>) -> Self {
        Self { initializer }
    }
}

impl Format<JsFormatContext> for FormatInitializerClause<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(initializer) = self.initializer {
            write!(f, [space(), initializer.format()])
        } else {
            Ok(())
        }
    }
}

pub(crate) struct FormatInterpreterToken<'a> {
    token: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatInterpreterToken<'a> {
    pub fn new(interpreter_token: Option<&'a JsSyntaxToken>) -> Self {
        Self {
            token: interpreter_token,
        }
    }
}

impl Format<JsFormatContext> for FormatInterpreterToken<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(interpreter) = self.token {
            // Trim trailing spaces of the interpreter
            let interpreter_text = interpreter.text_trimmed();
            let trimmed_interpreter_text = interpreter_text.trim_end();
            let diff = (interpreter_text.len() - trimmed_interpreter_text.len()) as u32;
            let trimmed_range = interpreter
                .text_trimmed_range()
                .sub_end(TextSize::from(diff));

            // We replace the interpreter token by its trimmed version
            write!(
                f,
                [format_replaced(
                    interpreter,
                    &located_token_text(interpreter, trimmed_range)
                )]
            )?;

            match interpreter
                .next_token()
                .map_or(0, |next_token| get_lines_before_token(&next_token))
            {
                0 | 1 => write!(f, [hard_line_break()]),
                _ => write!(f, [empty_line()]),
            }
        } else {
            Ok(())
        }
    }
}

/// Formats the body of a statement where it can either be a single statement, an empty statement,
/// or a block statement.
pub(crate) struct FormatStatementBody<'a> {
    body: &'a AnyJsStatement,
    force_space: bool,
}

impl<'a> FormatStatementBody<'a> {
    pub fn new(body: &'a AnyJsStatement) -> Self {
        Self {
            body,
            force_space: false,
        }
    }

    /// Prevents that the consequent is formatted on its own line and indented by one level and
    /// instead gets separated by a space.
    pub fn with_forced_space(mut self, forced: bool) -> Self {
        self.force_space = forced;
        self
    }
}

impl Format<JsFormatContext> for FormatStatementBody<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use AnyJsStatement::*;

        if let JsEmptyStatement(empty) = &self.body {
            write!(f, [empty.format()])
        } else if matches!(&self.body, JsBlockStatement(_)) || self.force_space {
            write!(f, [space(), self.body.format()])
        } else {
            write!(
                f,
                [indent(&format_args![
                    soft_line_break_or_space(),
                    self.body.format()
                ])]
            )
        }
    }
}

/// This function consumes a list of modifiers and applies a predictable sorting.
pub(crate) fn sort_modifiers_by_precedence<List, Node>(list: &List) -> Vec<Node>
where
    Node: AstNode<Language = JsLanguage>,
    List: AstNodeList<Language = JsLanguage, Node = Node>,
    Modifier: for<'a> From<&'a Node>,
{
    let nodes = list.iter().collect::<Vec<Node>>();
    let mut order: SmallVec<[ModifierPosition; 8]> = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| ModifierPosition {
            precedence: Modifier::from(node),
            index,
        })
        .collect();

    if sort_by_precedence(&mut order) {
        order
            .into_iter()
            .map(|position| nodes[position.index].clone())
            .collect()
    } else {
        nodes
    }
}

/// The precedence of a modifier and its position in its list.
///
/// The unstable sort of the standard library orders equal elements according
/// to the size of the elements and to whether they are `Copy`. This type
/// takes two words and isn't `Copy`, like a modifier node, so sorting the
/// positions gives the order that sorting the nodes would give.
#[derive(Debug, Eq, PartialEq)]
struct ModifierPosition {
    precedence: Modifier,
    index: usize,
}

/// Sorts the positions of modifiers by precedence. Returns `false` if they
/// already were in that order.
///
/// It doesn't depend on the type of the modifiers, so the modifier lists
/// share a single copy of the sorting code.
fn sort_by_precedence(modifiers: &mut [ModifierPosition]) -> bool {
    if modifiers.is_sorted_by_key(|position| position.precedence) {
        return false;
    }

    modifiers.sort_unstable_by_key(|position| position.precedence);
    true
}

pub(crate) type FormatStatementSemicolon<'a> = FormatOptionalSemicolon<'a>;

/// Formats a semicolon in a position where it is optional (not needed to maintain syntactical correctness).
///
/// * Inserts a new semicolon if it is absent and [JsFormatOptions::semicolons] is [Semicolons::Always].
/// * Removes the semicolon if it is present and [JsFormatOptions::semicolons] is [Semicolons::AsNeeded].
pub(crate) struct FormatOptionalSemicolon<'a> {
    semicolon: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatOptionalSemicolon<'a> {
    pub(crate) fn new(semicolon: Option<&'a JsSyntaxToken>) -> Self {
        Self { semicolon }
    }
}

impl Format<JsFormatContext> for FormatOptionalSemicolon<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match f.options().semicolons() {
            Semicolons::Always => FormatSemicolon::new(self.semicolon).fmt(f),
            Semicolons::AsNeeded => match self.semicolon {
                None => Ok(()),
                Some(semicolon) => format_removed(semicolon).fmt(f),
            },
        }
    }
}

/// Format some code followed by an optional semicolon.
/// Performs semicolon insertion if it is missing in the input source, the [semicolons option](crate::JsFormatOptions::semicolons) is [Semicolons::Always], and the
/// preceding element isn't an bogus node
pub(crate) struct FormatSemicolon<'a> {
    semicolon: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatSemicolon<'a> {
    pub fn new(semicolon: Option<&'a JsSyntaxToken>) -> Self {
        Self { semicolon }
    }
}

impl Format<JsFormatContext> for FormatSemicolon<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self.semicolon {
            Some(semicolon) => semicolon.format().fmt(f),
            None => {
                let is_after_bogus =
                    f.elements()
                        .start_tag(TagKind::Verbatim)
                        .is_some_and(|signal| match signal {
                            Tag::StartVerbatim(kind) => kind.is_bogus(),
                            _ => unreachable!(),
                        });

                if !is_after_bogus {
                    write!(f, [token(";")])?;
                }

                Ok(())
            }
        }
    }
}

/// A call like expression is one of:
///
/// - [JsNewExpression]
/// - [JsImportCallExpression]
/// - [JsCallExpression]
pub(crate) fn is_call_like_expression(expression: &AnyJsExpression) -> bool {
    matches!(
        expression,
        AnyJsExpression::JsNewExpression(_)
            | AnyJsExpression::JsImportCallExpression(_)
            | AnyJsExpression::JsCallExpression(_)
    )
}

/// This function is in charge to format the call arguments.
pub(crate) fn write_arguments_multi_line<S: Format<JsFormatContext>, I>(
    separated: I,
    f: &mut JsFormatter,
) -> FormatResult<()>
where
    I: Iterator<Item = S>,
{
    let mut iterator = separated.peekable();
    let mut join_with = f.join_with(soft_line_break_or_space());

    while let Some(element) = iterator.next() {
        let last = iterator.peek().is_none();

        if last {
            join_with.entry(&format_args![&element, FormatTrailingCommas::All]);
        } else {
            join_with.entry(&element);
        }
    }

    join_with.finish()
}

#[cfg(test)]
mod tests {
    use super::{ModifierPosition, sort_by_precedence, sort_modifiers_by_precedence};
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_syntax::{AnyJsPropertyModifier, JsPropertyModifierList, Modifier};
    use biome_languages::JsFileSource;
    use biome_rowan::{AstNode, AstNodeList, TextRange};

    fn positions<const N: usize>(precedences: [Modifier; N]) -> [ModifierPosition; N] {
        let mut index = 0;
        precedences.map(|precedence| {
            index += 1;
            ModifierPosition {
                precedence,
                index: index - 1,
            }
        })
    }

    fn indexes<const N: usize>(positions: [ModifierPosition; N]) -> [usize; N] {
        positions.map(|position| position.index)
    }

    fn ranges(nodes: &[AnyJsPropertyModifier]) -> Vec<TextRange> {
        nodes
            .iter()
            .map(|node| node.syntax().text_trimmed_range())
            .collect()
    }

    /// A small linear congruential generator, to keep the test deterministic.
    struct Random(u32);

    impl Random {
        fn below(&mut self, bound: usize) -> usize {
            self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (self.0 >> 16) as usize % bound
        }
    }

    #[test]
    fn a_modifier_position_has_the_layout_of_a_modifier_node() {
        assert_eq!(
            size_of::<ModifierPosition>(),
            size_of::<AnyJsPropertyModifier>()
        );
    }

    #[test]
    fn modifiers_in_order_of_precedence_are_left_alone() {
        assert!(!sort_by_precedence(&mut []));
        assert!(!sort_by_precedence(&mut positions([Modifier::Static])));

        let mut modifiers = positions([
            Modifier::Decorator,
            Modifier::Decorator,
            Modifier::Public,
            Modifier::Readonly,
        ]);
        assert!(!sort_by_precedence(&mut modifiers));
        assert_eq!(indexes(modifiers), [0, 1, 2, 3]);
    }

    #[test]
    fn modifiers_are_ordered_by_precedence_and_ties_keep_their_order() {
        let mut modifiers = positions([
            Modifier::Readonly,
            Modifier::Decorator,
            Modifier::Static,
            Modifier::Decorator,
            Modifier::Public,
        ]);

        assert!(sort_by_precedence(&mut modifiers));
        assert_eq!(indexes(modifiers), [1, 3, 4, 2, 0]);
    }

    /// Sorting the positions gives the order that sorting the modifier nodes
    /// themselves gives, also for the long lists that the standard library
    /// doesn't sort with an insertion sort.
    #[test]
    fn modifiers_are_ordered_like_their_nodes_would_be() {
        const MODIFIERS: [&str; 6] = ["static", "readonly", "public", "override", "@a", "@b()"];

        let mut random = Random(0x2545_f491);
        for len in [0, 1, 2, 5, 19, 20, 21, 22, 31, 32, 33, 47, 64, 100, 257] {
            for _ in 0..8 {
                let modifiers: Vec<_> = (0..len)
                    .map(|_| MODIFIERS[random.below(MODIFIERS.len())])
                    .collect();
                let source = format!("class A {{ {} property = 1; }}", modifiers.join(" "));
                let root = parse(&source, JsFileSource::ts(), JsParserOptions::default()).syntax();
                let list = root
                    .descendants()
                    .find_map(JsPropertyModifierList::cast)
                    .unwrap_or_else(|| panic!("no property modifier list in {source}"));
                assert_eq!(list.len(), len, "{source}");

                let mut expected: Vec<_> = list.iter().collect();
                expected.sort_unstable_by_key(|node| Modifier::from(node));

                let sorted = sort_modifiers_by_precedence(&list);
                assert_eq!(ranges(&sorted), ranges(&expected), "{source}");
            }
        }
    }
}
