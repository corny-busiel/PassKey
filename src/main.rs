use std::io::Write; // Импортируем модуль для работы с вводом/выводом
use sha2::{self, Digest}; // Импортируем библиотеку sha2 для хеширования

fn main() {
    let mut stdout = std::io::stdout(); 

    // Запрашиваем ввод у пользователя
    write!(stdout, "Введите строку для генерации пароля: ").unwrap();
    stdout.flush().unwrap();

    let password = rpassword::read_password().unwrap(); 
    let hash = create_hash(&password); 
    let ch_vec = create_ch_vector(); 
    let result = create_pass(hash, &ch_vec); 
    writeln!(stdout, "{}", result).unwrap(); 
}

/// Создает вектор символов из ASCII (от 33 до 126)
fn create_ch_vector() -> Vec<char> {
    (33..127).filter_map(std::char::from_u32).collect()
}

/// Создает вектор байтов из хеша
fn create_hash(word: &str) -> Vec<u8> {
    let word_as_bytes = word.as_bytes();
    let hasher = sha2::Sha256::digest(word_as_bytes);
    hasher.to_vec()
}

/// Преобразует хеш в вектор индексов символов
fn create_vec_result(hash: Vec<u8>, ch_vec: &Vec<char>) -> Vec<u8> {
    let mut result_vec = Vec::new();
    for i in hash {
        let a = i % ch_vec.len() as u8;
        result_vec.push(a);
    }
    result_vec
}

/// Генерирует строку пароля на основе хеша и вектора символов
fn create_pass(hash: Vec<u8>, ch_vec: &Vec<char>) -> String {
    let mut string_result = String::new();
    let result_vec = create_vec_result(hash, &ch_vec);
    for i in &result_vec {
        string_result.push(ch_vec[*i as usize]);
    }
    string_result
}
