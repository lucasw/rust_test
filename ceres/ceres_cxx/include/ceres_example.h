#pragma once
#include "rust/cxx.h"
#include <memory>
#include <vector>

namespace org {
namespace ceres_example {

class CeresExample {
public:
  CeresExample();

  void run(const rust::Vec<double>& vals) const;

private:
  class impl;
  std::shared_ptr<impl> impl;
};

std::unique_ptr<CeresExample> new_ceres_example();

} // namespace ceres_example
} // namespace org
