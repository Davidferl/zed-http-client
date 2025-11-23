# Rest/Http Client Extension for Zed

A Zed extension that brings http client capabilities to the editor, enabling you to write, execute, and test http requests directly from `.http` and `.rest` files.

## Features

- **Syntax Highlighting** - Full syntax support for `.http` and `.rest` files using the kulala tree-sitter grammar
- **Language Server Integration** - Intelligent code completion and validation via `kulala-ls`
- **Execute http Requests** - Run individual requests or entire files
- **Verbose Mode** - Detailed debugging output for troubleshooting requests
- **Zero Configuration** - Automatic installation of required dependencies

## Prerequisites

- **Node.js** - Required for executing http requests and running the language server
- **npm** - Used to download dependencies automatically

The extension will automatically install the following on first use:
- `@mistweaverco/kulala-ls` - Language server for http file support
- `httpyac` - cli for executing requests (via npx)

## License

Licensed under the Apache License, Version 2.0. See [LICENSE.md](LICENSE.md) for details.
