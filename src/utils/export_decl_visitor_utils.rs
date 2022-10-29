use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{ExportNamedSpecifier, ExportSpecifier, Ident, ModuleExportName, NamedExport, Str},
        atoms::JsWord,
    },
};

use super::get_dist_location::get_dist_location;

pub fn is_react_native_module(node: &NamedExport) -> bool {
    if let Some(src) = &node.src {
        return (src.value == Str::from("react-native").value
            || src.value == Str::from("react-native-web").value)
            && node.specifiers.len() > 0;
    }
    return false;
}

pub fn create_new_export_decl(specifier: ExportSpecifier, common_js: bool) -> NamedExport {
    if let ExportSpecifier::Named(named_specifier) = specifier {
        let local_name = get_local_name(&named_specifier).unwrap();
        let dist_location = get_dist_location(&local_name, common_js);
        let new_src = Str::from(dist_location);
        let new_specifiers = vec![ExportSpecifier::Named(ExportNamedSpecifier {
            span: DUMMY_SP,
            orig: ModuleExportName::Ident(Ident {
                span: DUMMY_SP,
                sym: "default".into(),
                optional: Default::default(),
            }),
            exported: Some(named_specifier.orig),
            is_type_only: Default::default(),
        })];
        NamedExport {
            span: DUMMY_SP,
            src: Some(new_src),
            specifiers: new_specifiers,
            type_only: Default::default(),
            asserts: None,
        }
    } else {
        let dist_location = get_dist_location("index", common_js);
        let new_src = Str::from(dist_location);
        NamedExport {
            span: DUMMY_SP,
            src: Some(new_src),
            specifiers: vec![specifier],
            type_only: Default::default(),
            asserts: None,
        }
    }
}

fn get_local_name(named_specifier: &ExportNamedSpecifier) -> Option<&JsWord> {
    if let ModuleExportName::Ident(ident) = &named_specifier.orig {
        Some(&ident.sym)
    } else {
        None
    }
}
