use swc_plugin::ast::*;
use swc_plugin::syntax_pos::DUMMY_SP;

pub fn is_react_native_module(node: &mut ImportDecl) -> bool {
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
            span: named_specifier.span.clone(),
            local: named_specifier.local.clone(),
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

fn get_import_name(named_specifier: &ImportNamedSpecifier) -> JsWord {
    let specifier = named_specifier.clone();
    if let Some(ModuleExportName::Ident(ident)) = specifier.imported {
        ident.sym
    } else {
        specifier.local.sym
    }
}

fn get_dist_location(import_name: &str, common_js: bool) -> String {
    let format = if common_js { "cjs/" } else { "" };
    let internal_name = if import_name == "unstable_createElement" {
        "createElement"
    } else {
        import_name
    };

    if internal_name == "index" {
        return String::from(format!("react-native-web/dist/{}index", format));
    } else if internal_name != "" && module_map().contains(&internal_name) {
        return String::from(format!(
            "react-native-web/dist/{}exports/{}",
            format, internal_name
        ));
    } else {
        // TODO: we should throw, but babel plugin return the following value
        // panic!("{} is not a valid import", import_name);
        return String::from(format!("react-native-web/dist/{}index", format));
    }
}

// TODO: we should generate this file automatically
// https://github.com/necolas/react-native-web/blob/master/packages/babel-plugin-react-native-web/src/moduleMap.js
fn module_map() -> Vec<&'static str> {
    Vec::from([
        "AccessibilityInfo",
        "ActivityIndicator",
        "Alert",
        "Animated",
        "AppRegistry",
        "AppState",
        "Appearance",
        "BackHandler",
        "Button",
        "CheckBox",
        "Clipboard",
        "DeviceEventEmitter",
        "DeviceInfo",
        "Dimensions",
        "DrawerLayoutAndroid",
        "Easing",
        "FlatList",
        "I18nManager",
        "Image",
        "ImageBackground",
        "InputAccessoryView",
        "InteractionManager",
        "Keyboard",
        "KeyboardAvoidingView",
        "LayoutAnimation",
        "Linking",
        "LogBox",
        "Modal",
        "NativeEventEmitter",
        "NativeModules",
        "PanResponder",
        "PermissionsAndroid",
        "Picker",
        "PixelRatio",
        "Platform",
        "Pressable",
        "ProgressBar",
        "RefreshControl",
        "SafeAreaView",
        "ScrollView",
        "SectionList",
        "Settings",
        "Share",
        "StatusBar",
        "StyleSheet",
        "Switch",
        "Systrace",
        "TVEventHandler",
        "Text",
        "TextInput",
        "ToastAndroid",
        "Touchable",
        "TouchableHighlight",
        "TouchableNativeFeedback",
        "TouchableOpacity",
        "TouchableWithoutFeedback",
        "UIManager",
        "Vibration",
        "View",
        "VirtualizedList",
        "YellowBox",
        "createElement",
        "findNodeHandle",
        "processColor",
        "render",
        "unmountComponentAtNode",
        "useColorScheme",
        "useLocaleContext",
        "useWindowDimensio",
    ])
}
