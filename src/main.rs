use std::thread;
use std::sync::mpsc;

fn main() {
    let n = 12; // Размер массива
    let _numbers: Vec<i32> = (1..=n).collect();

    // Создаем канал для передачи результатов
    let (tx, rx) = mpsc::channel();

    // Создаем потоки для расчета сумм квадратов
    for num in _numbers {
        let tx = tx.clone();

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