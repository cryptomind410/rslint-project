The RSLint CLI works without a configuration file and will select reccomended non-stylistic rules to run.

## Features

**Speed**. RSLint uses parallelism to utilize multiple threads to speed up linting on top of being compiled to
native code.

**Low memory footprint**. RSLint's syntax tree utilizes interning and other ways of drastically reducing memory usage
while linting.

**Sensible defaults**. The CLI assumes reccomended non-stylistic rules if no configuration file is specified and ignores directories such as
`node_modules`.

**Error recovery**. RSLint's custom parser can recover from syntax errors and produce a usable syntax tree even when whole parts of
a statement are missing. Allowing accurate on-the-fly linting as you type.

**No confusing options**. ECMAScript version for the parser does not have to be configured, the parser assumes latest syntax and
assumes scripts for `*.js` and modules for `*.mjs`.

**Native TypeScript support**. `*.ts` files are automatically linted, no configuration for different parsers or rules is required.

**Rule groups**. Rules are grouped by scope for ease of configuration, understanding, and a cleaner file structure for the project.

**Understandable errors**. Each error emitted by the linter points out the area in the source code in an understandable and clean manner as well as contains labels, notes, and suggestions to explain how to fix each issue. There is also an alternative formatter similar to ESLint's formatter available using the `-F` flag or the `formatter` key in the config.

**Strongly typed rule configuration**. RSLint ships a JSON schema and links it for `rslintrc.json` to provide autocompletion for the config file in Visual Studio Code. The JSON Schema describes rule config options in full, allowing easy configuration. Moreover, RSLint's language server protocol implementation provides autocompletion for `rslintrc.toml` files too.

**Powerful directives**. Directives (commands through comments) use a parser based around the internal JavaScript lexer with instructions, allowing us to provide:

- Autocompletion for directives such as `// rslint-ignore no-empty` in the language server protocol.
- Hover support for directives to offer information on a command on hover.
- Understandable errors for incorrect directives.

**Standalone**. RSLint is compiled to a single standalone binary, it does not require Node, v8, or any other runtime. RSLint can run on any platform which can be targeted by LLVM.

**Powerful autofix**. Automatic fixes for some errors are provided and can be applied through the `--fix` flag or actions in the IDE. Fixes can even be applied if the file contains syntax errors through the `--dirty` flag.

**Built-in documentation**. RSLint contains rule documentation in its binary, allowing it to show documentation in the terminal through the explain subcommand, e.g. `rslint explain no-empty, for-direction`.

## Internal Features

**Clean and clear project layout**. The RSLint project is laid out in a monorepo and each crate has a distinct job, each crate can be used in other Rust projects and each crate has good documentation and a good API.

**Easy rule declaration**. Rules are declared using a `declare_lint!` macro. The macro accepts doc comments, a struct name, the group name, a rule code, and configuration options. The macro generates a struct definition and a `Rule` implementation and processes the doc comments into the documentation for the struct as well as into a static string used in the `docs()` method on each rule. Everything is concise and kept in one place.

**Full fidelity syntax tree**. Unlike ESTree, RSLint's custom syntax tree retains:

- All whitespace
- All comments
- All tokens

Allowing it to have powerful analysis without having to rely on separate structures such as ESLint's `SourceCode`.

**Untyped Syntax Tree**. RSLint's syntax tree is made of untyped nodes and untyped tokens at the low level, this allows for powerful, efficient traversal through the tree, e.g. `if_stmt.cons()?.child_with_ast::<SwitchStmt>()`.

**Easy APIs**. RSLint uses easy to use builders for its complex errors, as well as builders for autofix. Everything is laid out to minimize the effort required to implement rules.

## License

This project is Licensed under the [MIT license](http://opensource.org/licenses/MIT).
