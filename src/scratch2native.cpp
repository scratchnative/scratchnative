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
        if (val.at("next").is_null() && val.at("parent").is_null())
            continue;

        _blocks[key] = val;
    }

    for (auto [key, val] : _blocks)
    {
        if (val.at("parent").is_null())
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

    if (block.at("mutation").at("proccode").get<std::string>() == "poke8 %s %s")
    {
        _out.print("*(uint8_t*)((uintptr_t)");

        codegen_expr(block.at("inputs").at(args_ids[0].get<std::string>()));
        _out.print(") = ");
        codegen_expr(block.at("inputs").at(args_ids[1].get<std::string>()));
        _out.print(";\n");
    }

    else if (block.at("mutation").at("proccode").get<std::string>() == "poke16 %s %s")
    {
        _out.print("*(uint16_t*)((uintptr_t)");

        codegen_expr(block.at("inputs").at(args_ids[0].get<std::string>()));
        _out.print(") = ");
        codegen_expr(block.at("inputs").at(args_ids[1].get<std::string>()));
        _out.print(";\n");
    }

    fmt::print("{}\n", args_ids.dump());
}
