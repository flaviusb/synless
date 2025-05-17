# synless

More or less ometa-ish approach to parsing TokenSteams for Rust proc macros, without going through syn.

You make normal data structures to describe a parse with matching and extraction into a user-defined accumulator, and normal functions to describe the extraction of a match into the accumulator.

There are data structures that represent matchers on each of the four TokenTree enum arms, and represent combinators on arbitrary matchers.

There is a `transform` function that uses these same datastructures to recursively match and transform a TokenStream.

## Testing

Due to some complications with testing functions that manipulate TokenStreams that need to be exported as functions (that is, that they can only run under proc-macro context, but I want to export them as functions so this can't be a proc-macro lib) the tests are in a sub-crate that is excluded from `cargo publish`, at `nested-crates-test/using-the-proc/`. You can run the tests by running `cargo test` in that subdirectory.

## AI/LLM Policy

No use of AI or LLMs at all. No contributions involving or using such tools. No contributors who use such tools, even in other projects.

No lead poisoning / brainworms please.

