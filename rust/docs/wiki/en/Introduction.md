
# Introduction

variants_de is a crate for Rust to deserilize Rust data from TOML-formatted like files/streams based on variant context.
This guide introduces you to the core and crate wraps of variants_de. After reading this guide, it would be very easy to leverage variants_de 
to build Rust apps with different features/behaviors based on configurations with a context.

## Audience

Readers are assumed to be familiar with the Rust programming language. Readers new to Rust are encouraged to read the [Rust Book](https://doc.rust-lang.org/book/). 
This guide also assumes readers to be familiar with some popular Rust web frameworks like Actix Web, Rocket...

## Foreword

variants_de' design is centered around two core prioritties:

  * **Security, correctness**

    Security and correctness should not come at the cost of a degraded developer experience by leveraging variants_de. variants_de are fully covered by different types
    of tests to make sure the correctness.

  * **Easy to use.**

    variants_de provides **self-contained** wraps which requires minimium effort to use it in your apps.

You would find these two embedded in the core and wraps.

---
### [Next: Variants_de Types and Configuration format](./Types_and_config.md)