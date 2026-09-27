# Where Clause в Rust

## Что такое `where`

**`where`** — это **синтаксическая конструкция** для **отделения** ограничений trait bounds от **списка параметров**. Позволяет **вынести** ограничения **после** сигнатуры.

```rust
fn foo<T>(x: T)
where
    T: Display + Clone,
{
    // ...
}
```

**Эквивалент:**

```rust
fn foo<T: Display + Clone>(x: T) {
    // ...
}
```

## Синтаксис

### С inline-ограничениями

```rust
fn foo<T: Display, U: Clone + Debug>(x: T, y: U) { ... }
```

### С `where`

```rust
fn foo<T, U>(x: T, y: U)
where
    T: Display,
    U: Clone + Debug,
{
    // ...
}
```

**Оба** варианта — **эквивалентны**.

## Когда `where` **удобнее**

### 1. **Много** ограничений

```rust
// Inline — длинно и нечитаемо
fn process<T: Display + Clone + Debug + Send + Sync, U: Iterator<Item = T> + Clone>(
    x: T, iter: U
) { ... }

// where — читаемо
fn process<T, U>(x: T, iter: U)
where
    T: Display + Clone + Debug + Send + Sync,
    U: Iterator<Item = T> + Clone,
{
    // ...
}
```

### 2. Ограничение на **ассоциированный тип**

```rust
// Inline — невозможно или громоздко
fn print_all<I: IntoIterator>(item: I)
where
    I::Item: Display,
{
    // ...
}
```

**В inline** это **нельзя** записать:

```rust
fn print_all<I: IntoIterator<Item: Display>>(item: I) { }   // ❌ синтаксис
```

**`where`** — **единственный** способ.

### 3. Ограничение на **`Self`**

```rust
trait MyTrait {
    fn method(&self)
    where
        Self: Clone,
    {
        // ...
    }
}
```

### 4. **Многострочные** сигнатуры

```rust
// Inline — вся строка «уезжает»
fn complex_function<T: Display + Clone, U: Debug + Send, V: Iterator<Item = T> + Sync>(
    a: T, b: U, c: V
) -> Result<T, U> { ... }

// where — вертикально
fn complex_function<T, U, V>(a: T, b: U, c: V) -> Result<T, U>
where
    T: Display + Clone,
    U: Debug + Send,
    V: Iterator<Item = T> + Sync,
{
    // ...
}
```

### 5. **Diff** в системе контроля версий

При **изменении** ограничений `where` даёт **чистый** diff:

```diff
 fn foo<T, U>(x: T, y: U)
 where
-    T: Display,
+    T: Display + Clone,
     U: Debug,
 {
```

**Inline:**

```diff
-fn foo<T: Display, U: Debug>(x: T, y: U) {
+fn foo<T: Display + Clone, U: Debug>(x: T, y: U) {
```

**Вся строка** меняется — **сложнее** читать.

## Разбор примера

```rust
use std::fmt::Display;

pub fn print_all<I>(item: I)
where
    I: IntoIterator,
    I::Item: Display,
{
    for x in item {
        println!("{}", x);
    }
}

fn main() {
    print_all(vec![1, 2, 3, 4, 5]);
    // 1
    // 2
    // 3
    // 4
    // 5
}
```

### Что происходит

- **`I: IntoIterator`** — `I` можно **итерировать**.
- **`I::Item: Display`** — элемент итератора **реализует `Display`**.
- **`for x in item`** — перебор.
- **`println!("{}", x)`** — вывод.

### Почему `where` **необходим**

**`I::Item: Display`** — ограничение на **ассоциированный тип**. **Inline** это **нельзя** записать:

```rust
pub fn print_all<I: IntoIterator<Item: Display>>(item: I) { }   // ❌
```

**`where`** — **единственный** способ.

## Сводная таблица

| Случай | Inline | `where` |
|---|---|---|
| **1 ограничение** | ✅ | ⚠️ |
| **Много ограничений** | ⚠️ | ✅ |
| **Ассоциированный тип** | ❌ | ✅ |
| **Ограничение на `Self`** | ❌ | ✅ |
| **Многострочная сигнатура** | ⚠️ | ✅ |
| **Diff** | ⚠️ | ✅ |

## Где можно использовать `where`

### 1. **Функции**

```rust
fn foo<T>(x: T)
where T: Display { }
```

### 2. **Структуры**

```rust
struct Wrapper<T>
where
    T: Display,
{
    value: T,
}
```

### 3. **Enum**

```rust
enum MyEnum<T>
where
    T: Display,
{
    Value(T),
}
```

### 4. **Impl-блоки**

```rust
impl<T> MyStruct<T>
where
    T: Display,
{
    fn show(&self) { }
}
```

### 5. **Трейты**

```rust
trait MyTrait<T>
where
    T: Display,
{
    fn method(&self, x: T);
}
```

### 6. **Методы трейта**

```rust
trait MyTrait {
    fn method(&self)
    where
        Self: Clone;
}
```

### 7. **`impl Trait` в аргументе**

```rust
fn foo(x: impl Display)
where
    i32: From<u8>,   // ← can be here
{ }
```

## Сводная таблица

| Место | `where` работает? |
|---|---|
| Функции | ✅ |
| Структуры | ✅ |
| Enum | ✅ |
| `impl`-блоки | ✅ |
| Трейты | ✅ |
| Методы трейта | ✅ |
| `impl Trait` | ⚠️ |

## Пример: комбинация inline и `where`

```rust
fn foo<T: Clone, U>(x: T, y: U)
where
    U: Display + Debug,
{
    // T: Clone — inline
    // U: Display + Debug — where
}
```

**Можно** комбинировать.

## Практические советы

### 1. **1–2 ограничения** — inline

```rust
fn foo<T: Display>(x: T) { }
```

### 2. **3+ ограничений** — `where`

```rust
fn foo<T, U>(x: T, y: U)
where
    T: Display + Clone + Debug,
    U: Send + Sync,
{ }
```

### 3. **Ассоциированный тип** — всегда `where`

```rust
fn foo<I>(x: I)
where
    I: Iterator,
    I::Item: Display,
{ }
```

### 4. **Многострочная сигнатура** — `where`

```rust
fn complex<T, U, V>(
    a: T, b: U, c: V
) -> Result<T, U>
where
    T: Display,
    U: Clone,
    V: Debug,
{ }
```

## Сводная таблица

| Аспект | Описание |
|---|---|
| **`where`** | Отделяет ограничения от параметров |
| **Эквивалент** | `<T: Trait>` |
| **Когда** | Много ограничений, ассоциированный тип, многострочность |
| **Где** | Функции, структуры, impl, трейты, enum |
| **Diff** | Чище с `where` |

## Итог

- **`where`** — **выносит** ограничения **после** сигнатуры.
- **Эквивалент** inline `<T: Trait>`.
- **Удобнее:**
  - **много** ограничений;
  - **ассоциированный тип** (`I::Item: Display`);
  - **`Self`** в трейтах;
  - **многострочные** сигнатуры;
  - **чистый diff**.
- **Работает** в функциях, структурах, enum, impl, трейтах.
- **В вашем примере:**
  - `I: IntoIterator`;
  - `I::Item: Display` — **только** через `where`.
- **Правило:** 1–2 ограничения — inline; 3+ или ассоциированный тип — `where`.
