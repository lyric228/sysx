use std::str::FromStr;
use std::thread;
pub use std::time::Duration;

use thiserror::Error;

/// Спит заданное время, поддерживая различные входные типы и единицы измерения.
///
/// Функция принимает аргумент, который может быть:
/// - u64 (миллисекунды)
/// - Duration (прямо)
/// - &str с суффиксом единицы (например, "5s", "100ms", "1.5h")
/// - f64 (секунды)
///
/// После преобразования входного значения в Duration, функция вызывает thread::sleep.
///
/// # Пример
/// ```
/// // Сон 500 миллисекунд
/// sleep(500);
///
/// // Сон 2 секунды
/// sleep("2s");
///
/// // Сон 150 миллисекунд
/// sleep("150ms");
///
/// // Сон 1.8 секунды
/// sleep(1.8);
///
/// // Сон 3 секунды через Duration
/// sleep(std::time::Duration::from_secs(3));
/// ```
pub fn sleep<T: Into<SleepTime>>(time: T) {
    let duration = time.into().to_duration();
    thread::sleep(duration);
}

/// Возможные ошибки при парсинге времени сна.
#[derive(Debug, Error)]
pub enum SleepError {
    /// Неверный формат строки времени.
    #[error("Неверный формат времени: {0}")]
    InvalidFormat(String),
    /// Значение времени вне допустимого диапазона.
    #[error("Время вне допустимого диапазона")]
    OutOfRange,
    /// Отрицательное значение времени.
    #[error("Отрицательное время сна")]
    NegativeTime,
}

/// Представление времени сна.
///
/// Структура SleepTime содержит время в секундах в виде числа с плавающей точкой.
#[derive(Debug, Clone, Copy)]
pub struct SleepTime {
    seconds: f64,
}

impl SleepTime {
    /// Преобразует SleepTime в Duration с защитой от переполнения.
    ///
    /// Происходит разделение секунд и наносекунд, с корректировкой при переполнении наносекунд.
    ///
    /// # Возвращаемое значение
    /// Возвращает Duration, соответствующий значению SleepTime.
    ///
    /// # Пример
    /// ```
    /// let t = SleepTime { seconds: 1.5 };
    /// let d = t.to_duration();
    /// // d будет эквивалентно 1.5 секундам
    /// ```
    pub fn to_duration(self) -> Duration {
        assert!(self.seconds >= 0.0, "Время не может быть отрицательным");

        // Вычисляем целые секунды и наносекунды
        let secs = self.seconds.trunc() as u64;
        let nanos = (self.seconds.fract() * 1_000_000_000.0).round() as u32;

        // Корректировка при переполнении наносекунд
        let (secs, nanos) = if nanos >= 1_000_000_000 {
            (secs.saturating_add(1), 0)
        } else {
            (secs, nanos)
        };

        Duration::new(secs, nanos)
    }
}

impl From<u64> for SleepTime {
    /// Преобразует значение типа u64, рассматриваемое как миллисекунды, в SleepTime.
    ///
    /// # Пример
    /// ```
    /// // 2000 миллисекунд преобразуются в 2 секунды.
    /// let t: SleepTime = 2000u64.into();
    /// ```
    fn from(ms: u64) -> Self {
        SleepTime {
            seconds: ms as f64 / 1000.0,
        }
    }
}

impl From<Duration> for SleepTime {
    /// Преобразует тип Duration в SleepTime.
    ///
    /// # Пример
    /// ```
    /// let d = std::time::Duration::from_secs(5);
    /// let t: SleepTime = d.into();
    /// // t.seconds будет равно 5.0
    /// ```
    fn from(d: Duration) -> Self {
        SleepTime {
            seconds: d.as_secs_f64(),
        }
    }
}

impl From<f64> for SleepTime {
    /// Преобразует значение f64, представляющее секунды, в SleepTime.
    ///
    /// # Пример
    /// ```
    /// let t: SleepTime = 3.5f64.into();
    /// // t.seconds будет равно 3.5
    /// ```
    fn from(secs: f64) -> Self {
        SleepTime { seconds: secs }
    }
}

impl From<&str> for SleepTime {
    /// Преобразует строку в SleepTime, используя реализацию FromStr.
    ///
    /// # Пример
    /// ```
    /// let t: SleepTime = "2s".into();
    /// // t.seconds будет равно 2.0
    /// ```
    fn from(s: &str) -> Self {
        s.parse()
            .unwrap_or_else(|e| panic!("Не удалось распарсить строку времени: {}", e))
    }
}

impl FromStr for SleepTime {
    type Err = SleepError;

    /// Парсит строку с указанием единиц измерения:
    /// - ns: наносекунды
    /// - ms: миллисекунды
    /// - s: секунды (по умолчанию)
    /// - m: минуты
    /// - h: часы
    ///
    /// # Пример
    /// ```
    /// // Парсинг строки "1.5h" вернёт SleepTime с соответствующим значением в секундах.
    /// let t = SleepTime::from_str("1.5h").unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        let (num_part, unit) = s.split_at(
            s.find(|c: char| !c.is_numeric() && c != '.')
                .unwrap_or_else(|| s.len()),
        );

        let num: f64 = num_part
            .parse()
            .map_err(|_| SleepError::InvalidFormat(s.clone()))?;

        let multiplier = match unit {
            "ns" => 1e-9,
            "ms" => 1e-3,
            "m" => 60.0,
            "h" => 3600.0,
            "s" | "" => 1.0,
            _ => return Err(SleepError::InvalidFormat(s.clone())),
        };

        if num < 0.0 {
            return Err(SleepError::NegativeTime);
        }

        Ok(SleepTime {
            seconds: num * multiplier,
        })
    }
}

/// Пытается выполнить функцию sleep с заданным временем, возвращая ошибку в случае неудачи.
///
/// Функция принимает аргумент, который можно преобразовать в SleepTime.
/// Если время отрицательное, возвращается ошибка NegativeTime, иначе выполняется sleep.
///
/// # Возвращаемое значение
/// Возвращает Ok(()) при успешном выполнении сна или Err(SleepError) при ошибке.
///
/// # Пример
/// ```
/// // Попытка выполнить сон в 2 секунды
/// safe_sleep("2s").unwrap();
/// ```
pub fn safe_sleep<T: Into<SleepTime>>(time: T) -> Result<(), SleepError> {
    let sleep_time = match time.into() {
        t if t.seconds < 0.0 => return Err(SleepError::NegativeTime),
        t => t,
    };

    thread::sleep(sleep_time.to_duration());
    Ok(())
}
