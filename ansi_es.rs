use std::collections::HashMap;

const SCI : &str = "\x33[";
//const OSC : &str = "\x33]";

fn convert_code(code : u8) -> String {
    /// create an ANSI ES character from the code
    return String::from(SCI) + &code.to_string() + "m";
}

fn fore() -> HashMap<&'static str, String> {
    /// Get HashMap with all fore colors
    /// Colors: "BLACK", "RED", "GREEN", YELLOW, "BLUE", "MAGENTA", "CYAN", "WHITE", "RESET"
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
    /// Get HashMap with all background colors
    /// Colors: "BLACK", "RED", "GREEN", YELLOW, "BLUE", "MAGENTA", "CYAN", "WHITE", "RESET"
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

struct FormatedString{
    _string : String,
    _fore : String,
    _back : String,
    _style : String
}

impl FormatedString{
    fn new(){
        FormatedString{
            string : String::new(),
            fore : convert_code(39),
            back : convert_code(49),
            style : convert_code(22)
        }
    }
    fn paint(color :&str){
        /// sets text color
        /// colors: black, red, yellow, green, blue, cyan, magenta and white
        match color {
            "black" => _fore = convert_code(30),
            "red" => _fore = convert_code(31),
            "green" => _fore = convert_code(32),
            "yellow" => _fore = convert_code(33),
            "blue" => _fore = convert_code(34),
            "magenta" => _fore = convert_code(35),
            "cyan" => _fore = convert_code(36),
            "white" => _fore = convert_code(37),
            _ => _fore = convert_code(39)
        }
    }
    fn fill(color : &str){
        /// sets background color
        /// colors: black, red, yellow, green, blue, cyan, magenta and white
        match color {
            "black" => _back = convert_code(40),
            "red" => _back = convert_code(41),
            "green" => _back = convert_code(42),
            "yellow" => _back = convert_code(43),
            "blue" => _back = convert_code(44),
            "magenta" => _back = convert_code(45),
            "cyan" => _back = convert_code(46),
            "white" => _back = convert_code(47),
            _ => _back = convert_code(49)
        }
    }
    fn styling( _type : &str ){
        /// sets style to text
        /// styles: bright, dim
        match _type{
            "bright" => _style = convert_code(1),
            "dim" => _style = convert_code(2),
            _ => _style = convert_code(22)
        }
    }
    fn compile() -> String{
        /// compiling FormatedString to String
        return fore + back + style + _string + reset();
    }
}
