<p align="center">
  <h1 align="center">veedo-rs</h1>
</p>

**Rust implementation of [VeeDo](https://github.com/starkware-libs/veedo), a STARK-based Verifiable Delay Function**

## Performance

A major goal of `veedo-rs` is to be the fastest VeeDo implementation, enabling it to act as a reference for calculating the required iterations needed for any specific amount of delay. Therefore, performance of the library is actively monitored. A patch would not be merged if it introduces performance regression.

To run the benchmark of computing 100,000 iterations (in both directions):

```console
cargo bench
```

The following tables includes benchmark results on a curated list of hardware. For now, it's manually maintained and updated only on code changes that affect performance and on new Rust releases.

The current numbers have been generated with Rust `1.81.0`.

| Processor          | Ararchitecture | OS                 | Duration (100k iterations) | Inverse   |
| ------------------ | -------------- | ------------------ | -------------------------- | --------- |
| Apple M3 Max       | ARM64          | macOS Sequoia 15.0 | 80.036 ms                  | 2.8085 ms |
| Snapdragon 8 Gen 2 | ARM64          | Android 14         | 118.18 ms                  | 4.7796 ms |

> [!TIP]
>
> Contributors do not need to update this table. The maintainer would add a commit for updating the numbers before merging any performance-related PR.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
