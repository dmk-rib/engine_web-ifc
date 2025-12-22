# Rust IFC schema generator

This crate provides a Rust implementation of the IFC schema generator currently authored in TypeScript under `src/schema-generator`.

## TS deps → Rust deps mapping

| TypeScript dependency | Purpose | Rust crate |
| --- | --- | --- |
| `fs` | Read schema files and write outputs | `std::fs` |
| `path` | File path handling | `std::path` |
| `ts-node` (runtime) | Execute generator | Rust binary target (`std::env::args`) |
