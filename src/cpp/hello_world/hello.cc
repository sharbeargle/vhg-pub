/**
 * clang++-13 -std=c++17 hello.cc lib/car.cc
 **/

#include <iostream>
#include <memory>     // for std::unique_ptr
#include <utility>    // for std::move
#include "lib/car.h"

void print_name(std::unique_ptr<vls::Car> c) {
  std::cout << c->get_name() << "\n";
}

int main(int argc, char* argv[]) {

  auto c = std::make_unique<vls::Car>("Honda");

  print_name(std::move(c));

	return 0;
}
