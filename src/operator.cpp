#include "scratch2native.hpp"
#include <utility>

using namespace scratch2native;

void Compiler::operator_add(const json &block)
{
    if (_lvalue)
    {
        _lvalue->type = NUMBER;
    }

    _out.print("(");
    codegen_expr(block.at("inputs").at("NUM1"));
    _out.print(" + ");
    codegen_expr(block.at("inputs").at("NUM2"));
    _out.print(")");
}

void Compiler::operator_substract(const json &block)
{
    if (_lvalue)
    {
        _lvalue->type = NUMBER;
    }

    _out.print("(");
    codegen_expr(block.at("inputs").at("NUM1"));
    _out.print(" - ");
    codegen_expr(block.at("inputs").at("NUM2"));
    _out.print(")");
}

void Compiler::operator_multiply(const json &block)
{
    if (_lvalue)
    {
        _lvalue->type = NUMBER;
    }

    _out.print("(");
    codegen_expr(block.at("inputs").at("NUM1"));
    _out.print(" * ");
    codegen_expr(block.at("inputs").at("NUM2"));
    _out.print(")");
}

void Compiler::operator_divide(const json &block)
{
    if (_lvalue)
    {
        _lvalue->type = NUMBER;
    }

    _out.print("(");
    codegen_expr(block.at("inputs").at("NUM1"));
    _out.print(" / ");
    codegen_expr(block.at("inputs").at("NUM2"));
    _out.print(")");
}

void Compiler::operator_lt(const json &block)
{
    codegen_expr(block.at("inputs").at("OPERAND1"));
    _out.print(" < ");
    codegen_expr(block.at("inputs").at("OPERAND2"));
}

void Compiler::operator_equals(const json &block)
{
    codegen_expr(block.at("inputs").at("OPERAND1"));
    _out.print(" == ");
    codegen_expr(block.at("inputs").at("OPERAND2"));
}

void Compiler::operator_gt(const json &block)
{
    codegen_expr(block.at("inputs").at("OPERAND1"));
    _out.print(" > ");
    codegen_expr(block.at("inputs").at("OPERAND2"));
}

void Compiler::operator_and(const json &block)
{
    codegen_expr(block.at("inputs").at("OPERAND1"));

    _out.print(" && ");

    codegen_expr(block.at("inputs").at("OPERAND2"));
}

void Compiler::operator_or(const json &block)
{
    codegen_expr(block.at("inputs").at("OPERAND1"));

    _out.print(" || ");

    codegen_expr(block.at("inputs").at("OPERAND2"));
}

void Compiler::operator_not(const json &block)
{
    codegen_expr(block.at("inputs").at("OPERAND1")[1]);
    _out.print(" != ");
    codegen_expr(block.at("inputs").at("OPERAND2")[1]);
}

void Compiler::operator_random(const json &block)
{
    assert(!"TODO");
}

void Compiler::operator_join(const json &block)
{
    assert(!"TODO");
}

void Compiler::operator_letter_of(const json &block)
{
    assert(!"TODO");
}

void Compiler::operator_length(const json &block)
{
    assert(!"TODO");
}

void Compiler::operator_contains(const json &block)
{
    assert(!"TODO");
}

void Compiler::operator_mod(const json &block)
{
    assert(!"TODO");
}

void Compiler::operator_round(const json &block)
{
    assert(!"TODO");
}
