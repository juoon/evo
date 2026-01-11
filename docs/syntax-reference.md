# Aevolang 语法参考 / Syntax Reference

## 语法概述 / Syntax Overview

Aevolang 使用 S-expression 语法，所有代码都是括号包围的列表。第一个元素通常是操作符或函数名，后续元素是参数。

## 表达式类型 / Expression Types

### 1. 字面量表达式 / Literal Expressions

```lisp
42          ; 整数
3.14        ; 浮点数
"hello"     ; 字符串
true        ; 布尔值 true
false       ; 布尔值 false
null        ; 空值
```

### 2. 变量引用 / Variable References

```lisp
x           ; 引用变量 x
myVar       ; 引用变量 myVar
```

### 3. 函数调用 / Function Calls

```lisp
(function-name arg1 arg2 ...)
```

示例：
```lisp
(+ 1 2)           ; 调用 + 函数
(print "hello")   ; 调用 print 函数
```

### 4. 特殊形式 / Special Forms

#### let - 变量绑定

```lisp
(let variable-name value body)
```

示例：
```lisp
(let x 10 (+ x 5))  ; 定义 x = 10，在 body 中使用
```

#### if - 条件表达式

```lisp
(if condition then-expr else-expr)
```

示例：
```lisp
(if (> x 0) x (- x))  ; 如果 x > 0 返回 x，否则返回 -x
```

#### def / function - 函数定义

```lisp
(def function-name (param1 param2 ...) body)
(function function-name (param1 param2 ...) body)
```

示例：
```lisp
(def add (x y) (+ x y))
(def factorial (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1)))))
```

## 操作符 / Operators

### 算术操作符 / Arithmetic Operators

```lisp
(+ a b)     ; 加法
(- a b)     ; 减法
(* a b)     ; 乘法
(/ a b)     ; 除法
```

### 比较操作符 / Comparison Operators

```lisp
(= a b)     ; 等于
(!= a b)    ; 不等于
(< a b)     ; 小于
(> a b)     ; 大于
(<= a b)    ; 小于等于
(>= a b)    ; 大于等于
```

## 数据类型 / Data Types

### 整数 / Integer

```lisp
0
42
-10
1000000
```

### 浮点数 / Float

```lisp
3.14
-0.5
1.0
```

### 字符串 / String

```lisp
"hello"
"world"
"multi-line
string"
```

支持转义字符：
```lisp
"line1\nline2"     ; 换行
"tab\there"        ; 制表符
"quote\"here"      ; 引号
```

### 布尔值 / Boolean

```lisp
true
false
```

### 空值 / Null

```lisp
null
nil
```

## 作用域 / Scope

`let` 绑定创建局部作用域：

```lisp
(let x 10
    (let y 20
        (+ x y)))  ; x 和 y 都在作用域内
```

外层作用域的变量在内层可见，内层可以遮蔽外层变量。

## 注释 / Comments

使用分号 `;` 开始注释，直到行尾：

```lisp
; 这是单行注释
(+ 1 2)  ; 行尾注释
```

## 代码风格 / Code Style

### 缩进建议

```lisp
; 好的风格
(def factorial (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1)))))

; 也可以写成一行（简单表达式）
(def add (x y) (+ x y))
```

### 命名约定

- 变量和函数名使用小写字母和下划线
- 常量使用大写字母和下划线
- 函数名应该描述其功能

## 错误处理 / Error Handling

### 常见错误

1. **语法错误 / Syntax Error**
   ```
   SyntaxError("Unexpected character '*' at line 1, column 3")
   ```

2. **未定义变量 / Undefined Variable**
   ```
   UndefinedVariable("x")
   ```

3. **类型错误 / Type Error**
   ```
   TypeError("Invalid types for addition")
   ```

4. **除以零 / Division By Zero**
   ```
   DivisionByZero
   ```

## 最佳实践 / Best Practices

1. **使用有意义的变量名**
   ```lisp
   ; 好
   (let userAge 25 ...)
   
   ; 不好
   (let x 25 ...)
   ```

2. **保持函数简短**
   ```lisp
   ; 将复杂逻辑拆分为多个小函数
   (def calculateTotal (items)
       (sum (map getPrice items)))
   ```

3. **使用注释说明复杂逻辑**
   ```lisp
   ; 使用快速排序算法
   (def quicksort (list)
       ...)
   ```

