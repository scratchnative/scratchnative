#include "scratch2native.hpp"
#include <fmt/format.h>

#include <utility>

using namespace scratch2native;

void scratch2native::compile_scratch(json j, fmt::ostream &out, bool freestanding)
{
    Compiler compiler{std::move(j), out, freestanding};

    compiler.compile();
}

void Compiler::initialize_from_json()
{
    for (auto [key, val] : _json.at("targets")[0].at("variables").items())
    {
        // Variables are uninitialized for now, we'll get their type later...
        _variables[key] = ScratchValue{};
    }

    for (auto [key, val] : _json.at("targets")[0].at("blocks").items())
    {
        if (val.at("next").is_null() && val.at("parent").is_null() && val.at("opcode").is_null())
            continue;

        _blocks[key] = val;
    }

    for (auto [key, val] : _blocks)
    {
        if (val.at("parent").is_null() && val.at("opcode").get<std::string>() != "procedures_definition")
        {
            _root_block = val;
            break;
        }
    }
}

void Compiler::compile()
{
    initialize_from_json();

    _out.print("#include <stdint.h>\n#include <stddef.h>\n#include <stdbool.h>\n");

    if (!_freestanding)
    {
        _out.print("#include <stdio.h>\n#include <vector>\n#include <string>\n#include <cmath>\n#include <ctime>\n");
    }

    for (auto [name, block] : _blocks)
    {
        if (block.at("opcode").get<std::string>() == "procedures_definition")
            codegen_block(block);
    }

    json block_to_execute = _root_block;

    codegen_block_chain(_root_block);

    _out.print("\n}}\n");

    if (!_freestanding)
    {
        _out.print("int main()\n{{\nscratch_main();\nreturn 0;\n}}\n");
    }
}

void Compiler::codegen_block_chain(const json &root)
{
    auto block_to_execute = root;

    if (block_to_execute.is_null())
        return;

    while (true)
    {
        codegen_block(block_to_execute);

        if (block_to_execute.at("next").is_null())
            break;

        block_to_execute = _blocks[block_to_execute.at("next")];
    }
}

void Compiler::codegen_block(const json &block)
{
    if (block.is_null())
        return;

    auto opcode = block.at("opcode").get<std::string>();

    fmt::print("opcode: {}\n", opcode);

    if (_opcode_callbacks.find(opcode) == _opcode_callbacks.end())
    {
        fmt::print("Unimplemented opcode: {}", opcode);
    }
    else
    {
        _opcode_callbacks.at(opcode)(block);
    }
}

void Compiler::event_whenflagclicked(const json &block)
{
    (void)block;

    _out.print("void scratch_main()\n{{\n\n");

    if (!_freestanding)
    {
        _out.print("srand(time(NULL));\n");
    }
}

void Compiler::codegen_value(const json &val)
{
    auto type = static_cast<ScratchValueType>(val[0].get<int>());

    if (_lvalue)
    {
        _lvalue->type = type;
    }

    switch (type)
    {
    case POSITIVE_INTEGER:
    case NUMBER:
    {
        _out.print("{}", val[1].get<std::string>());
        break;
    }

    case STRING:
    {
        auto str = val[1].get<std::string>();

        bool is_number = true;

        try
        {
            std::stoi(str);
        }
        catch (std::exception &e)
        {
            is_number = false;
        }

        if (is_number)
        {
            if (_lvalue)
                _lvalue->type = NUMBER;

            _out.print("{}", str);
        }
        else
        {
            _out.print("\"{}\"", str);
        }
        break;
    }

    case VARIABLE:
    {
        _out.print("{}", _variables[val[2].get<std::string>()].pretty_name);
        break;
    }

    default:
        fmt::print("TODO add support for {}\n", type);
        break;
    }
}

void Compiler::control_if(const json &block)
{
    _out.print("if (");

    codegen_block(_blocks[block.at("inputs").at("CONDITION")[1].get<std::string>()]);

    _out.print(")\n{{\n");

    codegen_block_chain(_blocks[block.at("inputs").at("SUBSTACK")[1].get<std::string>()]);

    _out.print("\n}}\n");
}

