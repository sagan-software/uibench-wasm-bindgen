const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CleanWebpackPlugin = require("clean-webpack-plugin");

const inDir = path.resolve(__dirname);
const outDir = path.resolve(__dirname, "dist");

module.exports = {
  entry: path.resolve(inDir, "index.js"),
  output: {
    path: outDir,
    filename: "index.js"
  },
  devServer: {
    contentBase: path.resolve(__dirname, "dist")
  },
  plugins: [
    new CleanWebpackPlugin(outDir),
    new HtmlWebpackPlugin({
      template: path.resolve(inDir, "index.html"),
      minify: true
    })
  ],
  mode: "production"
};
