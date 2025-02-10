cc_library(
  name = "car",
  srcs = ["car.cc"],
  hdrs = ["car.h"],
  visibility = [":hello_world_pkgs"],
)

package_group(
  name = "hello_world_pkgs",
  packages = [
    "//hello_world/...",
  ],
)