void Compiler::control_repeat(const json &block)
{
    _out.print("for (int i = 0; i < ");

    codegen_expr(block.at("inputs").at("TIMES"));

    _out.print("; i++");

    _out.print(")\n{{\n");

    codegen_block_chain(_blocks[block.at("inputs").at("SUBSTACK")[1].get<std::string>()]);

    _out.print("\n}}\n");
}

void Compiler::procedures_call(const json &block)
{

    auto args_ids_str = block.at("mutation").at("argumentids").get<std::string>();
    auto args_ids = json::parse(args_ids_str);
    auto func_name_str = block.at("mutation").at("proccode").get<std::string>();

    bool poke = false;

    if (func_name_str == "poke8 %s %s")
    {
        poke = true;
        _out.print("*(uint8_t*)((uintptr_t)");
    }

    else if (func_name_str == "poke16 %s %s")
    {
        poke = true;
        _out.print("*(uint16_t*)((uintptr_t)");
    }

    else if (func_name_str == "poke32 %s %s")
    {
        poke = true;
        _out.print("*(uint32_t*)((uintptr_t)");
    }

    else if (func_name_str == "poke64 %s %s")
    {
        poke = true;
        _out.print("*(uint64_t*)((uintptr_t)");
    }

    if (poke)
    {
        codegen_expr(block.at("inputs").at(args_ids[0].get<std::string>()));
        _out.print(") = ");
        codegen_expr(block.at("inputs").at(args_ids[1].get<std::string>()));
        _out.print(";\n");
    }

    else
    {
        if (func_name_str.find(':') != std::string::npos)
            func_name_str = func_name_str.substr(func_name_str.find(':') + 2, func_name_str.size());

        auto function_name = space2underscore(func_name_str.substr(0, func_name_str.find('%') - 1));

        _out.print("{}(", function_name);

        for (size_t i = 0; i < args_ids.size(); i++)
        {
            codegen_expr(block.at("inputs").at(args_ids[i].get<std::string>()));

            if (i != args_ids.size() - 1)
            {
                _out.print(",");
            }
        }

        _out.print(");");
    }
}

void Compiler::procedures_prototype(const json &block)
{
    auto func_name_str = block.at("mutation").at("proccode").get<std::string>();
    auto func_name = func_name_str.substr(0, func_name_str.find('%') - 1);
    auto args_names_str = block.at("mutation").at("argumentnames").get<std::string>();
    auto args_names = json::parse(args_names_str);

    auto arg_type_to_c_type = [&](std::string type)
    {
        if (type == "STRING")
            _out.print("const char *");
        else if (type == "INT")
            _out.print("int");
    };

    if (func_name != "poke8" && func_name != "poke16" && func_name != "poke32" && func_name != "poke64")
    {
        if (func_name.find(':') == std::string::npos)
        {
            _out.print("void");
            _out.print("{}", func_name);
        }

        else
        {
            arg_type_to_c_type(func_name.substr(0, func_name.find(':')));
            _out.print("{}", func_name.substr(func_name.find(':') + 1, func_name.size()));
        }

        _out.print("(");
    }

    for (auto arg : args_names)
    {
        auto arg_str = arg.get<std::string>();

        if (arg_str.find(':') == std::string::npos)
            break;

        auto arg_type = arg_str.substr(0, arg_str.find(':'));

        arg_type_to_c_type(arg_type);

        _out.print("{}", arg_str.substr(arg_str.find(':') + 1, arg_str.size()));
    }

    _out.print(")");
}

void Compiler::procedures_definition(const json &block)
{
    if (block.at("next").is_null())
    {
        _out.print("extern \"C\" ");
    }

    codegen_block(_blocks[block.at("inputs").at("custom_block")[1].get<std::string>()]);
    if (block.at("next").is_null())
    {
        _out.print(";\n");
    }

    else
    {
        _out.print("\n{{");
        codegen_block_chain(block);
        _out.print("\n}}");
    }
}
