# XVII

[![Crates.io](https://img.shields.io/crates/v/xvii.svg)](https://crates.io/crates/xvii)
[![Build Status](https://travis-ci.org/archer884/xvii.svg?branch=master)](https://travis-ci.org/archer884/xvii)


...Pronounced any way you like, including "seventeen."

This library provides parsing and formatting for Roman numerals. According to my (probably extremely suspect) benchmarks, this is the best-performing library of its kind available on crates.io (you know, as of St. Patrick's Day, 2017 when I did the tests), so you should definitely employ it in your high-availability NAAS (numerals-as-a-service) project.

(Seriously, though, read the license--no warranties implied!)

### Parsing

Parsing is provided through Rust's ordinary `FromStr` trait, meaning you can create `Roman` values same as you create any other number--albeit with slightly simpler error cases, since the only possible errors are "Hey, that letter can't go in there," or "Dude, they go from 1 to 3999 and that's it!"

```rust
let seventeen: Roman = "XVII".parse().unwrap();
```

### Formatting

There are several formatting options. `Roman` implements `Display`, which means that it'll work fine with `println!("{}")` et al., but for maximum efficiency (stop laughing!) I also provide two other functions: `to_lowercase()` and `to_uppercase()`. These skip the `Display` piping and just go straight into a new string.

Regarding formatting, there is one gotcha regarding the formatting of `Roman` values created via `Roman::from_unchecked()`: values that are less than the minimum printable value will come out as empty strings, while values that are larger will simply look like `MMMMMMMMMMMMMMMXIV` or something like that.

## Roadmap

I'm thinking of stealing that feature from the other library where you can use the `"{:x}"` or `"{:X}"` formatters to choose upper vs. lower case in formatting.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE][apc] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[apc]:https://github.com/archer884/xvii/blob/master/LICENSE-APACHE
[mit]:https://github.com/archer884/xvii/blob/master/LICENSE-MIT
