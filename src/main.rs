fn main() {
    chapter_1();
    chapter_2();
    chapter_3();
    chapter_4();
    chapter_5();
    chapter_6();
    chapter_7();
    chapter_8();
    chapter_9();
}

fn chapter_1() {
    print_chapter("CHAPTER 1");

    print_section("01 - Hello world");
    rust_by_example::chapter_1::_01_hello_world::run();
    print_section("01.a - Hello world");
    rust_by_example::chapter_1::_01a_hello_world::run();

    print_section("02 - Comments");
    rust_by_example::chapter_1::_02_comments::run();

    print_section("03 - Formatted print");
    rust_by_example::chapter_1::_03_formatted_print::run();

    print_section("03.a - Formatted print");
    rust_by_example::chapter_1::_03a_formatted_print::run();

    print_section("04 - Debug");
    rust_by_example::chapter_1::_04_debug::run();

    print_section("05 - Display");
    rust_by_example::chapter_1::_05_display::run();

    print_section("05.a - Complex display");
    rust_by_example::chapter_1::_05a_complex_display::run();

    print_section("06 - List testcase");
    rust_by_example::chapter_1::_06_list_testcase::run();

    print_section("06.a - List testcase");
    rust_by_example::chapter_1::_06a_list_testcase::run();

    print_section("07 - Formatting");
    rust_by_example::chapter_1::_07_formatting::run();

    print_section("07.a - Formatting");
    rust_by_example::chapter_1::_07a_formatting::run();
}

fn chapter_2() {
    print_chapter("CHAPTER 2");

    print_section("01 - Primitives");
    rust_by_example::chapter_2::_01_primitives::run();

    print_section("02 - Literals and operators");
    rust_by_example::chapter_2::_02_literals_and_operators::run();

    print_section("03 - Tuples");
    rust_by_example::chapter_2::_03_tuples::run();

    print_section("03.a - Tuples");
    rust_by_example::chapter_2::_03a_tuples::run();

    print_section("04 - Arrays and slices");
    rust_by_example::chapter_2::_04_arrays_and_slices::run();
}

fn chapter_3() {
    print_chapter("CHAPTER 3");

    print_section("01 - Structures");
    rust_by_example::chapter_3::_01_structures::run();

    print_section("01.a - Structures");
    rust_by_example::chapter_3::_01a_structures::run();

    print_section("02 - Enums");
    rust_by_example::chapter_3::_02_enums::run();

    print_section("03 - Type aliases");
    rust_by_example::chapter_3::_03_type_aliases::run();

    print_section("04 - Use");
    rust_by_example::chapter_3::_04_use::run();

    print_section("05 - C like");
    rust_by_example::chapter_3::_05_c_like::run();

    print_section("06 - Linked list");
    rust_by_example::chapter_3::_06_linked_list::run();

    print_section("07 - Constants");
    rust_by_example::chapter_3::_07_constants::run();
}

fn chapter_4() {
    print_chapter("CHAPTER 4");

    print_section("01 - Variable bindings");
    rust_by_example::chapter_4::_01_variable_bindings::run();

    print_section("02 - Mutability");
    rust_by_example::chapter_4::_02_mutability::run();

    print_section("03 - Scope and shadowing");
    rust_by_example::chapter_4::_03_scope_and_shadowing::run();

    print_section("04 - Declare first");
    rust_by_example::chapter_4::_04_declare_first::run();

    print_section("05 - Freezing");
    rust_by_example::chapter_4::_05_freezing::run();
}

fn chapter_5() {
    print_chapter("CHAPTER 5");

    print_section("01 - Casting");
    rust_by_example::chapter_5::_01_casting::run();

    print_section("02 - Literals");
    rust_by_example::chapter_5::_02_literals::run();

    print_section("03 - Inference");
    rust_by_example::chapter_5::_03_inference::run();

    print_section("04 - Aliasing");
    rust_by_example::chapter_5::_04_aliasing::run();
}

fn chapter_6() {
    print_chapter("CHAPTER 6");

    print_section("01 - From and Into");
    rust_by_example::chapter_6::_01_from_and_into::run();

    print_section("02 - Try From and Try Into");
    rust_by_example::chapter_6::_02_tryfrom_and_tryinto::run();

    print_section("03 - To and From Strings");
    rust_by_example::chapter_6::_03_to_and_from_strings::run();
}

fn chapter_7() {
    print_chapter("CHAPTER 7");

    print_section("01 - Expressions");
    rust_by_example::chapter_7::_01_expressions::run();
}

fn chapter_8() {
    print_chapter("CHAPTER 8");

    print_section("01 - If else");
    rust_by_example::chapter_8::_01_if_else::run();

    print_section("02 - Loop");
    rust_by_example::chapter_8::_02_loop::run();

    print_section("03 - Nesting and labels");
    rust_by_example::chapter_8::_03_nesting_and_labels::run();

    print_section("04 - Returning from loops");
    rust_by_example::chapter_8::_04_returning_from_loops::run();

    print_section("05 - While");
    rust_by_example::chapter_8::_05_while::run();

    print_section("06 - For and range");
    rust_by_example::chapter_8::_06_for_and_range::run();

    print_section("07 - Match");
    rust_by_example::chapter_8::_07_match::run();

    print_section("08 - Destructuring");
    rust_by_example::chapter_8::_08_destructuring::run();

    print_section("09 - Guards");
    rust_by_example::chapter_8::_09_guards::run();

    print_section("10 - Binding");
    rust_by_example::chapter_8::_10_binding::run();

    print_section("11 - If let");
    rust_by_example::chapter_8::_11_if_let::run();

    print_section("11.a - If let");
    rust_by_example::chapter_8::_11a_if_let::run();

    print_section("12 - While let");
    rust_by_example::chapter_8::_12_while_let::run();
}

fn chapter_9() {
    print_chapter("CHAPTER 9");

    print_section("01 - Functions");
    rust_by_example::chapter_9::_01_functions::run();

    print_section("02 - Methods");
    rust_by_example::chapter_9::_02_methods::run();

    print_section("03 - Closures");
    rust_by_example::chapter_9::_03_closures::run();

    print_section("04 - Capturing");
    rust_by_example::chapter_9::_04_capturing::run();

    print_section("05 - As input parameters");
    rust_by_example::chapter_9::_05_as_input_parameters::run();

    print_section("06 - Type anonimity");
    rust_by_example::chapter_9::_06_type_anonimity::run();

    print_section("07 - Input functions");
    rust_by_example::chapter_9::_07_input_functions::run();

    print_section("08 - As output parameters");
    rust_by_example::chapter_9::_08_as_output_parameters::run();

    print_section("09 - Iterator any");
    rust_by_example::chapter_9::_09_iterator_any::run();

    print_section("10 - Searching through iterators");
    rust_by_example::chapter_9::_10_searching_through_iterators::run();

    print_section("11 - Higher order functions");
    rust_by_example::chapter_9::_11_higher_order_functions::run();

    print_section("12 - Diverging functions");
    rust_by_example::chapter_9::_12_diverging_functions::run();

}

fn print_chapter(chapter: &str) {
    println!("\n{}", chapter);
    println!("====================================");
}

fn print_section(section: &str) {
    println!("\n{}", section);
    println!("------------------------------------");
}
