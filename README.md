# swc-plugin-react-native-web

[![npm version](https://badge.fury.io/js/@nissy-dev%2Fswc-plugin-react-native-web.svg)](https://badge.fury.io/js/@nissy-dev%2Fswc-plugin-react-native-web)

A SWC plugin that will alias react-native to react-native-web.  
Inspired from [babel-plugin-react-native-web](https://github.com/necolas/react-native-web/tree/master/packages/babel-plugin-react-native-web).


## Caution

I think the babel-plugin-react-native-web (also, this plugin) is not necessary in many cases in the term of tree shaking or minimizing bundle size. (See https://github.com/necolas/react-native-web/discussions/2217)

Before using this plugin, please consider about [the package aliasing](https://necolas.github.io/react-native-web/docs/setup/#package-aliasing).

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

## Compatibility

Below is a table showing compatibility of the plugin. 
The swc_core version refers to the version of the swc_core crate that the plugin has been compiled against.
The Next.js version refers to the version I confirmed to work properly with this plugin.

| Plugin version | swc_core version | Next.js version |
| :---: | :---: | :---: |
| 0.3.0 | 0.40.16 | ~13.0.0 |
| 0.2.6 | 0.23.24 | ~12.3.1 |
| 0.2.4 | - | 12.2.4 |
