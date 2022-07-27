# swc-plugin-react-native-web

[![npm version](https://badge.fury.io/js/@nissy-dev%2Fswc-plugin-react-native-web.svg)](https://badge.fury.io/js/@nissy-dev%2Fswc-plugin-react-native-web)

A SWC plugin that will alias react-native to react-native-web.  
Inspired from [babel-plugin-react-native-web](https://github.com/necolas/react-native-web/tree/master/packages/babel-plugin-react-native-web). 

## Installation

```sh
npm install --save-dev @nissy-dev/swc-plugin-react-native-web
```

## Usage

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["@nissy-dev/swc-plugin-react-native-web", { "commonjs": false }]]
    }
  }
}
```
