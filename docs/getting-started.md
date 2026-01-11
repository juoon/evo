# Aevolang 快速入门指南 / Getting Started Guide

## 安装 / Installation

### 前置要求 / Prerequisites

- Rust 1.70+ (安装方法: https://rustup.rs/)
- Cargo (随 Rust 一起安装)

### 构建项目 / Building the Project

```bash
# 克隆仓库（如果从远程获取）
git clone <repository-url>
cd aevo

# 构建项目
cargo build

# 运行示例程序
cargo run
```

## 基本语法 / Basic Syntax

Aevolang 使用 S-expression 风格语法（类似 Lisp），所有代码都是表达式。

### 字面量 / Literals

```lisp
42          ; 整数 / Integer
3.14        ; 浮点数 / Float
"hello"     ; 字符串 / String
true        ; 布尔值 / Boolean
false       ; 布尔值 / Boolean
null        ; 空值 / Null
```

### 算术运算 / Arithmetic Operations

```lisp
(+ 1 2)     ; 加法: 1 + 2 = 3
(- 10 3)    ; 减法: 10 - 3 = 7
(* 3 4)     ; 乘法: 3 * 4 = 12
(/ 10 2)    ; 除法: 10 / 2 = 5
```

### 变量绑定 / Variable Binding

使用 `let` 绑定变量：

```lisp
(let x 10 (+ x 5))  ; 定义 x = 10, 然后计算 x + 5 = 15
```

### 条件表达式 / Conditional Expressions

使用 `if` 进行条件判断：

```lisp
(if true 42 0)      ; 如果为真，返回 42，否则返回 0
(if false 0 42)     ; 如果为假，返回 0，否则返回 42
```

### 函数定义 / Function Definition

使用 `def` 或 `function` 定义函数：

```lisp
(def add (x y) (+ x y))  ; 定义加法函数
(add 3 4)                ; 调用函数: 7
```

### 比较操作符 / Comparison Operators

```lisp
(= 5 5)     ; 等于: true
(!= 3 4)    ; 不等于: true
(< 2 5)     ; 小于: true
(> 10 5)    ; 大于: true
(<= 3 3)    ; 小于等于: true
(>= 5 3)    ; 大于等于: true
```

## 第一个程序 / Your First Program

创建一个简单的程序：

```lisp
; 定义计算平方的函数
(def square (x) (* x x))

; 使用函数
(square 5)  ; 返回 25
```

## 递归函数 / Recursive Functions

Aevolang 支持递归函数调用：

```lisp
; 计算阶乘
(def factorial (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1)))))

(factorial 5)  ; 返回 120
```

## 运行代码 / Running Code

### 方式1: 使用示例程序

```bash
cargo run
```

这会运行内置的演示程序，展示各种功能。

### 方式2: 在代码中使用

在 Rust 代码中使用 Aevolang：

```rust
use aevo::parser::AdaptiveParser;
use aevo::runtime::Interpreter;

let parser = AdaptiveParser::new(true);
let mut interpreter = Interpreter::new();

let code = "(+ 1 2)";
match parser.parse(code) {
    Ok(ast) => {
        match interpreter.execute(&ast) {
            Ok(value) => println!("Result: {}", value),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    Err(e) => println!("Parse error: {:?}", e),
}
```

## 下一步 / Next Steps

- 查看 [完整语法参考](syntax-reference.md)
- 浏览 [示例代码](../examples/)
- 了解 [高级特性](advanced-features.md)

