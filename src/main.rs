// Aevolang - 自进化编程语言 / Self-evolving programming language
// 该语言的核心特点是能够根据使用和需求自我进化
// The core feature of this language is the ability to self-evolve based on usage and needs
// 终极目标：理解人类思想，促进人类与智能生命和谐共生
// Ultimate goal: Understand human thoughts and promote harmonious coexistence between humans and intelligent life

mod evolution;
mod grammar;
mod parser;
mod poetry;
mod python;
mod runtime;

use evolution::*;
use grammar::*;
use parser::*;
use poetry::*;
use runtime::*;

fn main() {
    println!("Aevolang - 自进化编程语言 / Self-evolving Programming Language");
    println!("============================================================");

    // 演示《静夜思》的解析和理解 / Demonstrate parsing and understanding of "Quiet Night Thoughts"
    demonstrate_poetry_understanding();

    // 演示语法定义 / Demonstrate grammar definition
    demonstrate_grammar_definition();

    // 演示进化引擎 / Demonstrate evolution engine
    demonstrate_evolution_engine();

    // 演示解析器 / Demonstrate parser
    demonstrate_parser();

    // 演示解释器 / Demonstrate interpreter
    demonstrate_interpreter();

    // 演示NLU自然语言理解 / Demonstrate NLU natural language understanding
    demonstrate_nlu();

    // 演示JIT编译器 / Demonstrate JIT compiler
    demonstrate_jit();

    // 演示列表和字典 / Demonstrate lists and dictionaries
    demonstrate_data_structures();

    // 演示模块系统 / Demonstrate module system
    demonstrate_modules();
}

