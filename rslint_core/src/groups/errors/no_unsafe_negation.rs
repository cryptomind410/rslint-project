use crate::rule_prelude::*;
use ast::{BinExpr, BinOp, Expr, UnaryOp};
use rslint_parser::{TextRange, TextSize};
use SyntaxKind::*;

declare_lint! {
    /**
    Deny the use of `!` on the left hand side of an `instanceof` or `in` expression where it is ambiguous.

    JavaScript precedence is higher for logical not than it is for in or instanceof. Oftentimes you see
    expressions such as `!foo instanceof bar`, which most of the times produces unexpected behavior. 
    precedence will group the expressions like `(!foo) instanceof bar`. Most of the times the developer expects
    the expression to check if `foo` is not an instance of `bar` however.
    
    ## Incorrect Code Examples

    ```ignore
    if (!foo instanceof String) {

    }
    ```

    ```ignore
    if (!bar in {}) {

    }
    ```
    */
    #[derive(Default)]
    NoUnsafeNegation,
    errors,
    "no-unsafe-negation"
}

impl CstRule for NoUnsafeNegation {
    fn check_node(&self, node: &SyntaxNode, ctx: &mut RuleCtx) -> Option<()> {
        if node.kind() == BIN_EXPR
            && matches!(node.to::<BinExpr>().op()?, BinOp::Instanceof | BinOp::In)
        {
            let expr = node.to::<BinExpr>();

            if let Expr::UnaryExpr(unary) = expr.lhs()? {
                if unary.op()? == UnaryOp::LogicalNot {
                    let unary_node = unary.expr()?.syntax().clone();
                    let no_op_text = &node.trimmed_text().to_string()[1..];
                    let mut eq_expr = format!("(!{}", no_op_text);
                    eq_expr.insert(usize::from(unary_node.trimmed_text().len()) + 2, ')');
                    let rest_range = TextRange::new(node.trimmed_range().start() + TextSize::from(1), node.trimmed_range().end());
                    
                    let err = ctx
                        .err(
                            self.name(),
                            "Unsafe negation of a value in a binary expression",
                        )
                        .primary(
                            unary.op_token().unwrap().text_range(),
                            format!(
                                "precedence makes this expression equivalent to `{}`",
                                eq_expr
                            ),
                        )
                        .secondary(rest_range, "`!` is not negating this expression")
                        .note(format!(
                            "help: try this: `!({})`",
                            color(&no_op_text.to_string())
                        ));

                    ctx.add_err(err);
                }
            }
        }
        None
    }
}

rule_tests! {
    NoUnsafeNegation::default(),
    err: {
        "!foo in bar",
        "![5] instanceof !4",
        ;/// ignore
        "!!!!!instanceof !!foo instanceof !!bar"
    },
    ok: {
        ;/// If this is intended behavior, you can wrap the expression
        "(!foo) instanceof bar",
        "key in bar",
        "bar instanceof bar",
        ;/// ignore
        "1 in [1, 1, 1, ((!1) in [1111111111, 111])]"
    }
}
