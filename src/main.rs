fn main() {
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

    print_chapter("CHAPTER 2");
}

fn print_chapter(chapter: &str) {
    println!("\n{}", chapter);
    println!("====================================");
}

fn print_section(section: &str) {
    println!("\n{}", section);
    println!("------------------------------------");
}
