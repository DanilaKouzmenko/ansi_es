use std::collections::HashMap;
use std::str;

const SCI : &str = "\x33[";
//const OSC : &str = "\x33]";

fn convert_code(code : u8) -> String {
    return String::from(SCI) + &code.to_string() + "m";
}

fn clear_screen() -> String { // навеное, то что делает cls
    return String::from(SCI) + "2" + "J";
}

fn clear_line() -> String {
    return String::from(SCI) + "2" + "K";
}

fn fore() -> HashMap<&'static str, String> {
    let mut colors = HashMap::new();
    colors.insert("BLACK", convert_code(30));
    colors.insert("RED", convert_code(31));
    colors.insert("GREEN", convert_code(32));
    colors.insert("YELLOW", convert_code(33));
    colors.insert("BLUE", convert_code(34));
    colors.insert("MAGENTA", convert_code(35));
    colors.insert("CYAN", convert_code(36));
    colors.insert("WHITE", convert_code(37));
    colors.insert("RESET", convert_code(39));
    return colors;
}

fn back() -> HashMap<&'static str, String> {
    let mut colors = HashMap::new();
    colors.insert("BLACK", convert_code(40));
    colors.insert("RED", convert_code(41));
    colors.insert("GREEN", convert_code(42));
    colors.insert("YELLOW", convert_code(43));
    colors.insert("BLUE", convert_code(44));
    colors.insert("MAGENTA", convert_code(45));
    colors.insert("CYAN", convert_code(46));
    colors.insert("WHITE", convert_code(47));
    colors.insert("RESET", convert_code(49));
    return colors;
}

fn style() -> HashMap<&'static str, String> {
    let mut styles = HashMap::new();
    styles.insert("BRIGHT", convert_code(1)); // жирный
    styles.insert("DIM", convert_code(2));
    styles.insert("NORMAL", convert_code(22));
    return styles;
}

fn reset() -> String{
    return convert_code(0);
}

struct AnsiCursor{} // чтобы создать сместитель курсора

impl AnsiCursor { // для перемещения курсора
    fn up( n : u16 ) -> String{
        return String::from(SCI) + &n.to_string() + "A";
    }
    fn down( n : u16 ) -> String{
        return String::from(SCI) + &n.to_string() + "B";
    }
    fn forward( n : u16 ) -> String{
        return String::from(SCI) + &n.to_string() + "C";
    }
    fn back( n : u16 ) -> String{
        return String::from(SCI) + &n.to_string() + "D";
    }
    fn pos( x : i16, y : i16 ) -> String {
        return String::from(SCI) + &y.to_string() + ";" + &x.to_string() + "H";
    }
}

//