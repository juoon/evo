# Aevolang 高级特性 / Advanced Features

## 函数作用域 / Function Scope

函数定义会创建新的作用域，参数只在函数体内可见：

```lisp
(let x 10
    (def addX (y) (+ x y))  ; x 来自外层作用域
    (addX 5))               ; 结果: 15
```

## 递归 / Recursion

Aevolang 完全支持递归函数调用：

```lisp
(def factorial (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1)))))
```

## 嵌套函数 / Nested Functions

可以在函数内部定义辅助函数：

```lisp
(def outer (x)
    (def inner (y) (+ x y))
    (inner 5))
```

## 闭包 / Closures

函数可以捕获外部作用域的变量：

```lisp
(let multiplier 3
    (def multiply (x) (* multiplier x))
    (multiply 4))  ; 结果: 12
```

## 函数组合 / Function Composition

可以组合多个函数：

```lisp
(def square (x) (* x x))
(def addOne (x) (+ x 1))

; 先平方再加1
(addOne (square 3))  ; 结果: 10
```

## 错误处理模式 / Error Handling Patterns

### 防御性编程

```lisp
(def safeDivide (a b)
    (if (= b 0)
        null
        (/ a b)))
```

### 参数验证

```lisp
(def factorial (n)
    (if (< n 0)
        null  ; 或抛出错误
        (if (= n 0)
            1
            (* n (factorial (- n 1))))))
```

## 性能优化建议 / Performance Tips

1. **避免不必要的递归**
   - 对于简单计算，使用迭代更高效
   - 考虑尾递归优化（未来支持）

2. **缓存计算结果**
   - 对于重复计算，考虑使用记忆化（未来支持）

3. **减少函数调用开销**
   - 内联简单函数（未来支持）

## 未来特性 / Future Features

以下特性正在开发中：

1. **列表操作**
   ```lisp
   (list 1 2 3)
   (map square (list 1 2 3))
   (filter isEven (list 1 2 3))
   ```

2. **模式匹配**
   ```lisp
   (match x
       (0 "zero")
       (1 "one")
       (_ "other"))
   ```

3. **类型系统**
   ```lisp
   (def add (x: Int y: Int) -> Int
       (+ x y))
   ```

4. **模块系统**
   ```lisp
   (import math)
   (math.sqrt 16)
   ```

5. **自然语言编程**
   ```lisp
   ; 用自然语言描述功能
   "创建一个函数，计算两个数的和"
   ```

## 调试技巧 / Debugging Tips

1. **使用 print 函数**
   ```lisp
   (print "Value of x:" x)
   ```

2. **分步执行**
   - 将复杂表达式拆分为多个步骤
   - 逐步验证每个部分

3. **检查类型**
   - 确保操作符的参数类型正确
   - 注意整数和浮点数的区别

## 常见陷阱 / Common Pitfalls

1. **括号不匹配**
   ```lisp
   ; 错误
   (+ 1 2
   
   ; 正确
   (+ 1 2)
   ```

2. **变量作用域**
   ```lisp
   ; 错误: x 未定义
   (let y 10 (+ x y))
   
   ; 正确
   (let x 5 (let y 10 (+ x y)))
   ```

3. **函数参数数量**
   ```lisp
   ; 错误: 参数数量不匹配
   (def add (x y) (+ x y))
   (add 1 2 3)
   
   ; 正确
   (add 1 2)
   ```

