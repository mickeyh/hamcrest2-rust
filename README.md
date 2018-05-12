[![Build Status](https://travis-ci.org/Valloric/hamcrest2-rust.svg?branch=master)](https://travis-ci.org/Valloric/hamcrest2-rust)

# Hamcrest

A port of [Hamcrest](http://hamcrest.org/) to [Rust](http://rust-lang.org).
This is a fork of the original hamcrest-rust which is unmaintained.

## Installing

To use Hamcrest, add this to your `Cargo.toml`:

```
[dev-dependencies]
hamcrest2 = "*"
```

And this to your crate root:

``` rust
#[cfg(test)] #[macro_use] extern crate hamcrest2;
```

After a quick `cargo build`, you should be good to go!

## Usage

Hamcrest2 supports a number of matchers. The easiest way is to just `use` them all like this:

``` rust
use hamcrest2::prelude::*;
```

If you want to be more selective make sure that you also import the `HamcrestMatcher` trait.

## General Matchers

### eq, not

``` rust
assert_that!(1, eq(1));  // also equal_to()
assert_that!(1, not(eq(2)));
```

### compared\_to

``` rust
assert_that!(1, lt(2));   // also less_than()
assert_that!(1, leq(1));  // also less_than_or_equal_to()
assert_that!(2, gt(1));   // also greater_than()
assert_that!(2, geq(2));  // also greater_than_or_equal_to()
```

### type_of

``` rust
assert_that!(123usize, type_of::<usize>());
assert_that!("test", type_of::<&str>());
```

### matches_regex

``` rust
assert_that!("1234", matches_regex(r"\d"));
assert_that!("abc", does_not(match_regex(r"\d")));
```

## Numerical Matchers

### close\_to

``` rust
assert_that!(1e-40f32, close_to(0.0, 0.01));
assert_that!(1e-40f32, not(close_to(0.0, 0.000001)));
```

## Filesystem Matchers

### existing\_{file,path,dir}

``` rust
assert_that!(&path, existing_path());
assert_that!(&path, existing_file());
assert_that!(&path, not(existing_dir()));
```

## Option and Result

### has

``` rust
let var: Option<i8> = Some(5);
assert_that!(var, has(5));

let var: Result<i8, String> = Ok(5);
assert_that!(var, has(5));
```

### some

``` rust
let var: Option<i8> = Some(5);
assert_that!(var, some());

assert_that!(Some(1), some::<int>());

let var: Option<i8> = None;
assert_that!(var, not(some()));
```

### none

``` rust
let var: Option<i8> = None;
assert_that!(var, none());

assert_that!(None, none::<int>());
assert_that!(Some(1), not(none::<int>()));
```

## Collection Matchers

### contains, contains\_exactly, contains\_in order

``` rust
assert_that!(&vec!(1i, 2, 3), contains(vec!(1i, 2)));
assert_that!(&vec!(1i, 2, 3), not(contains(vec!(4i))));

assert_that!(&vec!(1i, 2, 3), contains(vec!(1i, 2, 3)).exactly());
assert_that!(&vec!(1i, 2, 3), not(contains(vec!(1i, 2)).exactly()));

assert_that!(&vec!(1i, 2, 3), contains(vec!(1i, 2)).in_order());
assert_that!(&vec!(1i, 2, 3), not(contains(vec!(1i, 3)).in_order()));
```

## Compound Matchers

### all_of

``` rust
assert_that!(4, all_of!(lt(5), gt(3)));  // also all!()
assert_that!(
    &vec![1, 2, 3],
    all_of!(contains(vec![1, 2]), not(contains(vec![4])))
);
```

### any_of

``` rust
assert_that!(4, any_of!(less_than(2), greater_than(3)));  // also any!()
assert_that!(
    &vec![1, 2, 3],
    any_of!(contains(vec![1, 2, 5]), not(contains(vec![4])))
);
```

## Misc Matchers

### is(bool)

``` rust
assert_that!(true, is(true));
assert_that!(false, is(false));
```

### anything

``` rust
assert_that!(42, anything());
assert_that!("test", is(anything()));
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
