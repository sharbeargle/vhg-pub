#ifndef VLS_CAR
#define VLS_CAR

#include <string>
#include <string_view>

namespace vls {

class Car {
  private:
    std::string name;

  public:
    Car(std::string n);
    void drive();
    std::string_view get_name();
};

} // namespace vls

#endif // VLS_CAR
