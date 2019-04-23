const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CleanWebpackPlugin = require("clean-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = [
    ["uibench_wasm_bindgen_clone", "clone"],
    ["uibench_wasm_bindgen_inner_html", "inner-html"],
].map(([pkg, out]) => {
    const crateDirectory = path.resolve(__dirname, pkg);
    const outPath = path.resolve(__dirname, "docs", out);
    return {
        mode: "production",
        entry: path.resolve(crateDirectory, "static", "index.js"),
        output: {
            path: outPath,
            filename: "index.js",
        },
        devServer: {
            contentBase: path.resolve(__dirname, "docs"),
        },
        module: {
            rules: [
                {
                    test: /\.css$/,
                    use: [
                        "style-loader",
                        MiniCssExtractPlugin.loader,
                        "css-loader",
                    ],
                },
            ],
        },
        plugins: [
            new CleanWebpackPlugin({
                dry: true,
                cleanOnceBeforeBuildPatterns: [
                    path.resolve(crateDirectory, "pkg", "*"),
                ],
            }),
            new MiniCssExtractPlugin({
                filename: "index.css",
            }),
            new WasmPackPlugin({
                crateDirectory,
                watchDirectories: [
                    path.resolve(__dirname, "uibench_sys", "src"),
                    path.resolve(
                        __dirname,
                        "uibench_wasm_bindgen_inner_html",
                        "src",
                    ),
                    path.resolve(
                        __dirname,
                        "uibench_wasm_bindgen_clone",
                        "src",
                    ),
                ],
            }),
            new HtmlWebpackPlugin({
                template: path.resolve(crateDirectory, "static", "index.html"),
                minify: true,
            }),
        ],
    };
});
