// fn main() {
//     let number = 10;
//     if number < 5 {
//         println!("number is less than 5");
//     } else {
//         println!("number is higher than 5");
//     }
// }

// loop — бесконечный цикл:
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         println!("Counter: {}", counter);
//         counter += 1;

//         if counter == 5 {
//             break counter * 2; // 🔹 break возвращает значение
//         }
//     };

//     println!("Result from loop: {}", result); // 🔹 Печатаем значение, которое вышло из цикла
// }
// fn main() {
//     let numbers = [1, 2, 3, 4, 5];

//     for number in numbers.iter() {
//         println!("Number: {}", number);
//     }
// }

// while — цикл с условием:
fn main() {
    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }
}
