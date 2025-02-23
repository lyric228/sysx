use std::sync::{
    Mutex,
    Arc,
};
use std::time::Duration;
use std::thread;

use sysx::deadlock::deadlock_detection_thread;


#[test]
// Тест обнаружения дедлока при взаимной блокировке двух мьютексов.
fn test_deadlock_detection() {
    // Создаем два мьютекса
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    // Клонируем Arc для использования во втором потоке
    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);

    // Запускаем поток обнаружения дедлоков
    thread::spawn(deadlock_detection_thread);

    // Создаем поток, который будет участвовать в дедлоке
    let thread1 = thread::spawn(move || {
        let _lock1 = mutex1.lock().unwrap();
        // Даем время второму потоку захватить mutex2
        thread::sleep(Duration::from_millis(100));
        let _lock2 = mutex2.lock().unwrap();
    });

    // Создаем второй поток, который создаст взаимную блокировку
    let thread2 = thread::spawn(move || {
        let _lock2 = mutex2_clone.lock().unwrap();
        // Даем время первому потоку захватить mutex1
        thread::sleep(Duration::from_millis(100));
        let _lock1 = mutex1_clone.lock().unwrap();
    });

    // Ждем некоторое время, чтобы детектор дедлоков успел сработать
    thread::sleep(Duration::from_secs(12));

    // Проверяем, что потоки завершились с ошибкой из-за дедлока
    assert!(thread1.is_finished());
    assert!(thread2.is_finished());
}

#[test]
// Тест работы без дедлоков.
// Проверяет, что детектор не выдает ложных срабатываний.
fn test_no_deadlock() {
    // Запускаем поток обнаружения дедлоков
    thread::spawn(deadlock_detection_thread);

    let mutex = Arc::new(Mutex::new(0));
    let mutex_clone = Arc::clone(&mutex);

    // Создаем поток, который просто захватывает и освобождает мьютекс
    let handle = thread::spawn(move || {
        let mut lock = mutex.lock().unwrap();
        *lock += 1;
    });

    // Ждем завершения потока
    handle.join().unwrap();

    // Проверяем, что значение было успешно изменено
    assert_eq!(*mutex_clone.lock().unwrap(), 1);
}

#[test]
// Тест множественных потоков без дедлока.
// Проверяет работу детектора при интенсивной работе с мьютексами.
fn test_multiple_threads_no_deadlock() {
    thread::spawn(deadlock_detection_thread);

    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Создаем 10 потоков, которые инкрементируют счетчик
    for _ in 0..10 {
        let mutex_clone = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let mut lock = mutex_clone.lock().unwrap();
            *lock += 1;
        });
        handles.push(handle);
    }

    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    // Проверяем итоговое значение
    assert_eq!(*mutex.lock().unwrap(), 10);
}

#[test]
// Тест последовательного захвата мьютексов.
// Проверяет, что правильный порядок захвата предотвращает дедлоки.
fn test_sequential_mutex_locking() {
    thread::spawn(deadlock_detection_thread);

    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    let mut handles = vec![];

    // Создаем несколько потоков, которые захватывают мьютексы в одном порядке
    for _ in 0..5 {
        let mutex1_clone = Arc::clone(&mutex1);
        let mutex2_clone = Arc::clone(&mutex2);
        
        let handle = thread::spawn(move || {
            let _lock1 = mutex1_clone.lock().unwrap();
            let _lock2 = mutex2_clone.lock().unwrap();
            // Выполняем какую-то работу
            thread::sleep(Duration::from_millis(10));
        });
        
        handles.push(handle);
    }

    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    // Проверяем, что мьютексы все еще можно захватить
    assert!(mutex1.try_lock().is_ok());
    assert!(mutex2.try_lock().is_ok());
}
