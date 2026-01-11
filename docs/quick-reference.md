# Aevolang 快速参考 / Quick Reference

## 语法速查表 / Syntax Cheat Sheet

### 字面量 / Literals

```
42          ; 整数
3.14        ; 浮点数
"hello"     ; 字符串
true        ; 布尔值
false       ; 布尔值
null        ; 空值
```

### 操作符 / Operators

```lisp
(+ a b)     ; 加法
(- a b)     ; 减法
(* a b)     ; 乘法
(/ a b)     ; 除法
(= a b)     ; 等于
(!= a b)    ; 不等于
(< a b)     ; 小于
(> a b)     ; 大于
(<= a b)    ; 小于等于
(>= a b)    ; 大于等于
```

### 特殊形式 / Special Forms

```lisp
; 变量绑定
(let name value body)

; 条件表达式
(if condition then-expr else-expr)

; 函数定义
(def name (params...) body)
(function name (params...) body)
```

### 函数调用 / Function Calls

```lisp
(function-name arg1 arg2 ...)
```

## 常用模式 / Common Patterns

### 定义变量并计算

```lisp
(let x 10 (+ x 5))  ; 15
```

### 定义函数

```lisp
(def add (x y) (+ x y))
```

### 递归函数

```lisp
(def factorial (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1)))))
```

### 条件判断

```lisp
(if (> x 0) x (- x))  ; 绝对值
```

## 数据类型 / Data Types

| 类型 | 示例 | 说明 |
|------|------|------|
| Int | `42` | 整数 |
| Float | `3.14` | 浮点数 |
| String | `"hello"` | 字符串 |
| Bool | `true`, `false` | 布尔值 |
| Null | `null` | 空值 |

## 错误类型 / Error Types

- `SyntaxError` - 语法错误
- `UndefinedVariable` - 未定义变量
- `TypeError` - 类型错误
- `DivisionByZero` - 除以零
- `RuntimeError` - 运行时错误

## 代码示例 / Code Examples

### 计算平方

```lisp
(def square (x) (* x x))
(square 5)  ; 25
```

### 计算最大值

```lisp
(def max (a b)
    (if (> a b) a b))
```

### 判断奇偶

```lisp
(def isEven (n)
    (= (% n 2) 0))
```

## 提示 / Tips

1. 所有表达式都用括号包围
2. 第一个元素通常是操作符或函数名
3. 使用 `;` 添加注释
4. 函数名和变量名使用小写字母和下划线
5. 保持代码格式清晰

## 相关链接 / Related Links

- [完整语法参考](syntax-reference.md)
- [快速入门](getting-started.md)
- [示例代码](../examples/)

