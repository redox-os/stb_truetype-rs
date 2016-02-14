# stb_truetype-rs

[![Build Status](https://travis-ci.org/dylanede/stb_truetype-rs.svg?branch=master)](https://travis-ci.org/dylanede/stb_truetype-rs)

This is a straight translation of the font loading code in
[stb_truetype.h](https://github.com/nothings/stb/blob/master/stb_truetype.h)
from C to Rust. It is intended as a stopgap dependency for libraries that deal
with fonts until something better, written in idiomatic Rust, is available. This
library is not an example of good Rust code, but it works.

If you want to use this crate directly, add the following to your Cargo.toml:

```toml
[dependencies]
stb_truetype = "0.1.2"
```

Please note that the documentation provided is also a straight copy from the
original code.

Currently this port does not include the rasterisation or font querying API
provided by stb_truetype.h. If you are looking for font rasterisation, that is
provided by my other project,
[RustType](https://github.com/dylanede/rusttype).

## [Documentation](https://dylanede.github.io/stb_truetype-rs)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
