
# TODO

## elfprobe-core

- [ ] Make `elfprobe-core` a library and `elfprobe-cli` a binary.
- [ ] Rewrite `BytesError` as `ElfError`.
- [X] Rewrite `ChunkError` and `ParseHexError`.
- [X] `#[derive(Pod)]`.
- [X] Make Rust workspace.
- [X] Remove `syn` and `quote`.
- [ ] `std::concat_idents!()`

## elfprobe-macro

- [ ] Make a wrapper of `Entry`/`TokenTree` that stores to_string() returned value? (useless for far)
- [ ] <https://doc.rust-lang.org/proc_macro/struct.Diagnostic.html> (when stable)
- [X] Parse `where` .
- [ ] Parse `Type`.
- [ ] Parse must return an error.
