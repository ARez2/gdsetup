
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

void initialize_{}_module(ModuleInitializationLevel p_level);
void uninitialize_{}_module(ModuleInitializationLevel p_level);

#endif // {}_REGISTER_TYPES_H
        ", classname.to_uppercase(), classname.to_uppercase(), classname, classname, classname.to_uppercase()
        )
}


pub fn generate_register_cpp(classname: &str) -> String {
    format!("
#include 'register_types.h'

#include <gdextension_interface.h>

#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/core/defs.hpp>
#include <godot_cpp/godot.hpp>

#include '{}.h'
#include 'tests.h'

using namespace godot;

void initialize_{}_module(ModuleInitializationLevel p_level) {{
    if (p_level != MODULE_INITIALIZATION_LEVEL_SCENE) {{
        return;
    }}

    ClassDB::register_class<{}>();
}}

void uninitialize_{}_module(ModuleInitializationLevel p_level) {{
    if (p_level != MODULE_INITIALIZATION_LEVEL_SCENE) {{
        return;
    }}
}}

extern 'C' {{
// Initialization.
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
#include '{}.h'

#include <godot_cpp/core/class_db.hpp>

#include <godot_cpp/classes/global_constants.hpp>
#include <godot_cpp/variant/utility_functions.hpp>

using namespace godot;
{}::{}() {{
}}
{}::~{}() {{
}}
{}::exampleFunction(int number) {{
    UtilityFunctions::print('Your number was: ', number);
}}

void {}::_bind_methods() {{
    ClassDB::bind_method(D_METHOD('exampleFunction', 'number'), &{}::exampleFunction);
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