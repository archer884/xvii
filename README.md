# XVII

[![Crates.io](https://img.shields.io/crates/v/xvii.svg)](https://crates.io/crates/xvii)
[![documentation (docs.rs)](https://docs.rs/xvii/badge.svg)](https://docs.rs/xvii)
[![Build Status](https://travis-ci.org/archer884/xvii.svg?branch=master)](https://travis-ci.org/archer884/xvii)

...Pronounced any way you like, including "seventeen."

This library provides parsing and formatting for Roman numerals. According to my (probably extremely suspect) benchmarks, this is the best-performing library of its kind available on crates.io (you know, as of St. Patrick's Day, 2017 when I did the tests), so you should definitely employ it in your high-availability NAAS (numerals-as-a-service) project.

(Seriously, though, read the license--no warranties implied!)

Also, if you have a high-availability NAAS project, you need to have your head examined. I don't know if that was clear when I originally wrote this readme, so I'm adding it now.

## Usage

### Parsing

Parsing is provided through Rust's ordinary `FromStr` trait, meaning you can create `Roman` values same as you create any other number--albeit with slightly simpler error cases, since the only possible errors are "Hey, that letter can't go in there," or "Dude, they go from 1 to 4999 and that's it!"

```rust
let seventeen: Roman = "XVII".parse().unwrap();
```

### Formatting

There are several formatting options. `Roman` implements `Display`, which means that it'll work fine with `println!("{}")` et al., but for maximum efficiency (stop laughing!) I also provide two other functions: `to_lowercase()` and `to_uppercase()`. These skip the `Display` piping and just go straight into a new string. Lastly, the `format()` method builds a lazy formatter in either upper or lowercase mode.

## Changelog

* **v0.4.1** Upgrade to rust edition 2018, support for no-std usage (thanks to [WaffleLapkin](https://github.com/WaffleLapkin)!), plenty of cleanup (also thanks to him, really).
* **v0.2.2** Upgrade parsing to use some kind of whacky state machine in order to permit numbers up to the commonly accepted ceiling of 4999, or MMMMCMXCIX, thereby avoiding a potential Y4K bug. Your thousand year reich is now safe with me.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE][apc] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[apc]:https://github.com/archer884/xvii/blob/master/LICENSE-APACHE
[mit]:https://github.com/archer884/xvii/blob/master/LICENSE-MIT
