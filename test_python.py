#!/usr/bin/env python3
"""
Evo-lang Python互操作测试脚本
Evo-lang Python Interoperability Test Script

测试Evo-lang的Python接口
Tests Evo-lang's Python interface
"""

import sys
import os

# 添加构建目录到Python路径
# Add build directory to Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'target', 'release'))

try:
    import evo
    print("[OK] Evo-lang模块导入成功 / Evo-lang module imported successfully")
except ImportError as e:
    print(f"[ERROR] 无法导入Evo-lang模块 / Failed to import Evo-lang module: {e}")
    print("\n提示 / Note: 需要先构建Python模块 / Need to build Python module first")
    print("使用以下命令构建 / Use the following command to build:")
    print("  maturin develop")
    print("或者手动构建 / Or build manually:")
    print("  cargo build --release")
    sys.exit(1)

def test_basic_operations():
    """测试基本操作 / Test basic operations"""
    print("\n=== 测试基本操作 / Testing Basic Operations ===")
    
    test_cases = [
        ("(+ 1 2)", "3", "简单加法 / Simple addition"),
        ("(* 3 4)", "12", "乘法运算 / Multiplication"),
        ("(- 10 3)", "7", "减法运算 / Subtraction"),
        ("(/ 20 4)", "5", "除法运算 / Division"),
    ]
    
    for code, expected, description in test_cases:
        try:
            result = evo.execute(code)
            print(f"[OK] {description}: {code} = {result} (期望 / Expected: {expected})")
            assert result == expected, f"期望 {expected}, 得到 {result}"
        except Exception as e:
            print(f"[ERROR] {description}: {code} - 错误 / Error: {e}")

def test_variables():
    """测试变量绑定 / Test variable binding"""
    print("\n=== 测试变量绑定 / Testing Variable Binding ===")
    
    test_cases = [
        ("(let x 10 (+ x 5))", "15", "变量绑定和计算 / Variable binding and calculation"),
        ("(let y 20 (* y 2))", "40", "变量乘法 / Variable multiplication"),
    ]
    
    for code, expected, description in test_cases:
        try:
            result = evo.execute(code)
            print(f"[OK] {description}: {code} = {result} (期望 / Expected: {expected})")
            assert result == expected, f"期望 {expected}, 得到 {result}"
        except Exception as e:
            print(f"[ERROR] {description}: {code} - 错误 / Error: {e}")

def test_conditionals():
    """测试条件表达式 / Test conditional expressions"""
    print("\n=== 测试条件表达式 / Testing Conditional Expressions ===")
    
    test_cases = [
        ("(if true 42 0)", "42", "条件表达式（真） / Conditional (true)"),
        ("(if false 0 42)", "42", "条件表达式（假） / Conditional (false)"),
        ("(if (> 5 3) 10 20)", "10", "比较条件 / Comparison condition"),
    ]
    
    for code, expected, description in test_cases:
        try:
            result = evo.execute(code)
            print(f"[OK] {description}: {code} = {result} (期望 / Expected: {expected})")
            assert result == expected, f"期望 {expected}, 得到 {result}"
        except Exception as e:
            print(f"[ERROR] {description}: {code} - 错误 / Error: {e}")

def test_functions():
    """测试函数定义和调用 / Test function definition and call"""
    print("\n=== 测试函数定义和调用 / Testing Function Definition and Call ===")
    
    # 使用解释器类来保持函数定义
    # Use interpreter class to maintain function definitions
    try:
        interpreter = evo.EvoInterpreter()
        
        # 定义函数
        define_code = "(def add (x y) (+ x y))"
        interpreter.execute(define_code)
        print(f"[OK] 函数定义成功 / Function defined: {define_code}")
        
        # 调用函数
        call_code = "(add 3 4)"
        result = interpreter.execute(call_code)
        print(f"[OK] 函数调用成功 / Function call: {call_code} = {result} (期望 / Expected: 7)")
        assert result == "7", f"期望 7, 得到 {result}"
    except Exception as e:
        print(f"[ERROR] 函数测试失败 / Function test failed: {e}")

def test_interpreter_class():
    """测试解释器类 / Test interpreter class"""
    print("\n=== 测试解释器类 / Testing Interpreter Class ===")
    
    try:
        interpreter = evo.EvoInterpreter()
        print("[OK] 解释器创建成功 / Interpreter created successfully")
        
        result = interpreter.execute("(+ 1 2)")
        print(f"[OK] 执行代码 / Execute code: (+ 1 2) = {result}")
        assert result == "3", f"期望 3, 得到 {result}"
        
        result_obj = interpreter.eval("(* 2 3)")
        print(f"[OK] 求值代码 / Eval code: (* 2 3) = {result_obj} (类型 / Type: {type(result_obj)})")
    except Exception as e:
        print(f"[ERROR] 解释器测试失败 / Interpreter test failed: {e}")

def test_parser_class():
    """测试解析器类 / Test parser class"""
    print("\n=== 测试解析器类 / Testing Parser Class ===")
    
    try:
        parser = evo.EvoParser(enable_nlu=True)
        print("[OK] 解析器创建成功 / Parser created successfully")
        
        ast = parser.parse("(+ 1 2)")
        print(f"[OK] 解析代码 / Parse code: (+ 1 2) = {ast}")
    except Exception as e:
        print(f"[ERROR] 解析器测试失败 / Parser test failed: {e}")

def main():
    """主测试函数 / Main test function"""
    print("=" * 60)
    print("Evo-lang Python互操作测试 / Evo-lang Python Interoperability Test")
    print("=" * 60)
    
    test_basic_operations()
    test_variables()
    test_conditionals()
    test_functions()
    test_interpreter_class()
    test_parser_class()
    
    print("\n" + "=" * 60)
    print("所有测试完成 / All tests completed")
    print("=" * 60)

if __name__ == "__main__":
    main()

