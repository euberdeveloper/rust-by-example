fn main() {
    chapter_1();
    chapter_2();
    chapter_3();
}

fn chapter_1() {
    print_chapter("CHAPTER 1");

    print_section("01 - Hello world");
    prova::chapter_1::_01_hello_world::run();
    print_section("01.a - Hello world");
    prova::chapter_1::_01a_hello_world::run();

    print_section("02 - Comments");
    prova::chapter_1::_02_comments::run();

    print_section("03 - Formatted print");
    prova::chapter_1::_03_formatted_print::run();

    print_section("03.a - Formatted print");
    prova::chapter_1::_03a_formatted_print::run();

    print_section("04 - Debug");
    prova::chapter_1::_04_debug::run();

    print_section("05 - Display");
    prova::chapter_1::_05_display::run();

    print_section("05.a - Complex display");
    prova::chapter_1::_05a_complex_display::run();

    print_section("06 - List testcase");
    prova::chapter_1::_06_list_testcase::run();

    print_section("06.a - List testcase");
    prova::chapter_1::_06a_list_testcase::run();

    print_section("07 - Formatting");
    prova::chapter_1::_07_formatting::run();

    print_section("07.a - Formatting");
    prova::chapter_1::_07a_formatting::run();
}

fn chapter_2() {
    print_chapter("CHAPTER 2");

    print_section("01 - Primitives");
    prova::chapter_2::_01_primitives::run();

    print_section("02 - Literals and operators");
    prova::chapter_2::_02_literals_and_operators::run();

    print_section("03 - Tuples");
    prova::chapter_2::_03_tuples::run();

    print_section("03.a - Tuples");
    prova::chapter_2::_03a_tuples::run();

    print_section("04 - Arrays and slices");
    prova::chapter_2::_04_arrays_and_slices::run();
}

fn chapter_3() {
    print_chapter("CHAPTER 3");

    print_section("01 - Structures");
    prova::chapter_3::_01_structures::run();

    print_section("01.a - Structures");
    prova::chapter_3::_01a_structures::run();

    print_section("02 - Enums");
    prova::chapter_3::_02_enums::run();

    print_section("03 - Type aliases");
    prova::chapter_3::_03_type_aliases::run();

    print_section("04 - Use");
    prova::chapter_3::_04_use::run();

    print_section("05 - C like");
    prova::chapter_3::_05_c_like::run();

    print_section("06 - Linked list");
    prova::chapter_3::_06_linked_list::run();
}

fn print_chapter(chapter: &str) {
    println!("\n{}", chapter);
    println!("====================================");
}

fn print_section(section: &str) {
    println!("\n{}", section);
    println!("------------------------------------");
}
