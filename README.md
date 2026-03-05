# R-Organizer

A high-performance CLI utility that categorizes and moves files into structured folders - built to demonstrate safe systems programming in Rust.

---

## Why This Exists

Every developer ends up with a chaotic Downloads folder. Most solutions are Python scripts that crash on non-UTF-8 filenames or load thousands of entries into memory at once. This project solves the same problem correctly - using Rust's ownership model, lazy iterators, and a strict no-panic I/O policy.

---


### 1. The `Path` vs `OsStr` Problem

`String` in Rust is guaranteed UTF-8. File systems are not - especially on Windows and Unix where filenames can contain arbitrary bytes.

- Owned paths use `PathBuf`; borrowed paths use `&Path`
- Filenames are processed as `OsStr` to avoid unnecessary allocations and prevent panics on non-UTF-8 characters
- `.to_str().unwrap()` is never used

### 2. Error Handling - No-Panic Policy

No `unwrap()` anywhere in I/O paths. Every fallible operation returns a `Result`.

- The `?` operator propagates errors to the caller
- If a single file move fails (e.g., `Permission Denied`), the loop continues to the next file - the process does not crash

### 3. I/O Performance

- `std::fs::read_dir` returns a lazy iterator - 10,000 file entries are never loaded into memory at once
- Paths are passed by reference wherever ownership transfer is unnecessary, avoiding heap allocations

---

## Getting Started

**Prerequisites:** Rust toolchain via [rustup](https://rustup.rs)

```bash
git clone https://github.com/your-username/r-organizer.git
cd r-organizer
cargo build --release
./target/release/r-organizer --path ~/Downloads
```

---

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature-name`
3. Make your changes — ensure `cargo clippy` passes with no warnings
4. Run `cargo fmt` before committing
5. Commit with a clear message: `git commit -m "feat: describe what and why"`
6. Push and open a Pull Request against `main`

**Before opening a PR, confirm:**
- [ ] No `unwrap()` or `expect()` on I/O operations
- [ ] No new compiler warnings
- [ ] Logic is documented inline if non-obvious

---

## License

MIT