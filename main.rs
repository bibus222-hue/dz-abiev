// Функция для расчета оценки задачи
fn calculate_task_score(priority: u32, complexity: u32) -> u32 {
    // Возвращаем произведение приоритета и сложности
    priority * complexity
}

fn main() {
    // Примеры использования функции
    let priority = 3;
    let complexity = 5;
    let score = calculate_task_score(priority, complexity);
    
    println!("Приоритет: {}", priority);
    println!("Сложность: {}", complexity);
    println!("Оценка задачи: {}", score);
    println!("{} * {} = {}", priority, complexity, score);
    
    // Дополнительные тесты
    println!("\n--- Дополнительные примеры ---");
    println!("Задача 1: {} * {} = {}", 2, 4, calculate_task_score(2, 4));
    println!("Задача 2: {} * {} = {}", 5, 3, calculate_task_score(5, 3));
    println!("Задача 3: {} * {} = {}", 1, 10, calculate_task_score(1, 10));
}

// Тесты для проверки работы функции
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_task_score() {
        assert_eq!(calculate_task_score(3, 5), 15);
        assert_eq!(calculate_task_score(2, 4), 8);
        assert_eq!(calculate_task_score(5, 3), 15);
        assert_eq!(calculate_task_score(1, 10), 10);
        assert_eq!(calculate_task_score(0, 10), 0);
        assert_eq!(calculate_task_score(10, 0), 0);
    }
    
    #[test]
    fn test_with_large_numbers() {
        assert_eq!(calculate_task_score(100, 200), 20000);
        assert_eq!(calculate_task_score(50, 50), 2500);
    }
}

// Функция с проверкой на переполнение
fn calculate_task_score(priority: u32, complexity: u32) -> u32 {
    match priority.checked_mul(complexity) {
        Some(result) => result,
        None => {
            eprintln!("Предупреждение: переполнение при умножении {} * {}", priority, complexity);
            u32::MAX // Возвращаем максимальное значение
        }
    }
}

fn main() {
    let score = calculate_task_score(3, 5);
    println!("Оценка задачи: {}", score); // Выведет: 15
}

// Используем u64 для избежания переполнения
fn calculate_task_score(priority: u32, complexity: u32) -> u64 {
    (priority as u64) * (complexity as u64)
}

fn main() {
    let score = calculate_task_score(3, 5);
    println!("Оценка задачи: {}", score); // Выведет: 15
    
    // Работает с большими числами
    let large_score = calculate_task_score(100000, 100000);
    println!("Большая оценка: {}", large_score); // Выведет: 10000000000
}

fn calculate_task_score(priority: u32, complexity: u32) -> u32 {
    let score = priority * complexity;
    
    // Логируем вычисление
    println!("[DEBUG] Вычисление оценки: {} (приоритет) * {} (сложность) = {}", 
             priority, complexity, score);
    
    score
}

fn main() {
    let task_score = calculate_task_score(3, 5);
    assert_eq!(task_score, 15);
    println!("Финальная оценка: {}", task_score);
}