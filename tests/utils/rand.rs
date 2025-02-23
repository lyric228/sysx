use sysx::error::SysxError;
use sysx::rand::*;


// random function tests

#[test]
// Тест генерации случайных целых чисел в диапазоне.
// Проверяет, что числа попадают в заданный диапазон.
fn test_random_int() {
    for _ in 0..100 {
        let num = random(1, 10).unwrap();
        assert!(num >= 1 && num <= 10);
    }
}

#[test]
// Тест генерации случайных чисел с плавающей точкой.
// Проверяет, что числа попадают в заданный диапазон.
fn test_random_float() {
    for _ in 0..100 {
        let num = random(0.0, 1.0).unwrap();
        assert!(num >= 0.0 && num <= 1.0);
    }
}

#[test]
// Тест обработки инвертированного диапазона.
// Проверяет, что функция корректно обрабатывает случай, когда min > max.
fn test_random_inverted_range() {
    let num = random(10, 1).unwrap();
    assert!(num >= 1 && num <= 10);
}

#[test]
// Тест проверки некорректных значений.
// Проверяет обработку несравнимых значений.
fn test_random_invalid_comparison() {
    let result = random(f64::NAN, 1.0);
    assert!(matches!(result, Err(SysxError::InvalidSyntax(_))));
}


// random_bool tests

#[test]
// Тест генерации случайных булевых значений.
// Проверяет, что генерируются оба возможных значения.
fn test_random_bool() {
    let mut true_count = 0;
    let mut false_count = 0;
    
    for _ in 0..1000 {
        if random_bool().unwrap() {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }
    
    assert!(true_count > 0 && false_count > 0);
}


// random_string tests

#[test]
// Тест генерации строк заданной длины.
// Проверяет длину и допустимые символы по умолчанию.
fn test_random_string_default_charset() {
    let length = 10;
    let s = random_string(length, None).unwrap();
    
    assert_eq!(s.len(), length);
    assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
// Тест генерации строк с пользовательским набором символов.
// Проверяет использование указанного набора символов.
fn test_random_string_custom_charset() {
    let charset = "ABC123";
    let length = 15;
    let s = random_string(length, Some(charset)).unwrap();
    
    assert_eq!(s.len(), length);
    assert!(s.chars().all(|c| charset.contains(c)));
}

#[test]
// Тест обработки пустого набора символов.
// Проверяет, что функция возвращает ошибку при пустом наборе.
fn test_random_string_empty_charset() {
    let result = random_string(10, Some(""));
    assert!(matches!(result, Err(SysxError::InvalidSyntax(_))));
}


// random_bytes tests

#[test]
// Тест генерации случайных байтов.
// Проверяет длину и уникальность генерируемых байтов.
fn test_random_bytes() {
    let length = 16;
    let bytes = random_bytes(length).unwrap();
    
    assert_eq!(bytes.len(), length);
    
    // Проверяем, что байты не все одинаковые
    let unique_bytes: std::collections::HashSet<_> = bytes.iter().collect();
    assert!(unique_bytes.len() > 1);
}


// random_iter tests

#[test]
// Тест итератора случайных чисел.
// Проверяет генерацию последовательности чисел в диапазоне.
fn test_random_iterator() {
    let mut iter = random_iter(1, 10).unwrap();
    
    for _ in 0..100 {
        let num = iter.next().unwrap();
        assert!(num >= 1 && num <= 10);
    }
}

#[test]
// Тест итератора с инвертированным диапазоном.
// Проверяет корректность работы при min > max.
fn test_random_iterator_inverted_range() {
    let mut iter = random_iter(10, 1).unwrap();
    
    for _ in 0..100 {
        let num = iter.next().unwrap();
        assert!(num >= 1 && num <= 10);
    }
}


// random_range tests

#[test]
// Тест генерации чисел из включающего диапазона.
// Проверяет работу с RangeInclusive.
fn test_random_range() {
    for _ in 0..100 {
        let num = random_range(1..=10).unwrap();
        assert!(num >= 1 && num <= 10);
    }
}


// random_ratio tests

#[test]
// Тест генерации булевых значений с заданным отношением.
// Проверяет приблизительное соответствие заданной вероятности.
fn test_random_ratio() {
    let mut true_count = 0;
    let trials = 1000;
    
    for _ in 0..trials {
        if random_ratio(1, 4).unwrap() {
            true_count += 1;
        }
    }
    
    // Проверяем, что доля true примерно равна 0.25 (с погрешностью)
    let ratio = true_count as f64 / trials as f64;
    assert!(ratio > 0.15 && ratio < 0.35);
}

#[test]
// Тест обработки некорректного отношения.
// Проверяет ошибку при нулевом знаменателе.
fn test_random_ratio_zero_denominator() {
    let result = random_ratio(1, 0);
    assert!(matches!(result, Err(SysxError::InvalidSyntax(_))));
}
