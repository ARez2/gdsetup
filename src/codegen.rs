
pub fn get_classname_uppercase(classname: &str) -> String {
    let cln = classname.split_at(1);
    let mut class_name_first_upper = cln.0.to_uppercase();
    class_name_first_upper.push_str(cln.1);
    class_name_first_upper
}


pub fn generate_register_h(classname: &str) -> String {
    format!("
#ifndef {}_REGISTER_TYPES_H
#define {}_REGISTER_TYPES_H

#include <godot_cpp/core/class_db.hpp>

using namespace godot;

// Note: It is not recommended to rename that function, except you know what you are doing
void initialize_{}_module(ModuleInitializationLevel p_level);
// Note: It is not recommended to rename that function, except you know what you are doing
void uninitialize_{}_module(ModuleInitializationLevel p_level);

#endif // {}_REGISTER_TYPES_H", classname.to_uppercase(), classname.to_uppercase(), classname, classname, classname.to_uppercase())
}


pub fn generate_register_cpp(classname: &str) -> String {
    format!("
#include \"register_types.h\"

#include <gdextension_interface.h>

#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/core/defs.hpp>
#include <godot_cpp/godot.hpp>

#include \"{}.h\"
//#include \"tests.h\"

using namespace godot;

// Note: It is not recommended to rename that function, except you know what you are doing
void initialize_{}_module(ModuleInitializationLevel p_level) {{
    if (p_level != MODULE_INITIALIZATION_LEVEL_SCENE) {{
        return;
    }}

    ClassDB::register_class<{}>();
}}

// Note: It is not recommended to rename that function, except you know what you are doing
void uninitialize_{}_module(ModuleInitializationLevel p_level) {{
    if (p_level != MODULE_INITIALIZATION_LEVEL_SCENE) {{
        return;
    }}
}}

extern \"C\" {{
// Initialization.
// Note: It is not recommended to rename that function, except you know what you are doing
GDExtensionBool GDE_EXPORT {}_library_init(const GDExtensionInterface *p_interface, GDExtensionClassLibraryPtr p_library, GDExtensionInitialization *r_initialization) {{
    godot::GDExtensionBinding::InitObject init_obj(p_interface, p_library, r_initialization);

    init_obj.register_initializer(initialize_{}_module);
    init_obj.register_terminator(uninitialize_{}_module);
    init_obj.set_minimum_library_initialization_level(MODULE_INITIALIZATION_LEVEL_SCENE);

    return init_obj.init();
}}
}}

    ", classname, classname, get_classname_uppercase(classname), classname, classname, classname, classname)
}


