#pragma once
#include <string>
#include <variant>
#include <vector>

class ScratchValue;

typedef std::vector<ScratchValue> ScratchList;

enum ValueType {
  NUMBER,
  STRING,
  EMPTY,
};

#define OP(op)                                                                 \
  ScratchValue operator op(ScratchValue other) {                               \
    if ((_type == NUMBER || _type == EMPTY) && other._type == NUMBER)          \
      return (_type == EMPTY ? 0 : get<double>()) op other.get<double>();      \
    return {};                                                                 \
  }

#define COND_OP(op)                                                            \
  bool operator op(ScratchValue other) {                                       \
    if ((_type == NUMBER || _type == EMPTY) && other._type == NUMBER)          \
      return (_type == EMPTY ? 0 : get<double>()) op other.get<double>();      \
    return false;                                                              \
  }

class ScratchValue {
public:
  ScratchValue() : _type(EMPTY) {}
  ScratchValue(double number) : _value(number), _type(NUMBER) {}
  ScratchValue(std::string string) : _value(string), _type(STRING) {}
  ScratchValue(ScratchList list) : _value(list) {}

  COND_OP(==);
  OP(+);
  OP(-);
  OP(*);
  OP(/);

  ScratchValue operator+=(ScratchValue rhs) {
    if (_type == STRING) {
      _type = NUMBER;
      _value = 0.0;
    }

    get<double>() += rhs.get<double>();

    return *this;
  }

  template <typename T> T &get() { return std::get<T>(_value); }

private:
  std::variant<double, std::string, ScratchList> _value;
  ValueType _type;
};