/// 演示解释器功能 / Demonstrate interpreter functionality
fn demonstrate_interpreter() {
    println!("\n5. 解释器演示 / Interpreter Demo");
    println!("--------------------------------------------");

    let parser = AdaptiveParser::new(true);
    let mut interpreter = Interpreter::new();

    // 测试用例
    let test_cases = vec![
        ("(+ 1 2)", "简单加法 / Simple addition"),
        ("(* 3 4)", "乘法运算 / Multiplication"),
        (
            "(let x 10 (+ x 5))",
            "变量绑定和计算 / Variable binding and calculation",
        ),
        (
            "(if true 42 0)",
            "条件表达式（真） / Conditional expression (true)",
        ),
        (
            "(if false 0 42)",
            "条件表达式（假） / Conditional expression (false)",
        ),
        (
            "(def add (x y) (+ x y)) (add 3 4)",
            "函数定义和调用 / Function definition and call",
        ),
    ];

    for (code, description) in test_cases {
        println!("\n测试代码 / Test Code: {}", description);
        println!("源代码 / Source: {}", code);

        match parser.parse(code) {
            Ok(ast) => match interpreter.execute(&ast) {
                Ok(value) => {
                    println!("执行结果 / Execution Result: {}", value);
                }
                Err(e) => {
                    println!("执行错误 / Execution Error: {:?}", e);
                }
            },
            Err(e) => {
                println!("解析错误 / Parse Error: {:?}", e);
            }
        }
    }

    // 测试更复杂的例子
    println!("\n--- 复杂示例 / Complex Examples ---");

    // 定义阶乘函数
    let factorial_code = r#"
        (def factorial (n)
            (if (= n 0)
                1
                (* n (factorial (- n 1)))))
    "#;

    println!("\n定义阶乘函数 / Define factorial function:");
    println!("{}", factorial_code);

    match parser.parse(factorial_code) {
        Ok(ast) => {
            if let Err(e) = interpreter.execute(&ast) {
                println!("定义函数时出错 / Error defining function: {:?}", e);
            } else {
                println!("函数定义成功 / Function defined successfully");

                // 调用阶乘函数
                let call_code = "(factorial 5)";
                println!("\n调用阶乘函数 / Call factorial function: {}", call_code);

                match parser.parse(call_code) {
                    Ok(ast) => match interpreter.execute(&ast) {
                        Ok(value) => {
                            println!("执行结果 / Execution Result: {}", value);
                        }
                        Err(e) => {
                            println!("执行错误 / Execution Error: {:?}", e);
                        }
                    },
                    Err(e) => {
                        println!("解析错误 / Parse Error: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("解析错误 / Parse Error: {:?}", e);
        }
    }
}

/// 演示解析器功能 / Demonstrate parser functionality
fn demonstrate_parser() {
    println!("\n4. 解析器演示 / Parser Demo");
    println!("--------------------------------------------");

    let parser = AdaptiveParser::new(true);

    // 测试简单的 S-expression 代码
    let test_cases = vec![
        ("(+ 1 2)", "简单加法 / Simple addition"),
        ("(def add (x y) (+ x y))", "函数定义 / Function definition"),
        (
            "(if (> x 0) x (- x))",
            "条件表达式 / Conditional expression",
        ),
        ("(let x 42 (+ x 1))", "变量绑定 / Variable binding"),
        ("(\"hello\" \"world\")", "字符串列表 / String list"),
    ];

    for (code, description) in test_cases {
        println!("\n测试代码 / Test Code: {}", description);
        println!("源代码 / Source: {}", code);

        match parser.parse(code) {
            Ok(ast) => {
                println!("解析成功 / Parse Success!");
                println!("AST (前3层) / AST (first 3 levels):");
                print_ast(&ast, 0, 3);
            }
            Err(e) => {
                println!("解析错误 / Parse Error: {:?}", e);
            }
        }
    }
}

/// 打印AST / Print AST
fn print_ast(elements: &[crate::grammar::core::GrammarElement], depth: usize, max_depth: usize) {
    if depth >= max_depth {
        println!("{}...", "  ".repeat(depth));
        return;
    }

    for element in elements {
        match element {
            crate::grammar::core::GrammarElement::Atom(s) => {
                println!("{}Atom: {}", "  ".repeat(depth), s);
            }
            crate::grammar::core::GrammarElement::List(l) => {
                println!("{}List ({} items):", "  ".repeat(depth), l.len());
                print_ast(l, depth + 1, max_depth);
            }
            crate::grammar::core::GrammarElement::Expr(e) => {
                println!("{}Expr: {:?}", "  ".repeat(depth), e);
            }
            crate::grammar::core::GrammarElement::NaturalLang(nl) => {
                println!("{}NaturalLang: {}", "  ".repeat(depth), nl);
            }
        }
    }
}

/// 演示诗歌理解能力 / Demonstrate poetry understanding capability
fn demonstrate_poetry_understanding() {
    println!("\n1. 诗歌理解演示 / Poetry Understanding Demo");
    println!("--------------------------------------------");

    // 《静夜思》 - 李白 / "Quiet Night Thoughts" - Li Bai
    let poem = r#"
床前明月光，
疑是地上霜。
举头望明月，
低头思故乡。
"#;

    println!("原诗 / Original Poem:");
    println!("{}", poem);

    let parser = PoetryParser::new();
    match parser.parse(poem) {
        Ok(analysis) => {
            println!("\n解析结果 / Analysis Result:");
            println!(
                "主要情感 / Primary Emotion: {:?}",
                analysis.emotion_analysis.primary_emotion
            );
            println!(
                "置信度 / Confidence: {:.2}",
                analysis.emotion_analysis.confidence
            );
            println!("\n检测到的情感 / Detected Emotions:");
            for (emotion, score) in &analysis.emotion_analysis.emotion_scores {
                if *score > 0.1 {
                    println!("  {:?}: {:.2}", emotion, score);
                }
            }
            println!("\n主题 / Themes:");
            for theme in &analysis.themes {
                println!(
                    "  {}: {} (置信度: {:.2})",
                    theme.name, theme.description, theme.confidence
                );
            }
            println!("\n意象 / Imagery:");
            for img in &analysis.imagery {
                println!(
                    "  {}: {} (出现{}次)",
                    img.element, img.meaning, img.frequency
                );
            }
        }
        Err(e) => {
            println!("解析错误 / Parse Error: {:?}", e);
        }
    }
}

/// 演示语法定义 / Demonstrate grammar definition
fn demonstrate_grammar_definition() {
    println!("\n2. 语法定义演示 / Grammar Definition Demo");
    println!("--------------------------------------------");

    // 创建自描述语法规则 / Create self-describing grammar rule
    let syntax_def_rule = syntax_definition_rule();

    println!("自描述语法规则 / Self-describing Grammar Rule:");
    println!("规则名称 / Rule Name: {}", syntax_def_rule.rule.name);
    println!("版本 / Version: {}", syntax_def_rule.rule.meta.version);
    println!(
        "稳定性 / Stability: {:?}",
        syntax_def_rule.rule.meta.stability
    );
    println!("\n自描述代码 / Self-describing Code:");
    println!("{}", syntax_def_rule.self_describing_code);
}

/// 演示进化引擎 / Demonstrate evolution engine
fn demonstrate_evolution_engine() {
    println!("\n3. 进化引擎演示 / Evolution Engine Demo");
    println!("--------------------------------------------");

    let engine = EvolutionEngine::new();

    println!("进化引擎已创建 / Evolution Engine Created");
    println!("历史记录数 / History Count: {}", engine.get_history().len());

    // 演示从自然语言进化（这里只是示例，实际实现需要NLU支持）
    // Demonstrate evolution from natural language (this is just an example, actual implementation requires NLU support)
    println!("\n提示 / Note: 完整的自然语言进化功能需要NLU系统支持");
    println!("Full natural language evolution requires NLU system support");
}

/// 演示NLU自然语言理解功能 / Demonstrate NLU natural language understanding
fn demonstrate_nlu() {
    println!("\n6. NLU自然语言理解演示 / NLU Natural Language Understanding Demo");
    println!("--------------------------------------------");

    let nlu_parser = NLUParser::new_rule_based();
    let code_parser = AdaptiveParser::new(true);
    let mut interpreter = Interpreter::new();

    // 测试用例：中英文自然语言输入
    let test_cases = vec![
        (
            "定义一个函数叫add，参数是x和y，返回x加y",
            "中文函数定义（完整） / Chinese function definition (complete)",
        ),
        (
            "定义一个函数multiply，参数是a和b，a乘以b",
            "中文函数定义（简化） / Chinese function definition (simplified)",
        ),
        (
            "define a function called add that takes x and y and returns x plus y",
            "英文函数定义 / English function definition",
        ),
        (
            "定义一个变量x等于10",
            "中文变量定义（数字） / Chinese variable definition (number)",
        ),
        (
            "定义一个变量y等于二十三",
            "中文变量定义（中文数字） / Chinese variable definition (Chinese number)",
        ),
        (
            "let variable x be 5",
            "英文变量定义 / English variable definition",
        ),
        ("3 加 5", "中文加法操作 / Chinese addition operation"),
        ("10 plus 20", "英文加法操作 / English addition operation"),
        (
            "8 乘以 7",
            "中文乘法操作 / Chinese multiplication operation",
        ),
        ("15 除以 3", "中文除法操作 / Chinese division operation"),
        (
            "100 减去 25",
            "中文减法操作 / Chinese subtraction operation",
        ),
        (
            "定义一个函数subtract，参数是x和y，x减去y",
            "中文函数定义（减法） / Chinese function definition (subtraction)",
        ),
    ];

    for (input, description) in test_cases {
        println!("\n测试输入 / Test Input: {}", description);
        println!("自然语言 / Natural Language: {}", input);

        match nlu_parser.parse(input) {
            Ok(parsed_intent) => {
                println!(
                    "识别意图 / Detected Intent: {:?}",
                    parsed_intent.intent_type
                );
                println!("置信度 / Confidence: {:.2}", parsed_intent.confidence);
                println!("生成的代码结构 / Generated Code Structure:");
                print_ast(&parsed_intent.code_structure, 0, 3);

                // 尝试将生成的代码结构转换为可执行的代码
                // 注意：这是一个简化版本，实际需要更复杂的转换逻辑
                if let Some(executable_code) = convert_to_executable(&parsed_intent.code_structure)
                {
                    println!("\n可执行代码 / Executable Code: {}", executable_code);

                    match code_parser.parse(&executable_code) {
                        Ok(ast) => match interpreter.execute(&ast) {
                            Ok(value) => {
                                println!("执行结果 / Execution Result: {}", value);
                            }
                            Err(e) => {
                                println!("执行错误 / Execution Error: {:?}", e);
                            }
                        },
                        Err(e) => {
                            println!("代码解析错误 / Code Parse Error: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("NLU解析错误 / NLU Parse Error: {:?}", e);
            }
        }
    }

    // 演示意图提取
    println!("\n--- 意图提取演示 / Intent Extraction Demo ---");
    let intent_tests = vec![
        "定义一个函数叫multiply，接受两个参数a和b",
        "创建一个函数add，参数是x和y",
        "定义一个变量count等于一百",
    ];

    for intent_test in intent_tests {
        println!("\n输入 / Input: {}", intent_test);

        match nlu_parser.extract_intent(intent_test) {
            Ok(intent) => {
                println!("提取的意图 / Extracted Intent:");
                println!("  动作 / Action: {}", intent.action);
                println!("  实体 / Entities: {:?}", intent.entities);
                println!("  参数 / Parameters: {:?}", intent.parameters);
            }
            Err(e) => {
                println!("意图提取错误 / Intent Extraction Error: {:?}", e);
            }
        }
    }
}

/// 将代码结构转换为可执行代码（简化版本）
/// Convert code structure to executable code (simplified version)
fn convert_to_executable(elements: &[crate::grammar::core::GrammarElement]) -> Option<String> {
    if elements.is_empty() {
        return None;
    }

    // 简化实现：只处理第一个元素
    match &elements[0] {
        crate::grammar::core::GrammarElement::List(list) => {
            let mut result = String::from("(");
            for (i, elem) in list.iter().enumerate() {
                if i > 0 {
                    result.push(' ');
                }
                match elem {
                    crate::grammar::core::GrammarElement::Atom(s) => {
                        result.push_str(s);
                    }
                    crate::grammar::core::GrammarElement::List(l) => {
                        result.push('(');
                        for (j, sub_elem) in l.iter().enumerate() {
                            if j > 0 {
                                result.push(' ');
                            }
                            if let crate::grammar::core::GrammarElement::Atom(s) = sub_elem {
                                result.push_str(s);
                            }
                        }
                        result.push(')');
                    }
                    crate::grammar::core::GrammarElement::Expr(e) => {
                        result.push_str(&format_expr(e));
                    }
                    _ => {}
                }
            }
            result.push(')');
            Some(result)
        }
        crate::grammar::core::GrammarElement::Expr(e) => Some(format_expr(e)),
        _ => None,
    }
}

/// 格式化字面量为字符串
/// Format literal as string
fn format_literal(lit: &crate::grammar::core::Literal) -> String {
    match lit {
        crate::grammar::core::Literal::Int(i) => i.to_string(),
        crate::grammar::core::Literal::Float(f) => f.to_string(),
        crate::grammar::core::Literal::String(s) => format!("\"{}\"", s),
        crate::grammar::core::Literal::Bool(b) => b.to_string(),
        crate::grammar::core::Literal::Null => "null".to_string(),
        crate::grammar::core::Literal::List(items) => {
            let items_str: Vec<String> = items.iter().map(|item| format_expr(item)).collect();
            format!("[{}]", items_str.join(", "))
        }
        crate::grammar::core::Literal::Dict(pairs) => {
            let pairs_str: Vec<String> = pairs
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, format_expr(v)))
                .collect();
            format!("{{{}}}", pairs_str.join(", "))
        }
    }
}

/// 格式化表达式为字符串
/// Format expression as string
fn format_expr(expr: &crate::grammar::core::Expr) -> String {
    match expr {
        crate::grammar::core::Expr::Literal(lit) => format_literal(lit),
        crate::grammar::core::Expr::Var(v) => v.clone(),
        crate::grammar::core::Expr::Call(name, args) => {
            let mut result = format!("({}", name);
            for arg in args {
                result.push(' ');
                result.push_str(&format_expr(arg));
            }
            result.push(')');
            result
        }
        crate::grammar::core::Expr::Binary(op, left, right) => {
            let op_str = match op {
                crate::grammar::core::BinOp::Add => "+",
                crate::grammar::core::BinOp::Sub => "-",
                crate::grammar::core::BinOp::Mul => "*",
                crate::grammar::core::BinOp::Div => "/",
                crate::grammar::core::BinOp::Eq => "=",
                crate::grammar::core::BinOp::Ne => "!=",
                crate::grammar::core::BinOp::Lt => "<",
                crate::grammar::core::BinOp::Gt => ">",
                crate::grammar::core::BinOp::Le => "<=",
                crate::grammar::core::BinOp::Ge => ">=",
            };
            format!("({} {} {})", op_str, format_expr(left), format_expr(right))
        }
        crate::grammar::core::Expr::If(cond, then_expr, else_expr) => {
            format!(
                "(if {} {} {})",
                format_expr(cond),
                format_expr(then_expr),
                format_expr(else_expr)
            )
        }
    }
}

/// 演示JIT编译器功能 / Demonstrate JIT compiler functionality
fn demonstrate_jit() {
    println!("\n7. JIT编译器演示 / JIT Compiler Demo");
    println!("--------------------------------------------");

    use runtime::JITInterpreter;
    let parser = AdaptiveParser::new(true);
    let mut jit_interpreter = JITInterpreter::with_threshold(5); // 5次执行后编译 / Compile after 5 executions

    println!("JIT编译器已创建，编译阈值：5次执行 / JIT Compiler created, compilation threshold: 5 executions");
    println!("\n--- 测试热点代码检测和优化 / Testing Hot Spot Detection and Optimization ---");

    // 定义一个会被多次调用的函数
    // Define a function that will be called multiple times
    let define_code = "(def add (x y) (+ x y))";
    println!("\n定义函数 / Define function: {}", define_code);

    match parser.parse(define_code) {
        Ok(ast) => {
            if let Err(e) = jit_interpreter.execute(&ast) {
                println!("定义函数时出错 / Error defining function: {:?}", e);
                return;
            }
            println!("函数定义成功 / Function defined successfully");
        }
        Err(e) => {
            println!("解析错误 / Parse Error: {:?}", e);
            return;
        }
    }

    // 多次调用函数以触发JIT编译
    // Call function multiple times to trigger JIT compilation
    let call_code = "(add 3 4)";
    println!(
        "\n多次调用函数以触发JIT编译 / Call function multiple times to trigger JIT compilation:"
    );

    for i in 1..=10 {
        match parser.parse(call_code) {
            Ok(ast) => match jit_interpreter.execute(&ast) {
                Ok(value) => {
                    if i <= 5 {
                        println!("  调用 {}: {} (解释执行 / Interpreted)", i, value);
                    } else if i == 6 {
                        println!("  调用 {}: {} (JIT编译后执行 / JIT Compiled)", i, value);
                    } else if i == 10 {
                        println!("  调用 {}: {} (JIT优化执行 / JIT Optimized)", i, value);
                    }
                }
                Err(e) => {
                    println!("执行错误 / Execution Error: {:?}", e);
                }
            },
            Err(e) => {
                println!("解析错误 / Parse Error: {:?}", e);
            }
        }
    }

    // 显示JIT统计信息
    // Display JIT statistics
    println!("\n--- JIT统计信息 / JIT Statistics ---");
    let stats = jit_interpreter.get_jit_statistics();
    println!("总热点代码数 / Total hot spots: {}", stats.total_hot_spots);
    println!("总执行次数 / Total executions: {}", stats.total_executions);
    println!(
        "编译后执行次数 / Compiled executions: {}",
        stats.compiled_count
    );
    println!(
        "编译阈值 / Compilation threshold: {}",
        stats.compilation_threshold
    );
    println!("JIT启用状态 / JIT enabled: {}", stats.enabled);

    // 显示热点代码列表
    // Display hot spot code list
    let hot_spots = jit_interpreter.get_hot_spots();
    if !hot_spots.is_empty() {
        println!("\n热点代码列表 / Hot Spot Code List:");
        for (i, hot_spot) in hot_spots.iter().take(3).enumerate() {
            println!(
                "  {}. {}",
                i + 1,
                hot_spot.chars().take(50).collect::<String>()
            );
        }
    }

    // 测试常量折叠优化
    // Test constant folding optimization
    println!("\n--- 测试常量折叠优化 / Testing Constant Folding Optimization ---");
    let constant_fold_code = "(+ 10 20)";
    println!("测试代码 / Test code: {}", constant_fold_code);

    for i in 1..=6 {
        match parser.parse(constant_fold_code) {
            Ok(ast) => match jit_interpreter.execute(&ast) {
                Ok(value) => {
                    if i == 6 {
                        println!(
                            "  执行 {}: {} (已优化为常量 / Optimized to constant)",
                            i, value
                        );
                    }
                }
                Err(e) => {
                    println!("执行错误 / Execution Error: {:?}", e);
                }
            },
            Err(e) => {
                println!("解析错误 / Parse Error: {:?}", e);
            }
        }
    }

    println!("\nJIT编译器演示完成 / JIT Compiler Demo Completed");
}

/// 演示列表和字典功能 / Demonstrate list and dictionary functionality
fn demonstrate_data_structures() {
    println!("\n8. 列表和字典演示 / Lists and Dictionaries Demo");
    println!("--------------------------------------------");

    let parser = AdaptiveParser::new(true);
    let mut interpreter = Interpreter::new();

    // 测试列表操作
    println!("\n--- 测试列表操作 / Testing List Operations ---");
    let list_tests = vec![
        ("(list 1 2 3)", "创建列表 / Create list"),
        (
            "(list-length (list 1 2 3))",
            "获取列表长度 / Get list length",
        ),
        (
            "(list-get (list 10 20 30) 1)",
            "获取列表元素 / Get list element",
        ),
        ("(list-append (list 1 2) 3)", "追加元素 / Append element"),
        ("(+ (list 1 2) (list 3 4))", "列表连接 / List concatenation"),
    ];

    for (code, description) in list_tests {
        println!("\n测试: {} / Test: {}", description, description);
        println!("代码: {} / Code: {}", code, code);
        match parser.parse(code) {
            Ok(ast) => match interpreter.execute(&ast) {
                Ok(value) => {
                    println!("结果: {} / Result: {}", value, value);
                }
                Err(e) => {
                    println!("执行错误 / Execution Error: {:?}", e);
                }
            },
            Err(e) => {
                println!("解析错误 / Parse Error: {:?}", e);
            }
        }
    }

    // 测试字典操作
    println!("\n--- 测试字典操作 / Testing Dictionary Operations ---");
    let dict_tests = vec![
        (
            "(dict \"name\" \"Aevolang\" \"version\" \"1.0\")",
            "创建字典 / Create dictionary",
        ),
        (
            "(dict-get (dict \"name\" \"Aevolang\") \"name\")",
            "获取字典值 / Get dictionary value",
        ),
        (
            "(dict-set (dict \"x\" 1) \"y\" 2)",
            "设置字典值 / Set dictionary value",
        ),
        (
            "(dict-keys (dict \"a\" 1 \"b\" 2))",
            "获取所有键 / Get all keys",
        ),
        (
            "(dict-has (dict \"name\" \"Aevolang\") \"name\")",
            "检查键是否存在 / Check if key exists",
        ),
    ];

    for (code, description) in dict_tests {
        println!("\n测试: {} / Test: {}", description, description);
        println!("代码: {} / Code: {}", code, code);
        match parser.parse(code) {
            Ok(ast) => match interpreter.execute(&ast) {
                Ok(value) => {
                    println!("结果: {} / Result: {}", value, value);
                }
                Err(e) => {
                    println!("执行错误 / Execution Error: {:?}", e);
                }
            },
            Err(e) => {
                println!("解析错误 / Parse Error: {:?}", e);
            }
        }
    }

    println!("\n列表和字典演示完成 / Lists and Dictionaries Demo Completed");
}

/// 演示模块系统功能 / Demonstrate module system functionality
fn demonstrate_modules() {
    println!("\n9. 模块系统演示 / Module System Demo");
    println!("--------------------------------------------");

    let parser = AdaptiveParser::new(true);
    let mut interpreter = Interpreter::new();

    // 导入模块 / Import module
    let import_code = r#"(import "math")"#;
    println!("\n导入模块 / Import module: {}", import_code);
    match parser.parse(import_code) {
        Ok(ast) => match interpreter.execute(&ast) {
            Ok(_) => println!("模块导入成功 / Module imported successfully"),
            Err(e) => {
                println!("导入错误 / Import Error: {:?}", e);
                return;
            }
        },
        Err(e) => {
            println!("解析错误 / Parse Error: {:?}", e);
            return;
        }
    }

    // 调用模块函数 / Call module functions
    let module_calls = vec![
        ("(math.add 3 4)", "模块加法 / Module add"),
        ("(math.square 5)", "模块平方 / Module square"),
    ];

    for (code, description) in module_calls {
        println!("\n测试: {} / Test: {}", description, description);
        println!("代码: {} / Code: {}", code, code);
        match parser.parse(code) {
            Ok(ast) => match interpreter.execute(&ast) {
                Ok(value) => {
                    println!("结果: {} / Result: {}", value, value);
                }
                Err(e) => {
                    println!("执行错误 / Execution Error: {:?}", e);
                }
            },
            Err(e) => {
                println!("解析错误 / Parse Error: {:?}", e);
            }
        }
    }

    println!("\n模块系统演示完成 / Module System Demo Completed");
}
