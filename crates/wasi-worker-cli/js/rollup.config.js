// Rollup Config for Web Workers

import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import typescript from "@rollup/plugin-typescript";
import compiler from "@ampproject/rollup-plugin-closure-compiler";
import replace from '@rollup/plugin-replace';
import globals from "rollup-plugin-node-globals";
import builtins from "rollup-plugin-node-builtins";
//import copy from 'rollup-plugin-copy';

// Allowing to compile wasmer-js for browser
const replaceBrowserOptions = {
    delimiters: ["", ""],
    "/*ROLLUP_REPLACE_BROWSER": "",
    "ROLLUP_REPLACE_BROWSER*/": "",
    'Object.defineProperty(exports, "__esModule", { value: true });': "",
};

const sourcemapOption = process.env.PROD ? undefined : "inline";

const plugins = [
  replace(replaceBrowserOptions),
  typescript(),
  resolve({
    preferBuiltins: false,
  }),
  commonjs({
      transformMixedEsModules: true,
    }),
  globals(),
  builtins(),
  process.env.PROD ? compiler() : undefined,
];

const workerBundles = [
  {
    input: "./src/worker.ts",
    output: [
      {
        dir: "dist",
//        file: "dist/worker.js",
        format: "iife",
        sourcemap: sourcemapOption,
        name: "Process"
      }
    ],
    watch: {
      clearScreen: false
    },
    plugins: plugins
  }
];

export default workerBundles;