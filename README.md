# serde_to_v8

Converts serde_json `Value` to v8 binary format.

Why?

I was working with JS's built-in `JSON.parse`. It has no way to have key conversions, as I wanted snake_case keys to be convered to camelCase at parser-level. So I decided to instead translate untyped serde_json::Value to v8's binary serialization format which matches nearly the performance of v8's JSON parser.

## TODO/Ideas

- Maybe remove `wasm_bindgen` dependency and play with WASM at low level?
- Maybe write custom case-conversion function instead of using a dep?
- Maybe a custom JSON parser solely for translating to v8 binary?

## License

[Apache 2.0](./LICENSE)

Copyright 2021 @ DjDeveloperr
