use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use swc_core::{
    ecma::{
        ast::{Module, ModuleDecl, ModuleItem, Program},
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

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

    fn visit_mut_module_items_to_transform_import(&mut self, module_body: &mut Vec<ModuleItem>) {
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

                let len = new_declarations.len();
                for (i, new_declaration) in new_declarations.into_iter().enumerate() {
                    module_body.insert(idx + idx_diff + i, new_declaration);
                }

                idx_diff += len - 1;
            }
        }
    }
}

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        self.visit_mut_module_items_to_transform_import(&mut module.body);
        module.visit_mut_children_with(self);
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub commonjs: bool,
}

#[plugin_transform]
pub fn swc_plugin_react_native_web(
    program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    let mut visitor = TransformVisitor::new();
    let config = serde_json::from_str::<Config>(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to get plugin config for swc-plugin-react-native-web"),
    )
    .expect("invalid config for swc-plugin-react-native-web");
    visitor.set_config(config.commonjs);
    program.fold_with(&mut as_folder(visitor))
}

#[cfg(test)]
mod transform_visitor_tests {
    use swc_core::ecma::{
        parser::{EsConfig, Syntax},
        transforms::testing::test,
        visit::Fold,
    };

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
        Syntax::Es(EsConfig {
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
        Syntax::Es(EsConfig {
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
        Syntax::Es(EsConfig {
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
        Syntax::Es(EsConfig {
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
        Syntax::Es(EsConfig {
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
}
