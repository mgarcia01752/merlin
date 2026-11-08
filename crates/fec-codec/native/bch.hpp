/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#pragma once
#include "rust/cxx.h"

namespace pnm_fec {
struct BchDecoded;
rust::Vec<std::uint8_t> bch_encode(rust::Slice<const std::uint8_t> message);
BchDecoded bch_decode(rust::Slice<const std::uint8_t> codeword);
}
