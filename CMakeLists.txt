cmake_minimum_required(VERSION 3.22)
project(scratchnative)

set(CMAKE_CXX_STANDARD 17)

file(GLOB SOURCES
        src/*.hpp
        src/*.cpp
        )

find_package(nlohmann_json 3.2.0 REQUIRED)
find_package(fmt REQUIRED)
include_directories(${fmt_INCLUDE_DIRS})

add_executable(scratchnative ${SOURCES_FILES} ${SOURCES})


target_link_libraries(scratchnative PRIVATE fmt::fmt nlohmann_json::nlohmann_json)
