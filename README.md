# swc-plugin-react-native-web

A SWC plugin that will alias react-native to react-native-web.

## Installation

```sh
npm install --save-dev @nissy-dev/swc-plugin-react-native-web
```

## Usage

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["@nissy-dev/swc-plugin-react-native-web", { "common_js": false }]]
    }
  }
}
```
