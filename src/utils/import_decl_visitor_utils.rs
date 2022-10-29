use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{
            ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportSpecifier,
            ModuleExportName, Str,
        },
        atoms::JsWord,
    },
};

use super::get_dist_location::get_dist_location;

pub fn is_react_native_module(node: &ImportDecl) -> bool {
    (node.src.value == Str::from("react-native").value
        || node.src.value == Str::from("react-native-web").value)
        && node.specifiers.len() > 0
}

pub fn create_new_import_decl(specifier: ImportSpecifier, common_js: bool) -> ImportDecl {
    if let ImportSpecifier::Named(named_specifier) = specifier {
        let import_name = get_import_name(&named_specifier);
        let dist_location = get_dist_location(&import_name, common_js);
        let new_src = Str::from(dist_location);
        let new_specifiers = vec![ImportSpecifier::Default(ImportDefaultSpecifier {
            span: named_specifier.span,
            local: named_specifier.local,
        })];
        ImportDecl {
            span: DUMMY_SP,
            src: new_src,
            specifiers: new_specifiers,
            type_only: Default::default(),
            asserts: None,
        }
    } else {
        let dist_location = get_dist_location("index", common_js);
        let new_src = Str::from(dist_location);
        ImportDecl {
            span: DUMMY_SP,
            src: new_src,
            specifiers: vec![specifier],
            type_only: Default::default(),
            asserts: None,
        }
    }
}

fn get_import_name(named_specifier: &ImportNamedSpecifier) -> &JsWord {
    if let Some(ModuleExportName::Ident(ident)) = &named_specifier.imported {
        &ident.sym
    } else {
        &named_specifier.local.sym
    }
}