pub fn generate_class_cpp(classname: &str) -> String {
    let class_name_first_upper = get_classname_uppercase(classname);
    format!("
#include \"{}.h\"

#include <godot_cpp/core/class_db.hpp>

#include <godot_cpp/classes/global_constants.hpp>
#include <godot_cpp/variant/utility_functions.hpp>

using namespace godot;
{}::{}() {{
}}
{}::~{}() {{
}}
void {}::exampleFunction(int number) {{
    UtilityFunctions::print(\"Your number was: \", number);
}}

void {}::_bind_methods() {{
    ClassDB::bind_method(D_METHOD(\"exampleFunction\", \"number\"), &{}::exampleFunction);
}}"
, classname, class_name_first_upper, class_name_first_upper, class_name_first_upper, class_name_first_upper, class_name_first_upper, class_name_first_upper, class_name_first_upper)
}


pub fn generate_class_h(classname: &str) -> String {
    let class_name_first_upper = get_classname_uppercase(classname);
    format!(
"
#ifndef {}_CLASS_H
#define {}_CLASS_H

// We don't need windows.h in this example plugin but many others do, and it can
// lead to annoying situations due to the ton of macros it defines.
// So we include it and make sure CI warns us if we use something that conflicts
// with a Windows define.
#ifdef WIN32
#include <windows.h>
#endif

#include <godot_cpp/classes/global_constants.hpp>
#include <godot_cpp/variant/utility_functions.hpp>
#include <godot_cpp/classes/ref_counted.hpp>

#include <godot_cpp/core/binder_common.hpp>

using namespace godot;

class {} : public RefCounted {{
    GDCLASS({}, RefCounted);

private:

protected:
    static void _bind_methods();

public:
    {}();
    ~{}();

    void exampleFunction(int number);
}};
#endif // {}_CLASS_H
", classname.to_uppercase(), classname.to_uppercase(), class_name_first_upper, class_name_first_upper, class_name_first_upper, class_name_first_upper, classname.to_uppercase())
}


pub fn generate_sconstruct(classname: &str) -> String {
    format!("
# See: https://github.com/godotengine/godot-cpp/blob/master/test/SConstruct

#!/usr/bin/env python
import os
import sys

env = SConscript(\"godot-cpp/SConstruct\")

# For the reference:
# - CCFLAGS are compilation flags shared between C and C++
# - CFLAGS are for C-specific compilation flags
# - CXXFLAGS are for C++-specific compilation flags
# - CPPFLAGS are for pre-processor flags
# - CPPDEFINES are for pre-processor defines
# - LINKFLAGS are for linking flags

# tweak this if you want to use different folders, or more folders, to store your source code in.
env.Append(CPPPATH=[\"src/\"])
sources = Glob(\"src/*.cpp\")

if env[\"platform\"] == \"macos\":
    library = env.SharedLibrary(
        \"godot/bin/libgd{}.{{}}.{{}}.framework/libgd{}.{{}}.{{}}\".format(
            env[\"platform\"], env[\"target\"], env[\"platform\"], env[\"target\"]
        ),
        source=sources,
    )
else:
    library = env.SharedLibrary(
        \"godot/bin/libgd{}{{}}{{}}\".format(env[\"suffix\"], env[\"SHLIBSUFFIX\"]),
        source=sources,
    )

Default(library)
    ", classname, classname, classname)
}


pub fn generate_cmakelists(classname: &str) -> String {
    format!("
# See: https://github.com/godotengine/godot-cpp/blob/master/test/CMakeLists.txt

project({})
cmake_minimum_required(VERSION 3.6)

set(GODOT_GDEXTENSION_DIR ../gdextension/ CACHE STRING \"Path to GDExtension interface header directory\")
set(CPP_BINDINGS_PATH ../ CACHE STRING \"Path to C++ bindings\")

if(CMAKE_SYSTEM_NAME STREQUAL \"Linux\")
    set(TARGET_PATH x11)
elseif(CMAKE_SYSTEM_NAME STREQUAL \"Windows\")
    set(TARGET_PATH win64)
elseif(CMAKE_SYSTEM_NAME STREQUAL \"Darwin\")
    set(TARGET_PATH macos)
else()
    message(FATAL_ERROR \"Not implemented support for ${{CMAKE_SYSTEM_NAME}}\")
endif()

# Change the output directory to the bin directory
set(BUILD_PATH ${{CMAKE_SOURCE_DIR}}/bin/${{TARGET_PATH}})
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY \"${{BUILD_PATH}}\")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY \"${{BUILD_PATH}}\")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY \"${{BUILD_PATH}}\")
SET(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG \"${{BUILD_PATH}}\")
SET(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE \"${{BUILD_PATH}}\")
SET(CMAKE_LIBRARY_OUTPUT_DIRECTORY_DEBUG \"${{BUILD_PATH}}\")
SET(CMAKE_LIBRARY_OUTPUT_DIRECTORY_RELEASE \"${{BUILD_PATH}}\")
SET(CMAKE_ARCHIVE_OUTPUT_DIRECTORY_DEBUG \"${{BUILD_PATH}}\")
SET(CMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELEASE \"${{BUILD_PATH}}\")

# Set the c++ standard to c++17
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(GODOT_COMPILE_FLAGS )
set(GODOT_LINKER_FLAGS )

if (\"${{CMAKE_CXX_COMPILER_ID}}\" STREQUAL \"MSVC\")
    # using Visual Studio C++
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} /EHsc /WX\") # /GF /MP
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} /DTYPED_METHOD_BIND\")

    if(CMAKE_BUILD_TYPE MATCHES Debug)
        set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} /MDd\") # /Od /RTC1 /Zi
    else()
        set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} /MD /O2\") # /Oy /GL /Gy
        STRING(REGEX REPLACE \"/RTC(su|[1su])\" \"\" CMAKE_CXX_FLAGS \"${{CMAKE_CXX_FLAGS}}\")
        string(REPLACE \"/RTC1\" \"\" CMAKE_CXX_FLAGS_DEBUG ${{CMAKE_CXX_FLAGS_DEBUG}})
    endif(CMAKE_BUILD_TYPE MATCHES Debug)

    # Disable conversion warning, truncation, unreferenced var, signed mismatch
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} /wd4244 /wd4305 /wd4101 /wd4018 /wd4267\")

    add_definitions(-DNOMINMAX)

    # Unkomment for warning level 4
    #if(CMAKE_CXX_FLAGS MATCHES \"/W[0-4]\")
    #	string(REGEX REPLACE \"/W[0-4]\" \"\" CMAKE_CXX_FLAGS \"${{CMAKE_CXX_FLAGS}}\")
    #endif()

else()

#elseif (\"${{CMAKE_CXX_COMPILER_ID}}\" STREQUAL \"Clang\")
    # using Clang
#elseif (\"${{CMAKE_CXX_COMPILER_ID}}\" STREQUAL \"GNU\")
    # using GCC and maybe MinGW?

    set(GODOT_LINKER_FLAGS \"-static-libgcc -static-libstdc++ -Wl,-R,\"$$ORIGIN\"\")

    # Hmm.. maybe to strikt?
    set(GODOT_COMPILE_FLAGS \"-fPIC -g -Wwrite-strings\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wchar-subscripts -Wcomment -Wdisabled-optimization\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wformat -Wformat=2 -Wformat-security -Wformat-y2k\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wimport -Winit-self -Winline -Winvalid-pch -Werror\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wmissing-braces -Wmissing-format-attribute\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wmissing-include-dirs -Wmissing-noreturn -Wpacked -Wpointer-arith\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wredundant-decls -Wreturn-type -Wsequence-point\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wswitch -Wswitch-enum -Wtrigraphs\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wuninitialized -Wunknown-pragmas -Wunreachable-code -Wunused-label\")
    set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wunused-value -Wvariadic-macros -Wvolatile-register-var -Wno-error=attributes\")

    # -Wshadow -Wextra -Wall -Weffc++ -Wfloat-equal -Wstack-protector -Wunused-parameter -Wsign-compare -Wunused-variable -Wcast-align
    # -Wunused-function -Wstrict-aliasing -Wstrict-aliasing=2 -Wmissing-field-initializers

    if(NOT CMAKE_SYSTEM_NAME STREQUAL \"Android\")
        set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -Wno-ignored-attributes\")
    endif()

    if(CMAKE_BUILD_TYPE MATCHES Debug)
        set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -fno-omit-frame-pointer -O0\")
    else()
        set(GODOT_COMPILE_FLAGS \"${{GODOT_COMPILE_FLAGS}} -O3\")
    endif(CMAKE_BUILD_TYPE MATCHES Debug)
endif()

# Get Sources
file(GLOB_RECURSE SOURCES src/*.c**)
file(GLOB_RECURSE HEADERS include/*.h**)

# Define our godot-cpp library
add_library(${{PROJECT_NAME}} SHARED ${{SOURCES}} ${{HEADERS}})

target_include_directories(${{PROJECT_NAME}} SYSTEM
    PRIVATE
        ${{CPP_BINDINGS_PATH}}/include
        ${{CPP_BINDINGS_PATH}}/gen/include
        ${{GODOT_GDEXTENSION_DIR}}
)

# Create the correct name (godot.os.build_type.system_bits)
# Synchronized with godot-cpp's CMakeLists.txt

set(BITS 32)
if(CMAKE_SIZEOF_VOID_P EQUAL 8)
    set(BITS 64)
endif(CMAKE_SIZEOF_VOID_P EQUAL 8)

if(CMAKE_BUILD_TYPE MATCHES Debug)
    set(GODOT_CPP_BUILD_TYPE Debug)
else()
    set(GODOT_CPP_BUILD_TYPE Release)
endif()

string(TOLOWER ${{CMAKE_SYSTEM_NAME}} SYSTEM_NAME)
string(TOLOWER ${{GODOT_CPP_BUILD_TYPE}} BUILD_TYPE)

if(ANDROID)
    # Added the android abi after system name
    set(SYSTEM_NAME ${{SYSTEM_NAME}}.${{ANDROID_ABI}})
endif()

if(CMAKE_VERSION VERSION_GREATER \"3.13\")
    target_link_directories(${{PROJECT_NAME}}
        PRIVATE
        ${{CPP_BINDINGS_PATH}}/bin/
    )

    target_link_libraries(${{PROJECT_NAME}}
        godot-cpp.${{SYSTEM_NAME}}.${{BUILD_TYPE}}$<$<NOT:$<PLATFORM_ID:Android>>:.${{BITS}}>
    )
else()
    target_link_libraries(${{PROJECT_NAME}}
            ${{CPP_BINDINGS_PATH}}/bin/libgodot-cpp.${{SYSTEM_NAME}}.${{BUILD_TYPE}}$<$<NOT:$<PLATFORM_ID:Android>>:.${{BITS}}>.a
    )
endif()

# Add the compile flags
set_property(TARGET ${{PROJECT_NAME}} APPEND_STRING PROPERTY COMPILE_FLAGS ${{GODOT_COMPILE_FLAGS}})
set_property(TARGET ${{PROJECT_NAME}} APPEND_STRING PROPERTY LINK_FLAGS ${{GODOT_LINKER_FLAGS}})

set_property(TARGET ${{PROJECT_NAME}} PROPERTY OUTPUT_NAME \"{}\")
    ", classname, classname)
}


pub fn generate_gdextension(classname: &str) -> String {
    format!("
[configuration]
entry_symbol = \"{}_library_init\"

[libraries]

macos.debug = \"res://bin/libgd{}.macos.template_debug.framework\"
macos.release = \"res://bin/libgd{}.macos.template_release.framework\"
windows.debug.x86_32 = \"res://bin/libgd{}.windows.template_debug.x86_32.dll\"
windows.release.x86_32 = \"res://bin/libgd{}.windows.template_release.x86_32.dll\"
windows.debug.x86_64 = \"res://bin/libgd{}.windows.template_debug.x86_64.dll\"
windows.release.x86_64 = \"res://bin/libgd{}.windows.template_release.x86_64.dll\"
linux.debug.x86_64 = \"res://bin/libgd{}.linux.template_debug.x86_64.so\"
linux.release.x86_64 = \"res://bin/libgd{}.linux.template_release.x86_64.so\"
linux.debug.arm64 = \"res://bin/libgd{}.linux.template_debug.arm64.so\"
linux.release.arm64 = \"res://bin/libgd{}.linux.template_release.arm64.so\"
linux.debug.rv64 = \"res://bin/libgd{}.linux.template_debug.rv64.so\"
linux.release.rv64 = \"res://bin/libgd{}.linux.template_release.rv64.so\"
android.debug.x86_64 = \"res://bin/libgd{}.android.template_debug.x86_64.so\"
android.release.x86_64 = \"res://bin/libgd{}.android.template_release.x86_64.so\"
android.debug.arm64 = \"res://bin/libgd{}.android.template_debug.arm64.so\"
android.release.arm64 = \"res://bin/libgd{}.android.template_release.arm64.so\"
    ", classname, classname, classname, classname, classname, classname, classname, classname, classname, classname, classname, classname, classname, classname, classname, classname, classname)
}


pub fn generate_gdextension_list(classname: &str) -> String {
    format!("
res://{}.gdextension
    ", classname)
}