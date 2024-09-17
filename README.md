# MPL common constants

This crate provides shared common variables used across different crates.

As a general convention, public keys are stored in raw format—an array of u8 values.

To assist with generating this array, a simple CLI tool is included. You can run the following command to use it:

`cargo r --bin mpl_common_constants --features bs58 -- YOUR_BASE58_PUBLIC_KEY_HERE`