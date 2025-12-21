//! Auto-translated from `src/cpp/web-ifc/geometry/operations/boolean-utils/obj-exporter.h`.
//!
//! This Rust module mirrors the original C++ file layout for parity and future
//! maintenance. The original C++ source is embedded below for reference while
//! the Rust translation is incrementally implemented.

#![allow(dead_code, unused_variables, clippy::all)]

// NOTE: Performance-sensitive code should prefer slices, iterators, and
//       preallocation via Vec::with_capacity where appropriate.

const CPP_SOURCE: &str = r###"
#pragma once

#include <string>
#include <sstream>
#include <glm/glm.hpp>

#include "geometry.h"
#include "util.h"

namespace fuzzybools
{
	static std::string ToObj(const Geometry& geom, size_t& offset, glm::dmat4 transform = glm::dmat4(1), double inputScale = 1.0)
	{
		std::stringstream obj;

		double scale = inputScale;

		for (uint32_t i = 0; i < geom.numPoints; i++)
		{
			glm::dvec4 t = transform * glm::dvec4(geom.GetPoint(i), 1);
			obj << "v " << t.x * scale << " " << t.y * scale << " " << t.z * scale << "\n";
		}

		for (uint32_t i = 0; i < geom.numFaces; i++)
		{
			Face f = geom.GetFace(i);
			obj << "f " << (f.i0 + 1 + offset) << "// " << (f.i1 + 1 + offset) << "// " << (f.i2 + 1 + offset) << "//\n";
		}

		offset += geom.numPoints;

		return obj.str();
	}

	static void DumpGeometry(const Geometry& geom, std::wstring filename)
	{
		size_t offset = 0;
		writeFile(filename, ToObj(geom, offset));
	}
}

"###;

// TODO: Replace CPP_SOURCE with a full Rust implementation matching the C++ API.
