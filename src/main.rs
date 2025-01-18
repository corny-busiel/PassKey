use std::io::Write; // Импортируем модуль для работы с вводом/выводом
use std::time::Duration; // Импортируем модуль для работы с временными интервалами
use sha2::{self, Digest}; // Импортируем библиотеку sha2 для хеширования
use clipboard::ClipboardProvider; // Импортируем интерфейс для работы с буфером обмена
use clipboard::ClipboardContext; // Импортируем контекст для работы с буфером обмена
use std::thread::sleep; // Импортируем функцию для приостановки выполнения потока

const CLIPBOARD_SECOND: u64 = 10; // Константа для времени, в течение которого пароль будет храниться в буфере обмена
const COUNTER: i64 = 42; // Константа для счетчика, используемого в хешировании

fn main() {
    let mut stdout = std::io::stdout(); // Получаем стандартный вывод

    // Запрашиваем ввод seed-фразы у пользователя
    write!(stdout, "Введите seed_pharase: ").unwrap();
    stdout.flush().unwrap(); // Очищаем буфер вывода
    let seed_seed_pharase = rpassword::read_password().unwrap(); // Читаем seed-фразу без отображения

    // Запрашиваем ввод пароля у пользователя
    write!(stdout, "Введите password: ").unwrap();
    stdout.flush().unwrap(); // Очищаем буфер вывода
    let password = rpassword::read_password().unwrap(); // Читаем пароль без отображения

    //write!(stdout, "Введите counter: ").unwrap();
    //stdout.flush().unwrap(); // Очищаем буфер вывода
    //let counter: i64 = rpassword::read_password().unwrap().parse().unwrap();

    // Создаем вектор символов для генерации пароля
    let ch_vector = create_ch_vector();

    // Генерируем итоговый пароль
    let result_pass = create_pass(
        &ch_vector,
        create_vec_result(
            create_hash(&seed_seed_pharase), // Хешируем seed-фразу
            create_hash(&password), // Хешируем пароль
            COUNTER, // Используем константу COUNTER
            ch_vector.len() // Длина вектора символов
        )
    );

    // Выводим сгенерированный пароль
    //writeln!(stdout, "{}", result_pass).unwrap();
    
    // Копируем сгенерированный пароль в буфер обмена
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    clipboard.set_contents(result_pass.to_owned()).unwrap();
    writeln!(stdout, "Пароль скопирован").unwrap(); // Уведомляем пользователя о копировании пароля

    // Ждем указанное время перед завершением программы
    sleep(Duration::new(CLIPBOARD_SECOND, 0));
    writeln!(stdout, "Программа завершена").unwrap(); // Уведомляем о завершении программы
}

// Функция для хеширования строки с использованием SHA-256
fn create_hash(word: &str) -> Vec<u8> {
    let word_as_bytes = word.as_bytes(); // Преобразуем строку в байты
    let hasher = sha2::Sha256::digest(word_as_bytes); // Хешируем байты
    hasher.to_vec() // Возвращаем хеш в виде вектора байтов
}

// Функция для создания вектора результатов на основе хешей и счетчика
fn create_vec_result(hash_seed_pharase: Vec<u8>, hash_password: Vec<u8>, counter: i64, len_ch_vec: usize) -> Vec<u8> { 
    let mut result_vec = Vec::new(); // Создаем вектор для хранения результатов
    for i in 0..hash_seed_pharase.len() { // Проходим по всем элементам хеша seed-фразы
        // Вычисляем результат на основе хешей и счетчика
        let result = ((hash_password[i] as i64 * hash_seed_pharase[i] as i64) ^ counter) % len_ch_vec as i64;
        result_vec.push(result as u8); // Добавляем результат в вектор
    }
    result_vec // Возвращаем вектор результатов
}

// Функция для создания пароля на основе вектора символов и вектора результатов
fn create_pass(ch_vec: &[char], result_vec: Vec<u8>) -> String { 
    let mut string_result = String::new(); // Создаем пустую строку для результата
    for i in &result_vec { // Проходим по всем элементам вектора результатов
        string_result.push(ch_vec[*i as usize]); // Добавляем соответствующий символ в строку
    }
    string_result // Возвращаем сгенерированный пароль
}

// Функция для создания вектора символов из ASCII (от 33 до 126)
fn create_ch_vector() -> Vec<char> {
    // Генерируем вектор символов, фильтруя допустимые значения ASCII
    (33..127).filter_map(std::char::from_u32).collect() // Преобразуем диапазон в символы и собираем их в вектор
}