#include "scratch2native.hpp"

using namespace scratch2native;

void Compiler::data_setvariableto(const json &block)
{
    auto variable_name = block.at("fields").at("VARIABLE")[1].get<std::string>();
    auto unmangled_variable_name = space2underscore(block.at("fields").at("VARIABLE")[0].get<std::string>());

    auto &var = _variables[variable_name];

    if (var.pretty_name.empty())
    {
        var.pretty_name = unmangled_variable_name;
    }

    bool was_unknown = false;

    if (var.type == ScratchValueType::UNKNOWN)
    {
        was_unknown = true;
    }

    auto value_arr = block.at("inputs").at("VALUE");

    if (was_unknown)
    {
        _out.print("auto {} = ", unmangled_variable_name);
    }

    else
    {
        _out.print("{} = ", unmangled_variable_name);
    }

    if (value_arr[1].is_array())
    {
        auto type = static_cast<ScratchValueType>(value_arr[1][0].get<int>());

        if (was_unknown)
        {
            var.type = type;
        }

        if (var.type != type && type != VARIABLE)
        {
            fmt::print("Error: mismatched types for variable '{}'", unmangled_variable_name);
            exit(-1);
            return;
        }

        auto rvalue_type = static_cast<ScratchValueType>(value_arr[1][0].get<int>());

        if (rvalue_type == VARIABLE)
        {
            auto _var = _variables[value_arr[1][2].get<std::string>()];

            if (_var.type != var.type)
            {
                fmt::print("Error: mismatched types for variable '{}'", unmangled_variable_name);
                exit(-1);
                return;
            }
        }

        _lvalue = &_variables[variable_name];

        codegen_expr(value_arr);

        _out.print(";\n");
    }

    else if (value_arr[1].is_string())
    {
        _lvalue = &_variables[variable_name];

        codegen_block(_blocks[value_arr[1].get<std::string>()]);

        if (_lvalue->type != var.type)
        {
            fmt::print("Error: mismatched types for variable '{}'", unmangled_variable_name);
            exit(-1);
            return;
        }

        _out.print(";\n");
    }
}

void Compiler::data_showvariable(const json &block)
{
    auto var = _variables[block.at("fields").at("VARIABLE")[1]];

    if (!_freestanding)
    {
        _out.print("printf(\"{}\\n\", {});\n", var.type == NUMBER ? "%d" : "", space2underscore(block.at("fields").at("VARIABLE")[0].get<std::string>()));
    }
}
