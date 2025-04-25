const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

const POST_NAMES = ["aoc-24", "convolution"];// , "fourier-transforms", "image-resizing"];

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


module.exports = {
    entry: {
        'mkd': './index.js',
        'post': './renderer.js'
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js',
    },
    /*plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, './mkd')
        }),
        new HtmlWebpackPlugin({
            template: './index.html',
            title: 'Reshane Blog',
            chunks: ['mkd'],
            filename: 'index.html',
        }),
        new HtmlWebpackPlugin({
            template: './post_template.html',
            title: 'Test Post',
            chunks: ['post'],
            filename: 'posts/test-post/index.html',
        }),
        new CopyPlugin({
            patterns: [
                { from: 'posts', to: 'posts' }
            ],
        })
    ],*/
    plugins: getPlugins(),
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
   }
};

