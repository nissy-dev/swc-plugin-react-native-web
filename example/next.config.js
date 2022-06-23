/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  experimental: {
    swcPlugins: [
      ["@nissy-dev/swc-plugin-react-native-web", { commonjs: true }],
    ],
  },
};

module.exports = nextConfig;
