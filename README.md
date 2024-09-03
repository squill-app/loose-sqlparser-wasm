> A simple loose SQL parser for Javascript in WebAssembly

Based on a RUST implementation, **loose-sqlparser-wasm** is a non-validating SQL parser for Javascript. It provides support for parsing and splitting SQL statements.

# Usage

```ts
import { parseSql, Statement } from "loose-sqlparser-wasm";
const statements = parseSql("SELECT 1; SELECT 2") as Array<Statement>;
```

# Features

- Support multiple sql statements.

  ```ts
  const statements = parseSql("SELECT 1; SELECT 2") as Array<Statement>;
  statements.len(); // => 2
  ```

- SQL dialect agnostic. Intended to support a wild range of SQL dialects: **mysql**, **postgresql**, **sqlite**, **oracle**, ...

- Collect an AST of tokens for each statement:

  ```ts
  const statements = parseSql("SELECT 1; SELECT 2") as Array<Statement>;
  statements[0].tokens; // => ["SELECT", "1", ";"]
  statements[1].tokens; // => ["SELECT", "2"]
  ```

- For token, alongside of the _value_ itself, capture **line**:**column** and **byte offset** of the start and **line**:**column** for the end.

  ```sql
  SELECT 1+(4*5)-3
    FROM DUAL
  ;
  ```

  ```text
  start       end         offset  token
  ----------  ----------  ------  ------------------------------------------------
         1:1         1:6       0  SELECT
         1:8         1:8       7  1
         1:9         1:9       8  +
        1:10        1:10       9  (
        1:11        1:11      10    4
        1:12        1:12      11    *
        1:13        1:13      12    5
        1:14        1:14      13  )
        1:15        1:15      14  -
        1:16        1:16      15  3
         2:3         2:6      19  FROM
         2:8        2:11      24  DUAL
         3:1         3:1      29  ;
  ```

- Invalid SQL syntaxes don't stop the tokenization.

  ```ts
  // The missing opening parenthesis won't stop the tokenization.
  const statements = parseSql("SELECT (1+2)*3) FROM employee") as Array<Statement>;
  statements[0].tokens; // => ["SELECT", "(", "1", "+", "2", ")", "*", "3", ")", "FROM", "employee"]
  ```
