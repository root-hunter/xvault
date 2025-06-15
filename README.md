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

```
CA_UID = UIDv4()
DEVICE_UID = UIDv4()

USER_UID = UIDv5(CA_UID, USERNAME)
VOLUME_UID = UIDv5(USER_UID, DEVICE_UID+ABS_PATH)

FILE_UID = UIDv5(USER_UID, ABS_VIRTUAL_PATH)
CHUNK_UID = UIDv5(FILE_UID, CHUNK_NUMBER)

```

## 🙏 Acknowledgements

This project uses various external datasets for testing and validation.

Special thanks to:

- [pfalcon/canterbury-corpus](https://github.com/pfalcon/canterbury-corpus) — a collection of small files widely used for compression algorithm benchmarking and validation. It is used in XVault to test binary and textual file integrity through automated tests.