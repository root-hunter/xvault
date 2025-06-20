# XVault

**XVault** is a distributed, chunk-based file system written in Rust, designed for resilience, performance, and extensibility. It includes support for parity chunking (for fault tolerance), binary and textual file comparison, export/import mechanisms, and fine-grained user identification using UUIDs.

![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)

## ✨ Features

- 🔐 **Distributed Storage** — Designed to work over multiple nodes with chunk-based architecture.
- 💾 **Chunking System** — Files are split into addressable, verifiable chunks.
- 🧩 **Parity Support** — Future-proof design with parity chunks using SIMD-accelerated Reed-Solomon codes.
- 🧬 **UUID-based Identity** — Each user and device is identified via strong UUIDs.
- 🔄 **Import / Export** — Files can be exported and compared with original ones (bit-level integrity checks).
- 📄 **Binary & Text Comparison** — Accurate file diffing using `similar` and raw byte checks.
- 🚀 **Performance-Oriented** — Efficient I/O operations with `BufReader`, chunk streaming, and optimized serialization.
- 🧪 **Test Automation** — Macro-based test generation to validate file operations automatically.

## 📁 Folder Structure

```txt
.
├── assets/         # Original tests input files (text/bin)
├── tmp/            # Temporary virtual test volume files
├── exports/        # Exported tests output files
├── tests/          # Automated tests (text/bin)
├── src/
│   └── engine/
│       └── xfile.rs   # Core file logic
├── Cargo.toml
└── README.md

```

## 🙏 Acknowledgements

This project uses various external datasets for testing and validation.

Special thanks to:

- [pfalcon/canterbury-corpus](https://github.com/pfalcon/canterbury-corpus) — a collection of small files widely used for compression algorithm benchmarking and validation. It is used in XVault to test binary and textual file integrity through automated tests.


## Test Coverage
```
|| Uncovered Lines:
|| src/engine/chunk.rs: 50-52
|| src/engine/device.rs: 43, 64, 67-69, 71-73, 77, 80-81, 84, 88, 109
|| src/engine/utils.rs: 25, 50, 53, 57
|| src/engine/volume.rs: 61, 133, 165, 177, 205, 226, 258, 267, 340, 396-397, 400-401, 403-404, 406-409, 411, 415, 418
|| src/engine/xfile.rs: 56-57, 112, 116-117, 119, 125, 141, 147
|| Tested/Total Lines:
|| src/engine/chunk.rs: 6/9 +0.00%
|| src/engine/device.rs: 22/36 +0.00%
|| src/engine/utils.rs: 27/31 +0.00%
|| src/engine/volume.rs: 174/196 +0.00%
|| src/engine/xfile.rs: 44/53 +0.00%
|| 
84.00% coverage, 273/325 lines covered
```

## TODO

- Queue for write and read to/from file volume