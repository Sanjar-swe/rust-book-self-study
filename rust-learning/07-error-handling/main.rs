// 5. Обработка ошибок

// В Rust ошибки обрабатываются с помощью типов Result и Option.

//     Result<T, E>: используется для обработки ошибок, возвращая Ok(T) для успешного результата или Err(E) для ошибки.

//     Option<T>: используется, когда значение может отсутствовать (например, возвращение None).

// Пример с Result:
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
