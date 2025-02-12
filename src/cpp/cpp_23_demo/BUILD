# bazel build --subcommands --verbose_failures cpp_23_demo:main
cc_binary(
  name = "main",
  srcs = ["main.cc"],
)

cc_test(
  name = "hello_test",
  size = "small",
  srcs = ["hello_test.cc"],
  deps = [
    "@googletest//:gtest",
    "@googletest//:gtest_main",
  ],
)