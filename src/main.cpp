#include "scratch2native.hpp"
#include <cstring>
#include <fmt/color.h>
#include <fmt/format.h>
#include <fstream>

int main(int argc, const char **argv)
{
    bool freestanding = false;
    const char *output = "output.cpp", *input = "project.json";

    for (int i = 0; i < argc; i++)
    {
        if (!std::strcmp(argv[i], "--freestanding"))
        {
            freestanding = true;
        }
        else if (!std::strcmp(argv[i], "--output") || !std::strcmp(argv[i], "-o"))
        {
            output = argv[++i];
        }
        else if (argv[i][0] != '-' && i != 0)
        {
            input = argv[i];
        }
    }

    fmt::print("input: {}, output: {}, freestanding: {}\n", input, output, freestanding);

    std::ifstream i;

    i.open(input);

    if (!i)
    {
        fmt::print(fg(fmt::color::crimson) | fmt::emphasis::bold, "error: ");
        fmt::print("error opening file '{}'\n", input);
        return -1;
    }

    auto out = fmt::output_file(output);

    json j;
    i >> j;

    scratch2native::compile_scratch(j, out, freestanding);

    return 0;
}
