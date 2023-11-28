#pragma once
#include <iostream>
#include <string>
#include <variant>
#include <vector>

class ScratchValue;

typedef std::vector<ScratchValue> ScratchList;

enum ValueType {
  NUMBER,
  STRING,
  EMPTY,
  LIST,
};

#define COND_OP(op)                                                            \
  bool operator op(ScratchValue other) {                                       \
    if ((_type == NUMBER || _type == EMPTY) && other._type == NUMBER)          \
      return (_type == EMPTY ? 0 : get<double>()) op other.get<double>();      \
    if ((_type == STRING || _type == EMPTY) && other._type == STRING)          \
      return (_type == EMPTY ? "" : get<std::string>())                        \
          op other.get<std::string>();                                         \
    if ((_type == STRING || _type == EMPTY) && other._type == NUMBER)          \
      return (_type == EMPTY ? "" : get<std::string>())                        \
          op std::to_string(other.get<double>());                              \
    return false;                                                              \
  }

class ScratchValue {
public:
  ScratchValue() : _type(EMPTY) {}
  ScratchValue(int number) : _value((double)number), _type(NUMBER) {}
  ScratchValue(double number) : _value(number), _type(NUMBER) {}
  ScratchValue(std::string string) : _value(string), _type(STRING) {}
  ScratchValue(const char *string) : _value(string), _type(STRING) {}

  ScratchValue(ScratchList list) : _value(list), _type(LIST) {}

  COND_OP(==);
  COND_OP(!=);
  COND_OP(>);
  COND_OP(<);

  ScratchValue operator+(ScratchValue other) {
    if ((_type == NUMBER || _type == EMPTY) && other._type == NUMBER)
      return (_type == EMPTY ? 0 : get<double>()) + other.get<double>();
    if ((_type == STRING || _type == EMPTY) && other._type == STRING)
      return (_type == EMPTY ? "" : get<std::string>()) +
             other.get<std::string>();
    return {};
  }

  ScratchValue operator-(ScratchValue other) {
    if ((_type == NUMBER || _type == EMPTY) && other._type == NUMBER)
      return (_type == EMPTY ? 0 : get<double>()) - other.get<double>();
    return {};
  }
  ScratchValue operator*(ScratchValue other) {
    if ((_type == NUMBER || _type == EMPTY) && other._type == NUMBER)
      return (_type == EMPTY ? 0 : get<double>()) * other.get<double>();
    return {};
  }

  ScratchValue operator/(ScratchValue other) {
    if ((_type == NUMBER || _type == EMPTY) && other._type == NUMBER)
      return (_type == EMPTY ? 0 : get<double>()) / other.get<double>();
    return {};
  }

  ScratchValue operator+=(ScratchValue rhs) {
    if (_type == STRING) {
      _type = NUMBER;
      _value = 0.0;
    }

    get<double>() += rhs.get<double>();

    return *this;
  }

  ScratchValue operator[](int index) {
    if (_type == STRING) {
      return std::string(1, get<std::string>()[index]);
    }
    if (_type == LIST)
      return get<ScratchList>()[index];
    return ScratchValue{};
  }

  size_t length() {
    if (_type == STRING)
      return get<std::string>().length();
    if (_type == LIST)
      return get<ScratchList>().size();
    return 0;
  }

  void print(const char *string) {

    switch (_type) {
    case NUMBER:
      std::cout << string << get<double>() << std::endl;
      break;
    case STRING:
      std::cout << string << get<std::string>() << std::endl;
      break;
    case LIST:
      std::cout << string << "[";
      for (auto elem : get<ScratchList>()) {
        elem.print(",");
      }
      std::cout << "]" << std::endl;
      break;
    }
  }

  template <typename T> T &get() { return std::get<T>(_value); }

private:
  std::variant<double, std::string, ScratchList> _value;
  ValueType _type;
};