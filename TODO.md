
# TODO

## elfprobe-core

- [ ] Make `elfprobe-core` a library and `elfprobe-cli` a binary.
- [ ] Rewrite `BytesError` as `ElfError`.
- [X] Rewrite `ChunkError` and `ParseHexError`.
- [ ] `#[derive(Pod)]`.
- [ ] Make Rust workspace.
- [X] Remove `syn` and `quote`.
- [ ] `std::concat_idents!()`

## elfprobe-macro

- [ ] Make a wrapper of `Entry`/`TokenTree` that stores to_string() returned value?
