# Dependency Mapping (C++ → Rust)

- `uuid` (C++ uuid.h / uuids::basic_uuid_random_generator) → `uuid` crate
  - Reason: widely used, maintained, fast UUID v4 generation, permissive license.
- `mapbox/earcut.hpp` → `earcutr` crate
  - Reason: direct Rust port of earcut triangulation with similar API and performance.
