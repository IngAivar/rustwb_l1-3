use std::thread;
use std::sync::mpsc;
use std::sync::Arc;

fn main() {
    let n = 100; // Размер массива
    let numbers: Vec<i32> = (1..=n).collect();

    // Создаем канал для передачи результатов
    let (tx, rx) = mpsc::channel();

    // Преобразуем вектор в Arc для многократного использования в потоках
    let numbers = Arc::new(numbers);

    // Создаем потоки для расчета квадратов
    for num in numbers {
        let tx = tx.clone();
        let num = *num;
        thread::spawn(move || {
            let square = num * num + num * num;
            tx.send(square).unwrap();
        });
    }

    // Считываем результаты из канала и выводим их
    for result in rx {
        println!("{}", result);
    }
}