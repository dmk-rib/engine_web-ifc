//! Auto-translated from `src/cpp/web-ifc/geometry/operations/bim-geometry/circularSweep.cpp`.
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
#include "circularSweep.h"
#include "epsilons.h"
#include "geometry.h"
#include "utils.h"

using Vec = glm::dvec3;

namespace bimGeometry {

    Buffers CircularSweep::GetBuffers()
    {
        Buffers buffers;

        // Canviar la funcio revolution
        Geometry geom = SweepCircular(scaling, closed, profilePoints, radius, directrix, initialDirectrixNormal, rotate90);

        for (int r = 0; r < geom.numFaces; r++)
        {
            bimGeometry::Face f = geom.GetFace(r);
            buffers.AddTri(geom.GetPoint(f.i0), geom.GetPoint(f.i1),  geom.GetPoint(f.i2));
        }

        return buffers;
    }

    void CircularSweep::SetValues(double scaling_, bool closed_, std::vector<double> profilePoints_, double radius_, std::vector<double> directrix_, std::vector<double> initialDirectrixNormal_, bool rotate90_) {
        profilePoints.clear();
        for (size_t i = 0; i + 2 < profilePoints_.size(); i += 3) {
            profilePoints.emplace_back(profilePoints_[i], profilePoints_[i + 1], profilePoints_[i + 2]);
        }
        directrix.clear();
        for (size_t i = 0; i + 2 < directrix_.size(); i += 3) {
            directrix.emplace_back(directrix_[i], directrix_[i + 1], directrix_[i + 2]);
        }
        initialDirectrixNormal = glm::dvec3(initialDirectrixNormal_[0], initialDirectrixNormal_[1], initialDirectrixNormal_[2]);
        scaling = scaling_;
        closed = closed_;
        radius = radius_;
        rotate90 = rotate90_;
    }
}

"###;

// TODO: Replace CPP_SOURCE with a full Rust implementation matching the C++ API.
