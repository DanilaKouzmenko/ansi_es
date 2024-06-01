# ansi es ( ansi escape sequences ) 1.1

[EN] Library for convenient interaction with ANSI ESCAPE SEQUENCES. It allows formatting text and moving the cursor directly in the console. It is an equivalent to the [colorama](https://github.com/tartley/colorama) library in Python.

[UPDATE] Add class FormatedString

#### Now, more details about the content

| Function     | Input | Output  | Description                                  |
|--------------|-------|---------|----------------------------------------------|
| convert_code | u8    | String  | Creates an ANSI ES character from the code   |
| fore         |       | HashMap | Creates a HashMap with all text colors       |
| back         |       | HashMap | Creates a HashMap with all background colors |
| styles       |       | HashMap | Creates a HashMap with all text styles       |
| reset        |       | String  | Resets all changes                           |

#### About the implementation of AnsiCursor

| Method  | Input    | Description                                 |
|---------|----------|---------------------------------------------|
| up      | u16      | Moves the cursor up by n lines              |
| down    | u16      | Moves the cursor down by n lines            |
| forward | u16      | Moves the cursor forward (right) by n chars |
| back    | u16      | Moves the cursor back (left) by n chars     |
| pos     | i16, i16 | Sets the position by x and y coordinates    |

### Example:
```rust
mod ansi_es;

use ansi_es;
use std::collections::HashMap;

fn main() {
    let cursor = ansi_es::AnsiCursor; // cursor initialization
    let fore_colors = ansi_es::fore(); // text color initialization
    let back = ansi_es::back(); // background color initialization
    let styles = ansi_es::styles(); // styles initialization
    println!(" red  bold text.", fore_colors.get("RED"), styles.get("BRIGHT"));
    println!("Text is still red and bold.");
    println!( "And now it's not)))" ansi_es::reset() );
}
```


[RU] Библиотека для удобного взаимодействия с ANSI ESCAPE SEQUENSES.
Позволяет форматировать текст и перемещать курсор прямо в консоли.
Она является аналогом библиотеки [colorama](https://github.com/tartley/colorama) из python.

#### А теперь подробнее про содержание

| Функция      | Ввод | Вывод   | Описание                                      |
|--------------|------|---------|-----------------------------------------------|
| convert_code | u8   | String  | Создает символ ANSI ES по коду                |
| fore         |      | HashMap | Создает HashMap со всеми цветами текста       |
| back         |      | HashMap | Создает HashMap со всеми цветами заднего фона |
| styles       |      | HashMap | Создает HashMap со всеми стилями текста       |
| reset        |      | String  | Убирает все изменения                         |

#### Про имплементацию AnsiCursor

| Метод   | Ввод     | Описание                                     |
|---------|----------|----------------------------------------------|
| up      | u16      | Двигает курсор вверх на n строк              |
| down    | u16      | Двигает курсор вниз на n строк               |
| forward | u16      | двигает курсор вперед (вправо) на n символов |
| back    | u16      | Двигает курсор назад (влево) на n символов   |
| pos     | i16, i16 | Задает позицию по x и y                      |

### Пример:
```rust
mod ansi_es;

use ansi_es;
use std::collections::HashMap;

fn main(){
    let cursor = ansi_es::AnsiCursor{}; // инициализация курсора
    let fore_colors = ansi_es::fore(); // инициализация цветов текста
    let back = ansi_es::back(); // инициализация цветов заднего фона
    let styles = ansi_es::styles(); // инициализация стилей
    println!( "{} красный {} жирный текст.", fore_colors.get("RED"), styles.get("BRIGHT") );
    println!( "Текст все ещё красный и жирный." );
    println!( "{}А теперь нет)))" ansi_es::reset() );
}
```
