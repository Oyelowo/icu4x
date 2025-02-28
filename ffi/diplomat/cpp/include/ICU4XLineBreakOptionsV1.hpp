#ifndef ICU4XLineBreakOptionsV1_HPP
#define ICU4XLineBreakOptionsV1_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XLineBreakOptionsV1.h"

#include "ICU4XLineBreakRule.hpp"
#include "ICU4XWordBreakRule.hpp"

/**
 * A destruction policy for using ICU4XLineBreakOptionsV1 with std::unique_ptr.
 */
struct ICU4XLineBreakOptionsV1Deleter {
  void operator()(capi::ICU4XLineBreakOptionsV1* l) const noexcept {
    capi::ICU4XLineBreakOptionsV1_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `LineBreakOptions`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineBreakOptions.html) for more information.
 */
struct ICU4XLineBreakOptionsV1 {
 public:
  ICU4XLineBreakRule line_break_rule;
  ICU4XWordBreakRule word_break_rule;
  bool ja_zh;
};


#endif
