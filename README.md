Simple wrapper in Rust (to try out FFI really) for the [Haraka](https://github.com/kste/haraka) compression function.

Portability of this is not great as I didn't use [gcc-rs](https://github.com/alexcrichton/gcc-rs) to limit dependencies.

> Note: You _need_ a processor with the appropriate SSE and AES instructions. For example my 2nd gen i3 can't run it but my 3rd gen i5 can.
