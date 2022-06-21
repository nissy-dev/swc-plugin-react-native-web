use std::collections::HashMap;

use serde::Deserialize;
use swc_plugin::{ast::*, plugin_transform, TransformPluginProgramMetadata};

mod utils;

pub struct TransformVisitor {
    common_js: bool,
}

impl TransformVisitor {
    fn new() -> Self {
        Self { common_js: false }
    }

    fn set_config(&mut self, common_js: bool) {
        self.common_js = common_js;
    }

    fn visit_mut_module_items_to_transform(&mut self, module_body: &mut Vec<ModuleItem>) {
        // Create the pairs of current declaration and new declarations for each react-native import/export
        // Maps -> HashMap<target_declaration_idx, new_declarations>
        let react_native_transform_maps = module_body
            .into_iter()
            .enumerate()
            .filter_map(|(idx, module_item)| match module_item {
                ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) => {
                    if utils::import_decl_visitor_utils::is_react_native_module(import_decl) {
                        let new_imports = import_decl
                            .specifiers
                            .clone()
                            .into_iter()
                            .map(|specifier| {
                                ModuleItem::ModuleDecl(ModuleDecl::Import(
                                    utils::import_decl_visitor_utils::create_new_import_decl(
                                        specifier,
                                        self.common_js,
                                    ),
                                ))
                            })
                            .collect::<Vec<_>>();
                        Some(HashMap::from([(idx, new_imports)]))
                    } else {
                        None
                    }
                }
                ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(named_export_decl)) => {
                    if utils::export_decl_visitor_utils::is_react_native_module(named_export_decl) {
                        let new_exports = named_export_decl
                            .specifiers
                            .clone()
                            .into_iter()
                            .map(|specifier| {
                                ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(
                                    utils::export_decl_visitor_utils::create_new_export_decl(
                                        specifier,
                                        self.common_js,
                                    ),
                                ))
                            })
                            .collect::<Vec<_>>();
                        Some(HashMap::from([(idx, new_exports)]))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        // transform the imports
        let mut idx_diff = 0;
        for transform_map in react_native_transform_maps {
            for (idx, new_declarations) in transform_map {
                module_body.remove(idx + idx_diff);

                for (i, new_declaration) in new_declarations.clone().into_iter().enumerate() {
                    module_body.insert(idx + idx_diff + i, new_declaration);
                }

                idx_diff += new_declarations.len() - 1;
            }
        }
    }

    fn visit_mut_var_decl_to_transform(&mut self, var_decl: &mut VarDecl) {
        let react_native_transform_maps = var_decl
            .decls
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(idx, var_declarator)| {
                if utils::variable_declaration_visitor::is_react_native_require(&var_declarator) {
                    Some((
                        idx,
                        utils::variable_declaration_visitor::create_new_variable_decl(
                            &var_declarator,
                            self.common_js,
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut idx_diff = 0;
        for (idx, _new_variable_decls) in react_native_transform_maps {
            println!("idx: {}", idx);
            println!("_new_variable_decls: {:?}", _new_variable_decls);
            var_decl.decls.remove(idx + idx_diff);
            if let Some(new_variable_decls) = _new_variable_decls {
                for (i, new_variable_decl) in new_variable_decls.clone().into_iter().enumerate() {
                    var_decl.decls.insert(idx + idx_diff + i, new_variable_decl);
                }
                idx_diff += new_variable_decls.len() - 1;
            }
        }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, module: &mut Module) {
        self.visit_mut_module_items_to_transform(&mut module.body);
        module.visit_mut_children_with(self);
    }

    fn visit_mut_var_decl(&mut self, var_decl: &mut VarDecl) {
        self.visit_mut_var_decl_to_transform(var_decl);
        var_decl.visit_mut_children_with(self);
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub common_js: bool,
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let mut visitor = TransformVisitor::new();
    let config = serde_json::from_str::<Config>(&metadata.plugin_config)
        .expect("invalid config for styled-components");
    visitor.set_config(config.common_js);
    program.fold_with(&mut as_folder(visitor))
}

#[cfg(test)]
mod transform_visitor_tests {
    use swc_ecma_transforms_testing::test;

    use super::*;

    fn transform_visitor() -> impl 'static + Fold + VisitMut {
        as_folder(TransformVisitor::new())
    }

    fn transform_visitor_cjs() -> impl 'static + Fold + VisitMut {
        let mut visitor = TransformVisitor::new();
        visitor.set_config(true);
        as_folder(visitor)
    }

    test!(
        ::swc_ecma_parser::Syntax::Es(::swc_ecma_parser::EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| transform_visitor(),
        rewrite_react_native_imports,
        r#"
        import ReactNative from "react-native";
        import { View } from "react-native";
        import { View as MyView } from "react-native";
        import * as ReactNativeModules from "react-native";
        "#,
        r#"
        import ReactNative from "react-native-web/dist/index";
        import View from "react-native-web/dist/exports/View";
        import MyView from "react-native-web/dist/exports/View";
        import * as ReactNativeModules from "react-native-web/dist/index";
        "#
    );

    test!(
        ::swc_ecma_parser::Syntax::Es(::swc_ecma_parser::EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| transform_visitor_cjs(),
        rewrite_react_native_imports_cjs,
        r#"
        import ReactNative from "react-native";
        import { View } from "react-native";
        import { View as MyView } from "react-native";
        import * as ReactNativeModules from "react-native";
        "#,
        r#"
        import ReactNative from "react-native-web/dist/cjs/index";
        import View from "react-native-web/dist/cjs/exports/View";
        import MyView from "react-native-web/dist/cjs/exports/View";
        import * as ReactNativeModules from "react-native-web/dist/cjs/index";
        "#
    );

    test!(
        ::swc_ecma_parser::Syntax::Es(::swc_ecma_parser::EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| transform_visitor(),
        rewrite_react_native_web_imports,
        r#"
        import { unstable_createElement } from "react-native-web";
        import { StyleSheet, View, TouchableOpacity, processColor } from "react-native-web";
        import * as ReactNativeModules from "react-native-web";
        "#,
        r#"
        import unstable_createElement from "react-native-web/dist/exports/createElement";
        import StyleSheet from "react-native-web/dist/exports/StyleSheet";
        import View from "react-native-web/dist/exports/View";
        import TouchableOpacity from "react-native-web/dist/exports/TouchableOpacity";
        import processColor from "react-native-web/dist/exports/processColor";
        import * as ReactNativeModules from "react-native-web/dist/index";
        "#
    );

    test!(
        ::swc_ecma_parser::Syntax::Es(::swc_ecma_parser::EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| transform_visitor(),
        rewrite_react_native_exports,
        r#"
        export { View } from "react-native";
        export { StyleSheet, Text, unstable_createElement } from "react-native";
        "#,
        r#"
        export { default as View } from "react-native-web/dist/exports/View";
        export { default as StyleSheet } from "react-native-web/dist/exports/StyleSheet";
        export { default as Text } from "react-native-web/dist/exports/Text";
        export { default as unstable_createElement } from "react-native-web/dist/exports/createElement";
        "#
    );

    test!(
        ::swc_ecma_parser::Syntax::Es(::swc_ecma_parser::EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| transform_visitor(),
        rewrite_react_native_web_exports,
        r#"
        export { View } from "react-native-web";
        export { StyleSheet, Text, unstable_createElement } from "react-native-web";
        "#,
        r#"
        export { default as View } from "react-native-web/dist/exports/View";
        export { default as StyleSheet } from "react-native-web/dist/exports/StyleSheet";
        export { default as Text } from "react-native-web/dist/exports/Text";
        export { default as unstable_createElement } from "react-native-web/dist/exports/createElement";
        "#
    );

    test!(
        ::swc_ecma_parser::Syntax::Es(::swc_ecma_parser::EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| transform_visitor(),
        rewrite_react_native_require,
        r#"
        const ReactNative = require("react-native");
        const { View } = require("react-native");
        const { StyleSheet, TouchableOpacity } = require("react-native");
        "#,
        r#"
        const ReactNative = require("react-native-web/dist/index");
        const View = require("react-native-web/dist/exports/View").default;
        const StyleSheet = require("react-native-web/dist/exports/StyleSheet").default;
        const TouchableOpacity = require("react-native-web/dist/exports/TouchableOpacity").default;
        "#
    );
}
