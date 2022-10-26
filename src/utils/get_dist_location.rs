pub fn get_dist_location(import_name: &str, common_js: bool) -> String {
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
        panic!("{} is not a valid import", import_name);
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
        "useWindowDimensions",
    ])
}
