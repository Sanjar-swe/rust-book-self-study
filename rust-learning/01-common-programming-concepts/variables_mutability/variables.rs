// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     //x = 6; // Ошибка компиляции: нельзя изменить неизменяемую переменную
// }

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; //now it's allowed to change
    println!("The value of x is: {}", x);
}
