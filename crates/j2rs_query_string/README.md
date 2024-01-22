# j2rs_query_string

```rust
use j2rs_query_string::parse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Hello {
  a: String,
  b: f64,
}

fn main() {
  println!("{:?}", parse::<Hello>("a=1&b=2"));
}
```