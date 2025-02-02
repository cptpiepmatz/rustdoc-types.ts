<h1 align="center">rustdoc-types.ts</h1>
<p align="center">
  <b>A Node.js package that generates TypeScript bindings for the <a href="https://crates.io/crates/rustdoc-types">rustdoc-types</a> 
  crate.</b>
</p>

<br>

<div align="center">

  [![npm Version](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fcptpiepmatz%2Frustdoc-types.ts%2Frefs%2Fheads%2Fmain%2Fpackage.json&query=version&prefix=v&style=for-the-badge&label=version)](https://github.com/cptpiepmatz/rustdoc-types.ts)
  [![License](https://img.shields.io/github/license/cptpiepmatz/rustdoc-types.ts?style=for-the-badge)](https://github.com/cptpiepmatz/rustdoc-types.ts/blob/main/LICENSE)

</div>


## About

**rustdoc-types.ts** lets you use the data structures from the [rustdoc-types](https://crates.io/crates/rustdoc-types) crate directly in TypeScript. When you install this package, it pulls the specified version of **rustdoc-types**, extracts the core definitions from `lib.rs`, and generates matching TypeScript bindings. This is especially handy for TypeScript projects that need to parse or handle Rust doc JSON using TypeScript types.

> [!NOTE]  
> To install this package, a Rust toolchain is necessary. Anyone dealing with these doc structures likely has Rust installed anyway, so it shouldn't be a huge hassle.

## Features
- **Automatic Rust-to-TypeScript Binding Generation**: No manual copying or editing of type definitions.
- **Consistent with rustdoc-types**: These definitions stay in sync with updates to rustdoc-types.
- **Local Generation**: The types aren't shipped with the repo, so you get fresh, up-to-date bindings every install.
- **Works Well with Existing TS Tooling**: Just import the types and use them in your code.

## Installation
Before installing, define the rustdoc-types version in your package.json:

```
"rustdoc-types": {
    "version": "0.35.0"
},
```

Make sure Rust is installed, then run:

```shell
npm install https://github.com/cptpiepmatz/rustdoc-types.ts.git
```

> [!IMPORTANT]  
> The first installation might take a while, as it compiles some Rust code on the fly.

This triggers the generation of TypeScript bindings, using your local Rust toolchain.

## Usage
After installation, just import the generated types anywhere in your TypeScript project:

```ts
// In ESM modules:
import { Crate, Item, Function, FORMAT_VERSIOn } from "rustdoc-types.ts";

// In CommonJS files:
import type { Crate, Item, Function } from "rustdoc-types.ts" with { "resolution-mode": "import" };
import { FORMAT_VERSION } from "rustdoc-types.ts";

// Use the types as you normally would.
```

If you plan on distributing your code to users who might not have Rust or this package installed, consider bundling your final output with a bundler like Webpack, Rollup, or esbuild. This way, they won't need the Rust toolchain, and the generated types or code are all neatly packaged together.


## Testing
To run the generation script and tests locally:

1. **Install dependencies**:
   ```shell
   npm install
   ```
2. **Build and generate**:
   ```shell
   npm run build
   ```
   This calls the Rust toolchain to grab the rustdoc-types crate and create TS bindings.
3. **Test**:
   ```shell
   npm test
   ```
   Ensures everything works as expected.

## License
**rustdoc-types.ts** is released under the [MIT License](LICENSE). Feel free to use it in your own projects.

