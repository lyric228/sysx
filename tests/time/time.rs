use sysx::time::*;


// SleepTime conversion tests

#[test]
// Тест конвертации миллисекунд в SleepTime.
// Проверяет преобразование из u64 (миллисекунды).
fn test_sleep_time_from_ms() {
    let time: SleepTime = 1500u64.into();
    assert_eq!(time.seconds, 1.5);
}

#[test]
// Тест конвертации Duration в SleepTime.
// Проверяет преобразование из std::time::Duration.
fn test_sleep_time_from_duration() {
    let duration = Duration::from_millis(2500);
    let time: SleepTime = duration.into();
    assert_eq!(time.seconds, 2.5);
}

#[test]
// Тест конвертации секунд (f64) в SleepTime.
// Проверяет прямое преобразование секунд.
fn test_sleep_time_from_seconds() {
    let time: SleepTime = 3.5f64.into();
    assert_eq!(time.seconds, 3.5);
}


// SleepTime string parsing tests

#[test]
// Тест парсинга различных форматов времени из строки.
// Проверяет все поддерживаемые единицы измерения.
fn test_sleep_time_from_str_valid() {
    assert_eq!(SleepTime::from_str("500ms").unwrap().seconds, 0.5);
    assert_eq!(SleepTime::from_str("2s").unwrap().seconds, 2.0);
    assert_eq!(SleepTime::from_str("1.5m").unwrap().seconds, 90.0);
    assert_eq!(SleepTime::from_str("1h").unwrap().seconds, 3600.0);
    assert_eq!(SleepTime::from_str("1000ns").unwrap().seconds, 0.000001);
}

#[test]
// Тест обработки некорректных строковых форматов.
// Проверяет различные случаи неправильного формата.
fn test_sleep_time_from_str_invalid() {
    assert!(matches!(
        SleepTime::from_str("invalid").unwrap_err(),
        SleepError::InvalidFormat(_)
    ));
    assert!(matches!(
        SleepTime::from_str("-1s").unwrap_err(),
        SleepError::NegativeTime
    ));
    assert!(matches!(
        SleepTime::from_str("2x").unwrap_err(),
        SleepError::InvalidFormat(_)
    ));
}


// SleepTime to Duration conversion tests

#[test]
// Тест преобразования SleepTime в Duration.
// Проверяет корректность конвертации времени.
fn test_sleep_time_to_duration() {
    let time = SleepTime { seconds: 1.5 };
    let duration = time.to_duration();
    assert_eq!(duration.as_secs(), 1);
    assert_eq!(duration.subsec_nanos(), 500_000_000);
}

#[test]
#[should_panic(expected = "Время не может быть отрицательным")]
// Тест проверки паники при отрицательном времени.
// Проверяет что to_duration паникует при отрицательных значениях.
fn test_sleep_time_to_duration_negative() {
    let time = SleepTime { seconds: -1.0 };
    time.to_duration();
}


// safe_sleep tests

#[test]
// Тест успешного выполнения safe_sleep.
// Проверяет корректную работу с допустимыми значениями.
fn test_safe_sleep_success() {
    assert!(safe_sleep(0.1).is_ok());
    assert!(safe_sleep("100ms").is_ok());
}

#[test]
// Тест обработки ошибок в safe_sleep.
// Проверяет корректную обработку отрицательного времени.
fn test_safe_sleep_error() {
    let negative_time = SleepTime { seconds: -1.0 };
    assert!(matches!(
        safe_sleep(negative_time),
        Err(SleepError::NegativeTime)
    ));
}


// sleep function tests

#[test]
// Тест функции sleep с различными типами входных данных.
// Проверяет работу с разными форматами времени.
fn test_sleep_various_inputs() {
    // Тестируем различные входные форматы
    sleep(100u64); // миллисекунды
    sleep(0.1f64); // секунды
    sleep("100ms"); // строка
    sleep(Duration::from_millis(100)); // Duration
}

#[test]
// Тест измерения реального времени выполнения sleep.
// Проверяет что фактическое время сна примерно соответствует заданному.
fn test_sleep_duration() {
    use std::time::Instant;
    
    let start = Instant::now();
    sleep(100u64); // 100ms
    let elapsed = start.elapsed();
    
    // Проверяем что прошло как минимум 100ms
    assert!(elapsed >= Duration::from_millis(100));
}
