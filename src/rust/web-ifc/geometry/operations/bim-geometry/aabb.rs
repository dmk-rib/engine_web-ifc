//! Auto-translated from `src/cpp/web-ifc/geometry/operations/bim-geometry/aabb.h`.
//!
//! This Rust module mirrors the original C++ file layout for parity and future
//! maintenance. The original C++ source is embedded below for reference while
//! the Rust translation is incrementally implemented.

#![allow(dead_code, unused_variables, clippy::all)]

// NOTE: Performance-sensitive code should prefer slices, iterators, and
//       preallocation via Vec::with_capacity where appropriate.

const CPP_SOURCE: &str = r###"
#include <vector>
#include <algorithm>
#include <glm/glm.hpp>
#include "buffers.h"

using Vec = glm::dvec3;

#pragma once

namespace bimGeometry {
    
    struct AABB
    {
        uint32_t index;
        Vec min = Vec(DBL_MAX, DBL_MAX, DBL_MAX);
        Vec max = Vec(-DBL_MAX, -DBL_MAX, -DBL_MAX);
        Vec center = Vec();

        bool intersects(const AABB& other) const;
        bool contains(const Vec& pos) const;
        void merge(const AABB& other);
        void merge(const glm::dvec3& other);
        bool Intersect(const Vec& origin, const Vec& dir) const;
        void SetValues(double minX, double minY, double minZ, double maxX, double maxY, double maxZ);
        Buffers GetBuffers();
    };
}
"###;

// TODO: Replace CPP_SOURCE with a full Rust implementation matching the C++ API.
