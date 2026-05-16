use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io;

// Лабораторная работа №3
// Тема: Консольная игра "Виселица"
// Выполнила: Мехова А.С.
//
// Идея игры простая:
// программа выбирает слово, игрок вводит буквы,
// а программа проверяет, есть ли такая буква в слове.

fn main() {
    println!("====================================");
    println!("        Игра Виселица");
    println!("        МеховаАС_СПиМП_лаб3");
    println!("====================================");
    println!("Угадайте слово по буквам.");
    println!("Можно ошибиться 6 раз.\n");

    // Небольшой список слов для игры.
    // Можно добавить больше слов, если нужно.
    let words = vec![
        "rust",
        "server",
        "module",
        "vector",
        "string",
        "program",
        "memory",
        "thread",
    ];

    // Выбираем случайное слово из списка.
    let secret_word = choose_word(&words);

    // Здесь будут храниться буквы, которые пользователь уже угадал.
    let mut guessed_letters: HashSet<char> = HashSet::new();

    // Здесь будут храниться все введенные буквы,
    // чтобы пользователь видел свои попытки.
    let mut used_letters: Vec<char> = Vec::new();

    let mut mistakes = 0;
    let max_mistakes = 6;

    // Главный игровой цикл.
    // Он работает до победы или поражения.
    loop {
        print_game_state(secret_word, &guessed_letters, &used_letters, mistakes, max_mistakes);

        let letter = read_letter();

        // Проверяем, вводил ли пользователь такую букву раньше.
        if used_letters.contains(&letter) {
            println!("Вы уже вводили букву '{}'. Попробуйте другую.\n", letter);
            continue;
        }

        used_letters.push(letter);

        if secret_word.contains(letter) {
            println!("Верно! Буква '{}' есть в слове.\n", letter);
            guessed_letters.insert(letter);
        } else {
            println!("Ошибка! Буквы '{}' нет в слове.\n", letter);
            mistakes += 1;
        }

        // Проверяем победу.
        if is_word_guessed(secret_word, &guessed_letters) {
            print_game_state(secret_word, &guessed_letters, &used_letters, mistakes, max_mistakes);
            println!("Поздравляю! Вы угадали слово: {}", secret_word);
            break;
        }

        // Проверяем поражение.
        if mistakes >= max_mistakes {
            println!("Вы проиграли. Загаданное слово было: {}", secret_word);
            break;
        }
    }
}

// Функция выбирает случайное слово из списка.
fn choose_word<'a>(words: &'a [&'a str]) -> &'a str {
    let mut rng = rand::thread_rng();
    words.choose(&mut rng).unwrap()
}

// Функция считывает одну букву из консоли.
fn read_letter() -> char {
    loop {
        println!("Введите одну английскую букву:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Ошибка чтения строки");

        let input = input.trim().to_lowercase();

        // Проверяем, что пользователь ввел ровно один символ.
        if input.chars().count() != 1 {
            println!("Нужно ввести только одну букву.\n");
            continue;
        }

        let letter = input.chars().next().unwrap();

        // В этой версии игры используются английские слова,
        // поэтому проверяем английский алфавит.
        if !letter.is_ascii_alphabetic() {
            println!("Можно вводить только английские буквы.\n");
            continue;
        }

        return letter;
    }
}

// Функция выводит текущее состояние игры.
fn print_game_state(
    secret_word: &str,
    guessed_letters: &HashSet<char>,
    used_letters: &Vec<char>,
    mistakes: i32,
    max_mistakes: i32,
) {
    println!("------------------------------------");
    print!("Слово: ");

    for ch in secret_word.chars() {
        if guessed_letters.contains(&ch) {
            print!("{} ", ch);
        } else {
            print!("_ ");
        }
    }

    println!();
    println!("Ошибки: {}/{}", mistakes, max_mistakes);
    println!("Использованные буквы: {:?}", used_letters);
    print_hangman(mistakes);
    println!("------------------------------------\n");
}

// Функция проверяет, угадано ли слово полностью.
fn is_word_guessed(secret_word: &str, guessed_letters: &HashSet<char>) -> bool {
    for ch in secret_word.chars() {
        if !guessed_letters.contains(&ch) {
            return false;
        }
    }

    true
}

// Рисунок виселицы зависит от количества ошибок.
// Для простоты используется match.
fn print_hangman(mistakes: i32) {
    match mistakes {
        0 => {
            println!("  +---+");
            println!("      |");
            println!("      |");
            println!("      |");
            println!("     ===");
        }
        1 => {
            println!("  +---+");
            println!("  O   |");
            println!("      |");
            println!("      |");
            println!("     ===");
        }
        2 => {
            println!("  +---+");
            println!("  O   |");
            println!("  |   |");
            println!("      |");
            println!("     ===");
        }
        3 => {
            println!("  +---+");
            println!("  O   |");
            println!(" /|   |");
            println!("      |");
            println!("     ===");
        }
        4 => {
            println!("  +---+");
            println!("  O   |");
            println!(" /|\\  |");
            println!("      |");
            println!("     ===");
        }
        5 => {
            println!("  +---+");
            println!("  O   |");
            println!(" /|\\  |");
            println!(" /    |");
            println!("     ===");
        }
        _ => {
            println!("  +---+");
            println!("  O   |");
            println!(" /|\\  |");
            println!(" / \\  |");
            println!("     ===");
        }
    }
}
