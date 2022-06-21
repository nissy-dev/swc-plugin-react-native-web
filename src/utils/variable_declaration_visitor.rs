use swc_plugin::ast::*;
use swc_plugin::syntax_pos::DUMMY_SP;

use super::get_dist_location::get_dist_location;

pub fn is_react_native_require(node: &VarDeclarator) -> bool {
    if let Some(expr) = &node.init {
        if let Expr::Call(call) = &**expr {
            if is_require_expr(&call) && is_react_native_args(&call) {
                return true;
            }
        }
    }
    return false;
}

pub fn create_new_variable_decl(
    node: &VarDeclarator,
    common_js: bool,
) -> Option<Vec<VarDeclarator>> {
    match &node.name {
        Pat::Ident(ident) => {
            let callee = get_callee_variable_decl(node).unwrap();
            let dist_location = get_dist_location(&ident.id.sym, common_js);
            Some(vec![VarDeclarator {
                span: DUMMY_SP,
                name: node.clone().name,
                init: Some(Box::new(Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: callee.clone(),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: DUMMY_SP,
                            value: dist_location.into(),
                            raw: None,
                        }))),
                    }],
                    type_args: Default::default(),
                }))),
                definite: Default::default(),
            }])
        }
        Pat::Object(object_pat) => {
            let new_variable_decls = object_pat
                .props
                .clone()
                .into_iter()
                .filter_map(|prop| {
                    if let ObjectPatProp::Assign(assign) = prop {
                        let callee = get_callee_variable_decl(&node).unwrap();
                        let dist_location = get_dist_location(&assign.key.sym, common_js);
                        Some(VarDeclarator {
                            span: DUMMY_SP,
                            name: Pat::Ident(BindingIdent {
                                id: assign.key.clone(),
                                type_ann: Default::default(),
                            }),
                            init: Some(Box::new(Expr::Member(MemberExpr {
                                span: DUMMY_SP,
                                obj: Box::new(Expr::Call(CallExpr {
                                    span: DUMMY_SP,
                                    callee: callee.clone(),
                                    args: vec![ExprOrSpread {
                                        spread: None,
                                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                                            span: DUMMY_SP,
                                            value: dist_location.into(),
                                            raw: None,
                                        }))),
                                    }],
                                    type_args: Default::default(),
                                })),
                                prop: MemberProp::Ident(Ident {
                                    span: DUMMY_SP,
                                    sym: "default".into(),
                                    optional: Default::default(),
                                }),
                            }))),
                            definite: Default::default(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            Some(new_variable_decls)
        }
        _ => None,
    }
}

fn is_require_expr(call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Ident(ident) = &**expr {
            if &*ident.sym == "require" {
                return true;
            }
        }
    }
    return false;
}

fn is_react_native_args(call_expr: &CallExpr) -> bool {
    if call_expr.args.len() == 1 {
        let arg = &call_expr.args[0];
        if let Expr::Lit(lit) = &*arg.expr {
            if let Lit::Str(str_lit) = &*lit {
                if &*str_lit.value == "react-native" || &*str_lit.value == "react-native-web" {
                    return true;
                }
            }
        }
    }
    return false;
}

fn get_callee_variable_decl(node: &VarDeclarator) -> Option<Callee> {
    if let Some(expr) = &node.init {
        if let Expr::Call(call) = &**expr {
            return Some(call.callee.clone());
        }
    }
    return None;
}
