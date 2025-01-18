use std::io::Write;
use std::time::Duration; // Импортируем модуль для работы с вводом/выводом
use sha2::{self, Digest}; // Импортируем библиотеку sha2 для хеширования
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::thread::sleep;

const CLIPBOARD_SECOND : u64 = 10;

//const COUNTER : i64 = 42;

fn main() {
    let mut stdout = std::io::stdout(); // Получаем стандартный вывод

    // Запрашиваем ввод у пользователя
    write!(stdout, "Введите seed_pharase: ").unwrap();
    stdout.flush().unwrap(); // Очищаем буфер вывода
    let seed_seed_pharase = rpassword::read_password().unwrap();

    //writeln!(stdout, "").unwrap();

    write!(stdout, "Введите password: ").unwrap();
    stdout.flush().unwrap(); // Очищаем буфер вывода
    let password = rpassword::read_password().unwrap();

    write!(stdout, "Введите counter: ").unwrap();
    stdout.flush().unwrap(); // Очищаем буфер вывода
    let counter: i64 = rpassword::read_password().unwrap().parse().unwrap();
    
    let ch_vector = create_ch_vector();

    let result_pass = 
        create_pass(&ch_vector,
        create_vec_result(
            create_hash(&seed_seed_pharase),
            create_hash(&password), 
            counter, 
            ch_vector.len())
    );
    writeln!(stdout, "{}", result_pass).unwrap();
    
    let mut clipboard : ClipboardContext = ClipboardProvider::new().unwrap();
    clipboard.set_contents(result_pass.to_owned()).unwrap();
    writeln!(stdout, "Пароль скопирован").unwrap();
    sleep(Duration::new(CLIPBOARD_SECOND, 0));
    writeln!(stdout, "Программа завершена").unwrap();
}

fn create_hash(word: &str) -> Vec<u8> {
        let word_as_bytes = word.as_bytes();
        let hasher = sha2::Sha256::digest(word_as_bytes);
        hasher.to_vec()
    }

fn create_vec_result(hash_seed_pharase: Vec<u8>, hash_password : Vec<u8>, counter : i64, len_ch_vec : usize) -> Vec<u8> { 
        let mut result_vec = Vec::new();
        for i in 0..hash_seed_pharase.len() {
            let result = ((hash_password[i] as i64 * hash_seed_pharase[i] as i64) ^ counter) % len_ch_vec as i64;
            result_vec.push(result as u8);

        }
        result_vec
    }

fn create_pass(ch_vec: &[char], result_vec : Vec<u8>) -> String { 
        let mut string_result = String::new();
        for i in &result_vec {
            string_result.push(ch_vec[*i as usize]);
        }
        string_result
    }


/// Создает вектор символов из ASCII (от 33 до 126)
fn create_ch_vector() -> Vec<char> {
    (33..127).filter_map(std::char::from_u32).collect()
}

