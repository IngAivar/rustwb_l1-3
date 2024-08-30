use std::thread;
use std::sync::mpsc;

fn main() {
    let n = 12; // Размер массива
    let _numbers: Vec<i32> = (1..=n).collect();

    // Создаем канал для передачи результатов
    let (tx, rx) = mpsc::channel();

    // Создаем потоки для расчета сумм квадратов
    let handles: Vec<_> = _numbers.into_iter()
        .map(|num| {
            let tx = tx.clone();
            thread::spawn(move || {
                let square = num * num + num * num;
                tx.send(square).unwrap();
            })
        })
        .collect();

    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    // Закрываем канал (необязательно, но рекомендуется)
    drop(tx);

    // Считываем результаты из канала и выводим их
    for result in rx {
        println!("{}", result);
    }
    
}