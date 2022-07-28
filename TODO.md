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

## Currently working on

- [ ] http: retrieve statements as JSON
- [ ] http: Determine input type based on `Content-Type` header

## Coming soon

- [ ] ktf: use serde.rs
- [ ] ktf: ignore newlines and comments
- [ ] statement: remove None variant

- [ ] statement: Transaction variant
- [ ] convert: CAMT.053 parser lib

- [ ] lib: implement cargo features
- [ ] statement: Budget variant

- [ ] lib: atomic edits for consumption in http, cli, etc

- [ ] lib: predictions
- [ ] cli: ANSI pie charts
- [ ] http: SVG chart generation
