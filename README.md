comma_serde_urlencoded
======================

[`serde_urlencoded`] with support for serialization and deserialization of sequences and tuples in comma separated format.

Installation
============

This crate works with Cargo and can be found on
[crates.io] with a `Cargo.toml` like:

```toml
[dependencies]
comma_serde_urlencoded = "0.8"
```

The documentation is available on [docs.rs].

[crates.io]: https://crates.io/crates/comma_serde_urlencoded
[docs.rs]: https://docs.rs/comma_serde_urlencoded/0.8.1/comma_serde_urlencoded/

Example
=======

```rust
let meal = &(
    ("bread", ["baguette", "strucia"]),
    ("cheese", vec!["comté", "cheddar"]),
    ("meat", ("ham", "becon")),
    ("fat", "butter"),
);

assert_eq!(
    comma_serde_urlencoded::to_string(meal),
    Ok("bread=baguette%2Cstrucia&cheese=comt%C3%A9%2Ccheddar&meat=ham%2Cbecon&fat=butter".to_owned())
);
```

## License

comma_serde_urlencoded is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

serde_urlencoded licenses

 * Apache License, Version 2.0, ([NOX-LICENSE-APACHE](NOX-LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([NOX-LICENSE-MIT](NOX-LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in serde_urlencoded by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[`serde_urlencoded`]: https://github.com/nox/serde_urlencoded
