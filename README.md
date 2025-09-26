# bitmap_writer

Add parsing support for numerical literals to integer types.

## Usage

To use `num_literal_traits`, first add this to your `Cargo.toml`:

```toml
[dependencies]
num_literal_traits = "0.1.0"
```

Next, add this to your crate:

```rust
use num_literal_traits::NumLiteralTrait
```

## Features

All major C/C++ literal integer formats are supported: `123456`, `0x123ABC`, `01234567`, `0b0101001`.

Additionally the numerical parts can include underscores which are removed when parsing: `0b0010_0011_0000_1001`.