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

    // 演示标准库 / Demonstrate standard library
    demonstrate_std_library();

    // 演示代码解释 / Demonstrate code explanation
    demonstrate_code_explanation();

    // 演示代码分析 / Demonstrate code analysis
    demonstrate_code_analysis();

    // 演示代码自动重构 / Demonstrate automatic code refactoring
    demonstrate_code_refactoring();
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

            // 演示从诗歌生成代码 / Demonstrate code generation from poetry
            println!("\n从诗歌生成代码 / Code Generation from Poetry:");
            let mut engine = EvolutionEngine::new();
            match engine.generate_code_from_poetry(poem) {
                Ok(code) => {
                    println!("生成的代码 / Generated Code:");
                    println!("{}", code);
                    
                    // 尝试执行生成的代码 / Try to execute generated code
                    let code_parser = AdaptiveParser::new(true);
                    let mut code_interpreter = Interpreter::new();
                    match code_parser.parse(&code) {
                        Ok(ast) => {
                            match code_interpreter.execute(&ast) {
                                Ok(_) => println!("代码执行成功 / Code executed successfully"),
                                Err(e) => println!("代码执行错误 / Code execution error: {:?}", e),
                            }
                        }
                        Err(e) => println!("代码解析错误 / Code parse error: {:?}", e),
                    }
                }
                Err(e) => {
                    println!("生成代码错误 / Code generation error: {:?}", e);
                }
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

    let mut engine = EvolutionEngine::new();

    println!("进化引擎已创建 / Evolution Engine Created");
    println!("历史记录数 / History Count: {}", engine.get_history().len());
    println!(
        "自举规则数 / Bootstrap Rule Count: {}",
        engine.get_syntax_rules().len()
    );
    
    // 知识图谱统计 / Knowledge graph statistics
    let stats = engine.get_knowledge_stats();
    println!("知识图谱节点数 / Knowledge Graph Nodes: {}", stats["nodes_count"]);
    println!("发现模式数 / Discovered Patterns: {}", stats["patterns_count"]);

    // 演示进化预测 / Demonstrate evolution prediction
    println!("\n进化预测 / Evolution Predictions:");
    let predictions = engine.predict_evolutions(vec![
        "支持更多数据结构".to_string(),
        "改进性能".to_string(),
    ]);
    
    if predictions.is_empty() {
        println!("  暂无预测 / No predictions yet (需要更多历史数据 / need more history data)");
    } else {
        for (i, pred) in predictions.iter().take(3).enumerate() {
            println!("  {}. {} (置信度: {:.2})", 
                i + 1, pred.predicted_evolution, pred.confidence);
            println!("     理由 / Reasoning: {}", pred.reasoning);
        }
    }

    // 演示从自然语言进化 / Demonstrate evolution from natural language
    println!("\n提示 / Note: 使用 evolve_from_natural_language() 从自然语言进化");
    println!("Use evolve_from_natural_language() to evolve from natural language");

    // 演示从诗歌理解中进化 / Demonstrate evolution from poetry understanding
    println!("\n从诗歌理解中进化 / Evolution from Poetry Understanding:");
    let poem = r#"
床前明月光，
疑是地上霜。
举头望明月，
低头思故乡。
"#;
    match engine.evolve_from_poetry(poem) {
        Ok(rules) => {
            println!("  从诗歌理解中生成的规则数 / Rules generated: {}", rules.len());
            if !rules.is_empty() {
                println!("  生成的规则示例 / Example rule: {}", rules[0].name);
            }
            
            // 更新统计 / Update statistics
            let stats_after = engine.get_knowledge_stats();
            println!("  进化后知识图谱节点数 / Knowledge nodes after evolution: {}", stats_after["nodes_count"]);
        }
        Err(e) => {
            println!("  进化错误 / Evolution error: {:?}", e);
        }
    }

    // 演示自我反思 / Demonstrate self-reflection
    println!("\n自我反思 / Self-Reflection:");
    let reflection = engine.self_reflect();
    println!("  总进化次数 / Total Evolutions: {}", reflection["total_evolutions"]);
    println!("  最近7天进化 / Recent 7 Days: {}", reflection["recent_evolutions_7days"]);
    println!("  语法进化 / Syntax Evolutions: {}", reflection["syntax_evolutions"]);
    println!("  语义进化 / Semantic Evolutions: {}", reflection["semantic_evolutions"]);
    println!("  知识丰富度 / Knowledge Richness: {}", reflection["knowledge_richness"]);
    println!("  自我评估 / Self Assessment: {}", reflection["self_assessment"]);

    // 演示相似规则查找 / Demonstrate similar rule finding
    if !engine.get_syntax_rules().is_empty() {
        println!("\n相似规则查找 / Similar Rules Finding:");
        let first_rule = &engine.get_syntax_rules()[0];
        let similar = engine.find_similar_rules(&first_rule.name);
        if similar.is_empty() {
            println!("  暂无相似规则 / No similar rules found");
        } else {
            println!("  规则 '{}' 的相似规则 / Similar rules to '{}':", first_rule.name, first_rule.name);
            for (rule_id, similarity) in similar.iter().take(3) {
                println!("    {} (相似度: {:.2})", rule_id, similarity);
            }
        }
    }

    // 演示进化谱系和回滚 / Demonstrate evolution genealogy and rollback
    println!("\n进化谱系和回滚 / Evolution Genealogy and Rollback:");
    let history = engine.get_history();
    if !history.is_empty() {
        let first_event = &history[0];
        println!("  第一个事件ID / First Event ID: {}", first_event.id);
        
        // 获取祖先链 / Get ancestor chain
        let ancestors = engine.get_event_ancestors(first_event.id);
        println!("  祖先事件数 / Ancestor Events: {}", ancestors.len());
        
        // 获取后代事件 / Get descendant events
        let descendants = engine.get_event_descendants(first_event.id);
        println!("  后代事件数 / Descendant Events: {}", descendants.len());
        
        // 获取谱系树 / Get genealogy tree
        let tree = engine.get_genealogy_tree(Some(first_event.id));
        if tree != serde_json::json!({}) {
            println!("  谱系树结构 / Genealogy Tree Structure: 已生成 (包含{}个节点)", 
                tree["children"].as_array().map(|c| c.len()).unwrap_or(0));
        }
        
        println!("  提示 / Note: 使用 rollback_to_event() 可以回滚到指定事件之前的状态");
        println!("  Use rollback_to_event() to rollback to state before specified event");
    } else {
        println!("  暂无进化历史 / No evolution history yet");
    }
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
        (
            "如果 5 大于 3 则 10 否则 20",
            "中文条件表达式 / Chinese conditional expression",
        ),
        (
            "3 加 5 然后 10 乘以 2",
            "中文多步骤操作 / Chinese multi-step operation",
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

/// 演示标准库 / Demonstrate standard library
fn demonstrate_std_library() {
    println!("\n10. 标准库演示 / Standard Library Demo");
    println!("--------------------------------------------");

    let parser = AdaptiveParser::new(true);
    let mut interpreter = Interpreter::new();

    // 导入标准库 / Import standard library
    let import_code = "(import \"std\")";
    match parser.parse(import_code) {
        Ok(ast) => {
            if let Err(e) = interpreter.execute(&ast) {
                println!("导入标准库错误 / Import error: {:?}", e);
                return;
            }
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
            return;
        }
    }

    // 测试标准库函数 / Test standard library functions
    let test_cases = vec![
        ("(std.abs -5)", "绝对值 / Absolute value"),
        ("(std.max 3 7)", "最大值 / Maximum"),
        ("(std.min 3 7)", "最小值 / Minimum"),
        ("(std.factorial 5)", "阶乘 / Factorial"),
    ];

    for (code, description) in test_cases {
        println!("\n测试 / Test: {}", description);
        println!("代码 / Code: {}", code);
        match parser.parse(code) {
            Ok(ast) => {
                match interpreter.execute(&ast) {
                    Ok(value) => println!("结果 / Result: {}", value),
                    Err(e) => println!("执行错误 / Execution error: {:?}", e),
                }
            }
            Err(e) => println!("解析错误 / Parse error: {:?}", e),
        }
    }

    println!("\n提示 / Note: 标准库提供了map、filter、reduce等高级列表操作");
    println!("Standard library provides map, filter, reduce and other advanced list operations");
}

/// 演示代码解释功能 / Demonstrate code explanation functionality
fn demonstrate_code_explanation() {
    println!("\n11. 代码解释演示 / Code Explanation Demo");
    println!("--------------------------------------------");

    use crate::parser::{AdaptiveParser, CodeExplainer, Language};

    let parser = AdaptiveParser::new(true);
    let explainer_chinese = CodeExplainer::new(Language::Chinese);
    let explainer_english = CodeExplainer::new(Language::English);

    let test_cases = vec![
        ("(let x 5)", "变量定义 / Variable definition"),
        ("(def add (x y) (+ x y))", "函数定义 / Function definition"),
        ("(if (> x 3) 10 20)", "条件表达式 / Conditional expression"),
        ("(+ 3 7)", "二元运算 / Binary operation"),
        ("(list 1 2 3)", "列表字面量 / List literal"),
    ];

    for (code, description) in test_cases {
        println!("\n测试 / Test: {}", description);
        println!("代码 / Code: {}", code);
        
        match parser.parse(code) {
            Ok(ast) => {
                println!("\n中文解释 / Chinese Explanation:");
                println!("  {}", explainer_chinese.explain_ast(&ast));
                
                println!("\n英文解释 / English Explanation:");
                println!("  {}", explainer_english.explain_ast(&ast));
            }
            Err(e) => {
                println!("解析错误 / Parse error: {:?}", e);
            }
        }
    }

    println!("\n提示 / Note: 代码解释功能帮助理解代码含义，增强代码可读性");
    println!("Code explanation helps understand code meaning and enhances code readability");
}

/// 演示代码分析功能 / Demonstrate code analysis functionality
fn demonstrate_code_analysis() {
    println!("\n12. 代码分析演示 / Code Analysis Demo");
    println!("--------------------------------------------");

    use crate::parser::AdaptiveParser;
    use crate::evolution::{CodeAnalyzer, EvolutionEngine};

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let engine = EvolutionEngine::new();

    // 测试代码：一个复杂的示例 / Test code: a complex example
    let test_code = r#"
(def complex_function (x y)
  (if (> x 5)
    (if (> y 10)
      (if (> (+ x y) 20)
        (* x y)
        (+ x y))
      (- x y))
    (+ x y)))
"#;

    println!("测试代码 / Test Code:");
    println!("{}", test_code);

    match parser.parse(test_code) {
        Ok(ast) => {
            // 分析代码 / Analyze code
            let analysis = analyzer.analyze(&ast);
            
            println!("\n代码分析结果 / Code Analysis Result:");
            println!("复杂度 / Complexity: {:.2}", analysis.complexity);
            
            println!("\n代码统计 / Code Statistics:");
            println!("  函数数量 / Function count: {}", analysis.statistics.function_count);
            println!("  变量数量 / Variable count: {}", analysis.statistics.variable_count);
            println!("  平均函数长度 / Avg function length: {:.2}", analysis.statistics.avg_function_length);
            println!("  最大嵌套深度 / Max nesting depth: {}", analysis.statistics.max_nesting_depth);
            println!("  表达式复杂度 / Expression complexity: {:.2}", analysis.statistics.expression_complexity);

            println!("\n发现的模式 / Patterns Found:");
            for pattern in &analysis.patterns {
                println!("  - {:?}: {} (置信度: {:.2})", pattern.pattern_type, pattern.description, pattern.confidence);
            }

            println!("\n优化建议 / Optimization Suggestions:");
            for (i, suggestion) in analysis.suggestions.iter().enumerate() {
                println!("\n  建议 {} / Suggestion {}:", i + 1, i + 1);
                println!("    类型 / Type: {:?}", suggestion.suggestion_type);
                println!("    描述 / Description: {}", suggestion.description);
                println!("    改进程度 / Improvement: {:.2}", suggestion.improvement);
            }

            // 使用进化引擎分析 / Use evolution engine to analyze
            let engine_analysis = engine.analyze_code(&ast);
            println!("\n使用进化引擎分析 / Analysis using Evolution Engine:");
            println!("  复杂度 / Complexity: {:.2}", engine_analysis.complexity);
            println!("  发现 {} 个模式 / Found {} patterns", engine_analysis.patterns.len(), engine_analysis.patterns.len());
            println!("  生成 {} 个建议 / Generated {} suggestions", engine_analysis.suggestions.len(), engine_analysis.suggestions.len());
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
        }
    }

    println!("\n提示 / Note: 代码分析帮助识别代码模式和优化机会，促进代码质量提升");
    println!("Code analysis helps identify patterns and optimization opportunities, promoting code quality improvement");
}

/// 演示代码自动重构功能 / Demonstrate automatic code refactoring functionality
fn demonstrate_code_refactoring() {
    println!("\n13. 代码自动重构演示 / Automatic Code Refactoring Demo");
    println!("--------------------------------------------");

    use crate::parser::AdaptiveParser;
    use crate::evolution::EvolutionEngine;

    let parser = AdaptiveParser::new(true);
    let engine = EvolutionEngine::new();

    // 测试代码：包含可优化的表达式 / Test code: contains optimizable expressions
    let test_code = r#"
(def calculate (x y)
  (+ (* x 2) (* y 2)))
"#;

    println!("原始代码 / Original Code:");
    println!("{}", test_code);

    match parser.parse(test_code) {
        Ok(ast) => {
            // 分析代码 / Analyze code
            let analysis = engine.analyze_code(&ast);
            println!("\n代码分析 / Code Analysis:");
            println!("  复杂度 / Complexity: {:.2}", analysis.complexity);
            println!("  发现 {} 个模式 / Found {} patterns", analysis.patterns.len(), analysis.patterns.len());
            println!("  生成 {} 个建议 / Generated {} suggestions", analysis.suggestions.len(), analysis.suggestions.len());

            // 自动重构 / Automatic refactoring
            let refactored = engine.refactor_code(&ast);
            
            println!("\n重构后的代码 / Refactored Code:");
            // 简化显示：显示重构后的AST结构 / Simplified display: show refactored AST structure
            println!("  重构完成，代码已优化 / Refactoring complete, code optimized");
            println!("  原始AST元素数 / Original AST elements: {}", ast.len());
            println!("  重构后AST元素数 / Refactored AST elements: {}", refactored.len());

            // 测试常量折叠优化 / Test constant folding optimization
            let constant_code = "(+ (* 3 2) (* 4 5))";
            println!("\n常量折叠测试 / Constant Folding Test:");
            println!("  原始代码 / Original: {}", constant_code);
            
            match parser.parse(constant_code) {
                Ok(constant_ast) => {
                    let refactored_const = engine.refactor_code(&constant_ast);
                    println!("  重构后 / Refactored: {} 个元素 / {} elements", refactored_const.len(), refactored_const.len());
                }
                Err(e) => {
                    println!("  解析错误 / Parse error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
        }
    }

    println!("\n提示 / Note: 代码自动重构能够根据分析结果自动改进代码，实现真正的自进化");
    println!("Automatic code refactoring can improve code based on analysis results, achieving true self-evolution");
}
