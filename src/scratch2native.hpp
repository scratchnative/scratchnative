#ifndef SCRATCHNATIVE_SCRATCH2NATIVE_HPP
#define SCRATCHNATIVE_SCRATCH2NATIVE_HPP
#include <fmt/os.h>
#include <nlohmann/json.hpp>
#include <unordered_map>
#include <utility>
#include <variant>

using json = nlohmann::json;

namespace scratch2native
{

enum ScratchValueType
{
    UNKNOWN = 0,
    NUMBER = 4,
    POSITIVE_NUMBER = 5,
    POSITIVE_INTEGER = 6,
    INTEGER = 7,
    ANGLE = 8,
    COLOR = 9,
    STRING = 10,
    BROADCAST = 11,
    VARIABLE = 12,
};

struct ScratchValue
{
    std::string pretty_name;

    ScratchValueType type;

    std::variant<int64_t, int32_t, uint32_t, std::string> data;
};

#define ADD_CALLBACK(name) _opcode_callbacks[#name] = std::bind(&Compiler::name, this, std::placeholders::_1);

struct Compiler
{
    Compiler(json j, fmt::ostream &out, bool freestanding) : _json(std::move(j)), _out(out), _freestanding(freestanding)
    {
        ADD_CALLBACK(event_whenflagclicked);
        ADD_CALLBACK(data_setvariableto);
        ADD_CALLBACK(data_showvariable);

        ADD_CALLBACK(operator_add);
        ADD_CALLBACK(operator_substract);
        ADD_CALLBACK(operator_multiply);
        ADD_CALLBACK(operator_divide);

        ADD_CALLBACK(operator_lt);
        ADD_CALLBACK(operator_equals);
        ADD_CALLBACK(operator_gt);
        ADD_CALLBACK(operator_and);
        ADD_CALLBACK(operator_or);
        ADD_CALLBACK(operator_not);
        ADD_CALLBACK(operator_random);
        ADD_CALLBACK(operator_join);
        ADD_CALLBACK(operator_letter_of);
        ADD_CALLBACK(operator_length);
        ADD_CALLBACK(operator_contains);
        ADD_CALLBACK(operator_mod);
        ADD_CALLBACK(operator_round);

        ADD_CALLBACK(control_if);
        ADD_CALLBACK(control_repeat);

        ADD_CALLBACK(procedures_call);
        ADD_CALLBACK(procedures_definition);
        ADD_CALLBACK(procedures_prototype);
    }

    ~Compiler() = default;

    void compile();

private:
    void initialize_from_json();
    void codegen_block(const json &json);

    void event_whenflagclicked(const json &block);

    void data_setvariableto(const json &block);
    void data_showvariable(const json &block);

    void operator_add(const json &block);
    void operator_substract(const json &block);
    void operator_multiply(const json &block);
    void operator_divide(const json &block);

    void operator_lt(const json &block);
    void operator_equals(const json &block);
    void operator_gt(const json &block);
    void operator_and(const json &block);
    void operator_or(const json &block);
    void operator_not(const json &block);
    void operator_random(const json &block);
    void operator_join(const json &block);
    void operator_letter_of(const json &block);
    void operator_length(const json &block);
    void operator_contains(const json &block);
    void operator_mod(const json &block);
    void operator_round(const json &block);

    void control_if(const json &block);
    void control_repeat(const json &block);

    void procedures_call(const json &block);
    void procedures_definition(const json &block);
    void procedures_prototype(const json &block);

    void codegen_value(const json &val);
    void codegen_block_chain(const json &root);

    static inline std::string space2underscore(std::string text)
    {
        std::replace(text.begin(), text.end(), ' ', '_');
        return text;
    }

    inline void codegen_expr(const json &array)
    {
        if (array[1].is_string())
        {
            codegen_block(_blocks[array[1].get<std::string>()]);
        }
        else if (array[1].is_array())
        {
            codegen_value(array[1]);
        }
    }

    std::unordered_map<std::string, ScratchValue> _variables{};
    std::unordered_map<std::string, json> _blocks{};
    std::unordered_map<std::string, std::function<void(const json &)>> _opcode_callbacks{};

    json _root_block{}, _json{};
    fmt::ostream &_out;
    ScratchValue *_lvalue = nullptr;

    bool _freestanding = false;
};

void compile_scratch(json j, fmt::ostream &out, bool freestanding);
} // namespace scratch2native

#endif // SCRATCHNATIVE_SCRATCH2NATIVE_HPP
