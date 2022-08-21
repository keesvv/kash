# TODO

## Done

- [x] statement: move to lib
- [x] ktf: implement Input
- [x] ktf: simplify MonthValues parsing
- [x] convert: lib, move ktf input here
- [x] convert: Output trait
- [x] cli: move JSON output to convert
- [x] cli: use clap.rs for argp
- [x] convert: implement cargo features
- [x] ktf: move to own package
- [x] docs: add README to each package
- [x] lib: custom serializer fields for MonthValues
- [x] convert: JSON input
- [x] lib: MonthValues deserializer
- [x] http: implement warp.rs
- [x] http: retrieve statements as JSON
- [x] statement: remove None variant
- [x] lib: implement cargo features
- [x] statement: Transaction variant
- [x] convert: CAMT.053 deserialization lib
- [x] lib: rename statement structs
- [x] convert: CAMT.053 input
- [x] statement: Account variant
- [x] cli: ANSI pie charts
- [x] account: link transactions to accounts
- [x] cli: sort transactions
- [x] repo: repo management, read inputs from repo
- [x] manage-cli: `show` subcmd
- [x] manage-cli: config management
- [x] http: implement kash_repo
- [x] statement: Budget variant
- [x] account: multiple AccountId variants
- [x] lib: Rule variant w/ patterns-based replacements
  - [x] lib/rule: auto tagging
  - [x] lib/rule: auto description renaming

## Currently working on

- [ ] web: add bindings with wasm_bindgen

## Coming soon

- [ ] web: wasm/http repository abstractions with TypeScript

- [ ] transaction: Products w/ price tracking
- [ ] statement: Goal variant

- [ ] convert: convert between inputs
- [ ] repo: atomic edits for consumption in http, cli, etc

- [ ] lib: predictions

- [ ] http: Determine input type based on `Content-Type` header
- [ ] http: SVG chart generation
- [ ] ktf: finalize
