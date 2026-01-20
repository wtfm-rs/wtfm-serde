#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! # WTFM for [serde]
//! 
//! Although this is an empty crate without doc-tests only, `cargo doc`
//! by default builds documentation for dependencies
//!
//! ```sh
//! % du -hsc docs/doc/*
//! 4.0K	docs/doc/crates.js
//! 4.0K	docs/doc/help.html
//! 64K	docs/doc/itoa
//! 2.9M	docs/doc/memchr
//! 700K	docs/doc/proc_macro2
//! 172K	docs/doc/quote
//! 1.4M	docs/doc/search.index
//! 5.0M	docs/doc/serde
//! 4.4M	docs/doc/serde_core
//! 20K	docs/doc/serde_derive
//! 4.1M	docs/doc/serde_json
//! 4.0K	docs/doc/settings.html
//! 9.1M	docs/doc/src
//! 4.0K	docs/doc/src-files.js
//! 1.8M	docs/doc/static.files
//! 11M	docs/doc/syn
//! 1.3M	docs/doc/trait.impl
//! 420K	docs/doc/type.impl
//! 44K	docs/doc/unicode_ident
//! 16K	docs/doc/wtfm_serde
//! 52K	docs/doc/zmij
//! 42M	total
//! ```
//! We can navigate to them via the sidebar menu.
//!
//! ## It works!
//! <https://serde.rs/#data-structures>
//! ```
//! #[derive(serde::Serialize, serde::Deserialize, Debug)]
//! struct Point {
//!    x: i32,
//!    y: i32,
//! }
//!
//! let point = Point { x: 1, y: 2 };
//!
//! let serialized = serde_json::to_string(&point).unwrap();
//! debug_assert_eq!(format!("{}", serialized), "{\"x\":1,\"y\":2}");
//!
//! let deserialized: Point = serde_json::from_str(&serialized).unwrap();
//! debug_assert_eq!(format!("{:?}", deserialized), "Point { x: 1, y: 2 }");
//! ``` 
//! ```sh
//!    Doc-tests wtfm_serde
//!
//! running 1 test
//! test src/lib.rs - (line 36) ... ok
//!
//! test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//!
//! all doctests ran in 0.48s; merged doctests compilation took 0.25s
//! ```
