#include "car.h"
#include <iostream>

namespace vls {

Car::Car(std::string n) : name{n} {}

void Car::drive() {
  std::cout << "vroooom\n";
}

std::string_view Car::get_name() {
  return name;
}

} // namespace vls
