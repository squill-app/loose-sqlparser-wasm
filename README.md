# Usage

```ts
import { parseSql } from "loose-sqlparser-wasm";
const statements = parseSql("SELECT 1; SELECT 2") as Array<Statement>;
```

# Building the package

```sh
wasm-pack build --target bundler
wasm-pack pack
wasm-pack publish
```
