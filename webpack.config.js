const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

const fs = require('fs');
let raw = fs.readFileSync('./site_data.json');
let json = JSON.parse(raw);

const POST_NAMES = json.post_names;
const DEMO_NAMES = json.demo_names;

const getPlugins = () => {
    plugins = [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, './mkd')
        }),
        new HtmlWebpackPlugin({
            template: './index.html',
            title: 'Reshane Blog',
            chunks: ['mkd'],
            filename: 'index.html',
        }),
        new CopyPlugin({
            patterns: [
                { from: 'posts', to: 'posts' }
            ],
        })
    ];
    DEMO_NAMES.forEach(page => {
        plugins.push(
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, `./demos/${page}`)
            }),
            new HtmlWebpackPlugin({
                template: `./demos/${page}/index.html`,
                title: page.replace("-", " "),
                chunks: [`demos/${page.toLowerCase()}`],
                filename: `demos/${page}/index.html`,
            }),
        );
    });
    POST_NAMES.forEach(page => {
        plugins.push(
            new HtmlWebpackPlugin({
                template: './post_template.html',
                title: page.replace("-", " "),
                chunks: ['post'],
                filename: `posts/${page}/index.html`,
            }),
        );
    });
    return plugins;
}

const getEntry = () => {
    var entry = {
        'mkd': './index.js',
        'post': './renderer.js'
    };
    DEMO_NAMES.forEach(page => {
        entry[`demos/${page}`] = `./demos/${page}/index.js`;
    });
    return entry;
}

module.exports = {
    entry: getEntry(),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js',
    },
    plugins: getPlugins(),
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
   }
};

