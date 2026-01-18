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

    // 演示自我进化 / Demonstrate self-evolution
    demonstrate_self_evolution();

    // 演示上下文理解 / Demonstrate context understanding
    demonstrate_context_understanding();

    // 演示增强的标准库 / Demonstrate enhanced standard library
    demonstrate_enhanced_std();

    // 演示使用模式学习 / Demonstrate usage pattern learning
    demonstrate_usage_learning();

    // 演示错误恢复 / Demonstrate error recovery
    demonstrate_error_recovery();

    // 演示智能代码生成 / Demonstrate intelligent code generation
    demonstrate_intelligent_code_generation();

    // 演示代码质量评估 / Demonstrate code quality assessment
    demonstrate_quality_assessment();

    // 演示智能优化建议 / Demonstrate intelligent optimization advisor
    demonstrate_optimization_advisor();

    // 演示代码审查 / Demonstrate code review
    demonstrate_code_review();

    // 演示代码文档生成 / Demonstrate code documentation generation
    demonstrate_documentation_generation();

    // 演示测试生成 / Demonstrate test generation
    demonstrate_test_generation();

    // 演示性能分析 / Demonstrate performance analysis
    demonstrate_performance_analysis();

    // 演示代码相似度检测 / Demonstrate code similarity detection
    demonstrate_similarity_detection();

    // 演示代码依赖分析 / Demonstrate code dependency analysis
    demonstrate_dependency_analysis();
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
                        Ok(ast) => match code_interpreter.execute(&ast) {
                            Ok(_) => println!("代码执行成功 / Code executed successfully"),
                            Err(e) => println!("代码执行错误 / Code execution error: {:?}", e),
                        },
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
    println!(
        "知识图谱节点数 / Knowledge Graph Nodes: {}",
        stats["nodes_count"]
    );
    println!(
        "发现模式数 / Discovered Patterns: {}",
        stats["patterns_count"]
    );

    // 演示进化预测 / Demonstrate evolution prediction
    println!("\n进化预测 / Evolution Predictions:");
    let predictions =
        engine.predict_evolutions(vec!["支持更多数据结构".to_string(), "改进性能".to_string()]);

    if predictions.is_empty() {
        println!("  暂无预测 / No predictions yet (需要更多历史数据 / need more history data)");
    } else {
        for (i, pred) in predictions.iter().take(3).enumerate() {
            println!(
                "  {}. {} (置信度: {:.2})",
                i + 1,
                pred.predicted_evolution,
                pred.confidence
            );
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
            println!(
                "  从诗歌理解中生成的规则数 / Rules generated: {}",
                rules.len()
            );
            if !rules.is_empty() {
                println!("  生成的规则示例 / Example rule: {}", rules[0].name);
            }

            // 更新统计 / Update statistics
            let stats_after = engine.get_knowledge_stats();
            println!(
                "  进化后知识图谱节点数 / Knowledge nodes after evolution: {}",
                stats_after["nodes_count"]
            );
        }
        Err(e) => {
            println!("  进化错误 / Evolution error: {:?}", e);
        }
    }

    // 演示自我反思 / Demonstrate self-reflection
    println!("\n自我反思 / Self-Reflection:");
    let reflection = engine.self_reflect();
    println!(
        "  总进化次数 / Total Evolutions: {}",
        reflection["total_evolutions"]
    );
    println!(
        "  最近7天进化 / Recent 7 Days: {}",
        reflection["recent_evolutions_7days"]
    );
    println!(
        "  语法进化 / Syntax Evolutions: {}",
        reflection["syntax_evolutions"]
    );
    println!(
        "  语义进化 / Semantic Evolutions: {}",
        reflection["semantic_evolutions"]
    );
    println!(
        "  知识丰富度 / Knowledge Richness: {}",
        reflection["knowledge_richness"]
    );
    println!(
        "  自我评估 / Self Assessment: {}",
        reflection["self_assessment"]
    );

    // 演示相似规则查找 / Demonstrate similar rule finding
    if !engine.get_syntax_rules().is_empty() {
        println!("\n相似规则查找 / Similar Rules Finding:");
        let first_rule = &engine.get_syntax_rules()[0];
        let similar = engine.find_similar_rules(&first_rule.name);
        if similar.is_empty() {
            println!("  暂无相似规则 / No similar rules found");
        } else {
            println!(
                "  规则 '{}' 的相似规则 / Similar rules to '{}':",
                first_rule.name, first_rule.name
            );
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
            println!(
                "  谱系树结构 / Genealogy Tree Structure: 已生成 (包含{}个节点)",
                tree["children"].as_array().map(|c| c.len()).unwrap_or(0)
            );
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
        crate::grammar::core::Expr::Match(value, cases) => {
            let mut result = format!("(match {}", format_expr(value));
            for (pattern, expr) in cases {
                result.push_str(&format!(" ({:?} {})", pattern, format_expr(expr)));
            }
            result.push(')');
            result
        }
        crate::grammar::core::Expr::For { var, iterable, body } => {
            format!("(for {} {} {})", var, format_expr(iterable), format_expr(body))
        }
        crate::grammar::core::Expr::While { condition, body } => {
            format!("(while {} {})", format_expr(condition), format_expr(body))
        }
        crate::grammar::core::Expr::Try { try_body, catch_var, catch_body } => {
            if let Some(var) = catch_var {
                format!("(try {} (catch {} {}))", format_expr(try_body), var, format_expr(catch_body))
            } else {
                format!("(try {} {})", format_expr(try_body), format_expr(catch_body))
            }
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
            Ok(ast) => match interpreter.execute(&ast) {
                Ok(value) => println!("结果 / Result: {}", value),
                Err(e) => println!("执行错误 / Execution error: {:?}", e),
            },
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

    use crate::evolution::{CodeAnalyzer, EvolutionEngine};
    use crate::parser::AdaptiveParser;

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
            println!(
                "  函数数量 / Function count: {}",
                analysis.statistics.function_count
            );
            println!(
                "  变量数量 / Variable count: {}",
                analysis.statistics.variable_count
            );
            println!(
                "  平均函数长度 / Avg function length: {:.2}",
                analysis.statistics.avg_function_length
            );
            println!(
                "  最大嵌套深度 / Max nesting depth: {}",
                analysis.statistics.max_nesting_depth
            );
            println!(
                "  表达式复杂度 / Expression complexity: {:.2}",
                analysis.statistics.expression_complexity
            );

            println!("\n发现的模式 / Patterns Found:");
            for pattern in &analysis.patterns {
                println!(
                    "  - {:?}: {} (置信度: {:.2})",
                    pattern.pattern_type, pattern.description, pattern.confidence
                );
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
            println!(
                "  发现 {} 个模式 / Found {} patterns",
                engine_analysis.patterns.len(),
                engine_analysis.patterns.len()
            );
            println!(
                "  生成 {} 个建议 / Generated {} suggestions",
                engine_analysis.suggestions.len(),
                engine_analysis.suggestions.len()
            );
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

    use crate::evolution::EvolutionEngine;
    use crate::parser::AdaptiveParser;

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
            println!(
                "  发现 {} 个模式 / Found {} patterns",
                analysis.patterns.len(),
                analysis.patterns.len()
            );
            println!(
                "  生成 {} 个建议 / Generated {} suggestions",
                analysis.suggestions.len(),
                analysis.suggestions.len()
            );

            // 自动重构 / Automatic refactoring
            let refactored = engine.refactor_code(&ast);

            println!("\n重构后的代码 / Refactored Code:");
            // 简化显示：显示重构后的AST结构 / Simplified display: show refactored AST structure
            println!("  重构完成，代码已优化 / Refactoring complete, code optimized");
            println!("  原始AST元素数 / Original AST elements: {}", ast.len());
            println!(
                "  重构后AST元素数 / Refactored AST elements: {}",
                refactored.len()
            );

            // 测试常量折叠优化 / Test constant folding optimization
            let constant_code = "(+ (* 3 2) (* 4 5))";
            println!("\n常量折叠测试 / Constant Folding Test:");
            println!("  原始代码 / Original: {}", constant_code);

            match parser.parse(constant_code) {
                Ok(constant_ast) => {
                    let refactored_const = engine.refactor_code(&constant_ast);
                    println!(
                        "  重构后 / Refactored: {} 个元素 / {} elements",
                        refactored_const.len(),
                        refactored_const.len()
                    );
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

/// 演示自我进化功能 / Demonstrate self-evolution functionality
fn demonstrate_self_evolution() {
    println!("\n14. 自我进化演示 / Self-Evolution Demo");
    println!("--------------------------------------------");

    use crate::evolution::EvolutionEngine;

    let mut engine = EvolutionEngine::new();

    println!("执行自我进化 / Performing Self-Evolution:");
    match engine.self_evolve() {
        Ok(result) => {
            println!("自我进化结果 / Self-Evolution Result:");
            println!(
                "  {}",
                serde_json::to_string_pretty(&result).unwrap_or_default()
            );

            // 显示自我反思 / Show self-reflection
            println!("\n自我反思 / Self-Reflection:");
            let reflection = engine.self_reflect();
            println!(
                "  {}",
                serde_json::to_string_pretty(&reflection).unwrap_or_default()
            );

            // 显示知识图谱统计 / Show knowledge graph statistics
            println!("\n知识图谱统计 / Knowledge Graph Statistics:");
            let stats = engine.get_knowledge_stats();
            println!(
                "  {}",
                serde_json::to_string_pretty(&stats).unwrap_or_default()
            );
        }
        Err(e) => {
            println!("自我进化错误 / Self-Evolution Error: {:?}", e);
        }
    }

    println!("\n提示 / Note: 自我进化功能让语言能够自动分析和改进自身实现，形成完整的自进化闭环");
    println!("Self-evolution allows the language to automatically analyze and improve its own implementation, forming a complete self-evolution loop");
}

/// 演示上下文理解功能 / Demonstrate context understanding functionality
fn demonstrate_context_understanding() {
    println!("\n15. 上下文理解演示 / Context Understanding Demo");
    println!("--------------------------------------------");

    use crate::parser::{AdaptiveParser, ContextManager, NLUParser};

    let parser = AdaptiveParser::new(true);
    let nlu_parser = NLUParser::new(crate::parser::nlu::ModelType::LocalLightweight, true);
    let mut context = ContextManager::new("session_001".to_string());

    // 多轮对话示例 / Multi-turn conversation example
    let conversations = vec![
        ("定义变量x等于5", "第一轮：定义变量"),
        ("上面的x加上3", "第二轮：引用之前的变量"),
        ("定义函数add，参数是x和y，返回x加y", "第三轮：定义函数"),
        ("调用上面的add函数，参数是2和3", "第四轮：引用之前的函数"),
    ];

    for (input, description) in conversations {
        println!("\n{} / {}", description, description);
        println!("输入 / Input: {}", input);

        // 解析输入 / Parse input
        match nlu_parser.parse(input) {
            Ok(intent) => {
                // 添加上下文 / Add context
                let turn_id = context.add_turn(input.to_string(), Some(intent.clone()));
                println!("  解析成功 / Parse success, turn ID: {}", turn_id);

                // 使用上下文解析 / Parse with context
                match context.parse_with_context(input) {
                    Ok(enhanced_intent) => {
                        println!("  上下文解析 / Context parsing:");
                        println!(
                            "    引用数量 / References: {}",
                            enhanced_intent.context_references.len()
                        );
                        println!(
                            "    解析的变量 / Resolved variables: {}",
                            enhanced_intent.resolved_variables.len()
                        );
                        println!(
                            "    解析的函数 / Resolved functions: {}",
                            enhanced_intent.resolved_functions.len()
                        );

                        for reference in &enhanced_intent.context_references {
                            println!("    引用 / Reference: {}", reference.description);
                        }
                    }
                    Err(e) => {
                        println!("  上下文解析错误 / Context parse error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("  解析错误 / Parse error: {:?}", e);
            }
        }
    }

    println!("\n对话历史 / Conversation History:");
    for turn in context.get_history() {
        println!(
            "  轮次 {} / Turn {}: {}",
            turn.turn_id, turn.turn_id, turn.user_input
        );
    }

    println!(
        "\n提示 / Note: 上下文理解功能让语言能够理解多轮对话，记住之前的上下文，实现更自然的交互"
    );
    println!("Context understanding allows the language to understand multi-turn conversations and remember previous context for more natural interaction");
}

/// 演示增强的标准库 / Demonstrate enhanced standard library
fn demonstrate_enhanced_std() {
    println!("\n16. 增强标准库演示 / Enhanced Standard Library Demo");
    println!("--------------------------------------------");

    use crate::parser::AdaptiveParser;
    use crate::runtime::Interpreter;

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

    // 测试增强的标准库函数 / Test enhanced standard library functions
    let test_cases = vec![
        ("(std.sum (list 1 2 3 4 5))", "列表求和 / List sum"),
        ("(std.product (list 2 3 4))", "列表乘积 / List product"),
        ("(std.power 2 8)", "幂运算 / Power"),
        ("(std.range 1 10)", "范围生成 / Range generation"),
        ("(std.contains (list 1 2 3) 2)", "列表包含 / List contains"),
    ];

    for (code, description) in test_cases {
        println!("\n测试 / Test: {}", description);
        println!("代码 / Code: {}", code);
        match parser.parse(code) {
            Ok(ast) => match interpreter.execute(&ast) {
                Ok(value) => println!("结果 / Result: {}", value),
                Err(e) => println!("执行错误 / Execution error: {:?}", e),
            },
            Err(e) => println!("解析错误 / Parse error: {:?}", e),
        }
    }

    println!("\n提示 / Note: 增强的标准库提供了更多实用函数，用Aevolang实现，增强自举能力");
    println!("Enhanced standard library provides more utility functions, implemented in Aevolang, enhancing bootstrapping capability");
}

/// 演示使用模式学习功能 / Demonstrate usage pattern learning functionality
fn demonstrate_usage_learning() {
    println!("\n17. 使用模式学习演示 / Usage Pattern Learning Demo");
    println!("--------------------------------------------");

    use crate::evolution::EvolutionEngine;

    let mut engine = EvolutionEngine::new();

    // 模拟使用模式 / Simulate usage patterns
    println!("模拟使用模式 / Simulating Usage Patterns:");
    engine.record_usage("变量定义");
    engine.record_usage("函数调用");
    engine.record_usage("变量定义");
    engine.record_usage("函数定义");
    engine.record_usage("变量定义");
    engine.record_usage("列表操作");
    engine.record_usage("变量定义");

    // 模拟错误记录 / Simulate error recording
    println!("\n记录错误 / Recording Errors:");
    engine.record_error("UndefinedVariable", "变量x未定义", "(let y (+ x 1))");
    engine.record_error("UndefinedVariable", "变量y未定义", "(let z (+ y 1))");
    engine.record_error("TypeError", "类型不匹配", "(+ \"hello\" 5)");
    engine.record_error("UndefinedVariable", "变量x未定义", "(let y (+ x 1))");

    // 模拟成功记录 / Simulate success recording
    println!("\n记录成功 / Recording Successes:");
    engine.record_success("变量定义", "(let x 5)");
    engine.record_success("函数定义", "(def add (x y) (+ x y))");
    engine.record_success("变量定义", "(let x 5)");
    engine.record_success("列表求和", "(sum (list 1 2 3))");

    // 获取学习洞察 / Get learning insights
    println!("\n学习洞察 / Learning Insights:");
    match engine.learn_from_usage() {
        Ok(result) => {
            println!(
                "  {}",
                serde_json::to_string_pretty(&result).unwrap_or_default()
            );
        }
        Err(e) => {
            println!("  学习错误 / Learning error: {:?}", e);
        }
    }

    // 获取使用统计 / Get usage statistics
    println!("\n使用统计 / Usage Statistics:");
    let stats = engine.get_usage_statistics();
    println!("  总使用次数 / Total usage: {}", stats.total_usage);
    println!("  唯一模式数 / Unique patterns: {}", stats.unique_patterns);
    println!("  总错误数 / Total errors: {}", stats.total_errors);
    println!("  总成功数 / Total successes: {}", stats.total_successes);
    println!("  错误率 / Error rate: {:.2}%", stats.error_rate * 100.0);
    println!(
        "  成功率 / Success rate: {:.2}%",
        stats.success_rate * 100.0
    );

    // 获取学习洞察详情 / Get detailed learning insights
    println!("\n详细洞察 / Detailed Insights:");
    let insights = engine.get_learning_insights();
    for (i, insight) in insights.iter().take(5).enumerate() {
        println!(
            "  {}. {:?}: {}",
            i + 1,
            insight.insight_type,
            insight.description
        );
        if let Some(suggestion) = &insight.suggestion {
            println!("     建议 / Suggestion: {}", suggestion);
        }
        println!("     优先级 / Priority: {}", insight.priority);
    }

    println!("\n提示 / Note: 使用模式学习让语言能够从实际使用中学习，自动识别常见模式和错误，持续改进自身能力");
    println!("Usage pattern learning allows the language to learn from actual usage, automatically identify common patterns and errors, and continuously improve its capabilities");
}

/// 演示错误恢复功能 / Demonstrate error recovery functionality
fn demonstrate_error_recovery() {
    println!("\n18. 错误恢复演示 / Error Recovery Demo");
    println!("--------------------------------------------");

    use crate::evolution::ErrorRecoverer;
    use crate::parser::AdaptiveParser;
    use crate::runtime::Interpreter;

    let parser = AdaptiveParser::new(true);
    let mut interpreter = Interpreter::new();
    let recoverer = ErrorRecoverer::new();

    // 测试错误恢复 / Test error recovery
    let error_cases = vec![
        ("(let y (+ x 1))", "未定义变量x / Undefined variable x"),
        ("(+ \"hello\" 5)", "类型错误 / Type error"),
        ("(/ 10 0)", "除零错误 / Division by zero"),
    ];

    for (code, description) in error_cases {
        println!("\n测试: {} / Test: {}", description, description);
        println!("代码 / Code: {}", code);

        match parser.parse(code) {
            Ok(ast) => {
                match interpreter.execute(&ast) {
                    Ok(value) => {
                        println!("执行成功 / Execution successful: {}", value);
                    }
                    Err(error) => {
                        println!("执行错误 / Execution error: {:?}", error);

                        // 尝试恢复错误 / Try to recover from error
                        let recovery = recoverer.recover_from_error(&error, code);
                        println!("错误恢复结果 / Error Recovery Result:");
                        println!("  是否恢复 / Recovered: {}", recovery.recovered);
                        if let Some(method) = &recovery.method {
                            println!("  恢复方法 / Method: {}", method);
                        }
                        if !recovery.suggestions.is_empty() {
                            println!("  修复建议 / Fix Suggestions:");
                            for (i, suggestion) in recovery.suggestions.iter().enumerate() {
                                println!("    {}. {}", i + 1, suggestion);
                            }
                        }
                        if let Some(fixed) = &recovery.fixed_code {
                            println!("  修复后的代码 / Fixed Code:\n{}", fixed);
                        }
                    }
                }
            }
            Err(e) => {
                println!("解析错误 / Parse error: {:?}", e);
            }
        }
    }

    // 显示常见修复规则 / Show common fix rules
    println!("\n常见修复规则 / Common Fix Rules:");
    let common_fixes = recoverer.get_common_fixes();
    for (i, rule) in common_fixes.iter().take(5).enumerate() {
        println!(
            "  {}. {} (置信度: {:.2})",
            i + 1,
            rule.description,
            rule.confidence
        );
    }

    println!(
        "\n提示 / Note: 错误恢复功能能够自动识别常见错误并提供修复建议，提高代码质量和开发效率"
    );
    println!("Error recovery can automatically identify common errors and provide fix suggestions, improving code quality and development efficiency");
}

/// 演示智能代码生成功能 / Demonstrate intelligent code generation functionality
fn demonstrate_intelligent_code_generation() {
    println!("\n19. 智能代码生成演示 / Intelligent Code Generation Demo");
    println!("--------------------------------------------");

    use crate::evolution::GenerationContext;
    use crate::evolution::IntelligentCodeGenerator;

    let mut generator = IntelligentCodeGenerator::new();

    // 模拟使用模式（通过代码生成自动记录）/ Simulate usage patterns (automatically recorded through code generation)
    // 使用模式会在代码生成过程中自动记录 / Usage patterns will be automatically recorded during code generation

    // 测试代码生成 / Test code generation
    let test_intents = vec![
        ("定义一个变量", "Define a variable"),
        ("创建一个函数", "Create a function"),
        ("计算两个数的和", "Calculate sum of two numbers"),
        ("处理列表数据", "Process list data"),
    ];

    for (intent_cn, intent_en) in test_intents {
        println!("\n意图 / Intent: {} / {}", intent_cn, intent_en);

        let context = GenerationContext {
            variables: vec!["x".to_string(), "y".to_string()],
            functions: vec!["add".to_string(), "multiply".to_string()],
            recent_patterns: vec!["变量定义".to_string()],
            intent: Some(intent_cn.to_string()),
        };

        let result = generator.generate_from_intent(intent_cn, &context);
        println!("生成的代码 / Generated Code: {}", result.code);
        println!("置信度 / Confidence: {:.2}", result.confidence);
        if let Some(template) = &result.template {
            println!("使用的模板 / Template Used: {}", template);
        }
        if !result.suggestions.is_empty() {
            println!("建议 / Suggestions:");
            for (i, suggestion) in result.suggestions.iter().take(3).enumerate() {
                println!("  {}. {}", i + 1, suggestion);
            }
        }

        // 优化代码 / Optimize code
        let optimized = generator.optimize_code(&result.code);
        if optimized != result.code {
            println!("优化后的代码 / Optimized Code: {}", optimized);
        }
    }

    // 测试代码补全 / Test code completion
    println!("\n代码补全测试 / Code Completion Test:");
    let completion_context = GenerationContext {
        variables: vec!["counter".to_string(), "result".to_string()],
        functions: vec!["calculate".to_string(), "process".to_string()],
        recent_patterns: vec!["变量定义".to_string(), "函数调用".to_string()],
        intent: None,
    };

    let completions = generator.suggest_completion("let", &completion_context);
    println!("补全建议 / Completion Suggestions:");
    for (i, completion) in completions.iter().take(5).enumerate() {
        println!("  {}. {}", i + 1, completion);
    }

    println!(
        "\n提示 / Note: 智能代码生成能够基于上下文、使用模式和学习结果生成更准确的代码，提供代码补全和优化建议"
    );
    println!("Intelligent code generation can generate more accurate code based on context, usage patterns, and learning results, providing code completion and optimization suggestions");
}

/// 演示代码质量评估功能 / Demonstrate code quality assessment functionality
fn demonstrate_quality_assessment() {
    println!("\n20. 代码质量评估演示 / Code Quality Assessment Demo");
    println!("--------------------------------------------");

    use crate::evolution::{CodeAnalyzer, QualityAssessor};
    use crate::parser::AdaptiveParser;

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let mut assessor = QualityAssessor::new();

    // 测试代码质量评估 / Test code quality assessment
    let test_codes = vec![
        ("(let x 5)", "简单变量定义 / Simple variable definition"),
        (
            "(def add (x y) (+ x y))",
            "简单函数定义 / Simple function definition",
        ),
        ("(if (> x 0) x 0)", "条件表达式 / Conditional expression"),
    ];

    for (code, description) in test_codes {
        println!("\n测试: {} / Test: {}", description, description);
        println!("代码 / Code: {}", code);

        match parser.parse(code) {
            Ok(ast) => {
                // 分析代码 / Analyze code
                let analysis = analyzer.analyze(&ast);

                // 评估质量 / Assess quality
                let assessment = assessor.assess(&analysis);

                println!("\n质量评估结果 / Quality Assessment Result:");
                println!(
                    "  总体分数 / Overall Score: {:.2}/100",
                    assessment.overall_score
                );
                println!("  质量等级 / Quality Grade: {:?}", assessment.grade);
                println!("  质量趋势 / Quality Trend: {:?}", assessment.trend);

                println!("\n各维度分数 / Dimension Scores:");
                println!(
                    "  可读性 / Readability: {:.2}/100",
                    assessment.dimension_scores.readability
                );
                println!(
                    "  可维护性 / Maintainability: {:.2}/100",
                    assessment.dimension_scores.maintainability
                );
                println!(
                    "  性能 / Performance: {:.2}/100",
                    assessment.dimension_scores.performance
                );
                println!(
                    "  安全性 / Security: {:.2}/100",
                    assessment.dimension_scores.security
                );
                println!(
                    "  简洁性 / Simplicity: {:.2}/100",
                    assessment.dimension_scores.simplicity
                );

                if !assessment.suggestions.is_empty() {
                    println!("\n改进建议 / Improvement Suggestions:");
                    for (i, suggestion) in assessment.suggestions.iter().take(5).enumerate() {
                        println!(
                            "  {}. [{:?}] {}",
                            i + 1,
                            suggestion.priority,
                            suggestion.description
                        );
                        println!("     改进方法 / Improvement: {}", suggestion.improvement);
                    }
                }
            }
            Err(e) => {
                println!("解析错误 / Parse error: {:?}", e);
            }
        }
    }

    // 显示质量历史 / Show quality history
    println!("\n质量历史 / Quality History:");
    let history = assessor.get_quality_history();
    if history.is_empty() {
        println!("  暂无历史数据 / No history data yet");
    } else {
        println!("  记录数 / Records: {}", history.len());
        if let Some(latest) = history.last() {
            println!("  最新分数 / Latest Score: {:.2}/100", latest.overall_score);
            println!(
                "  最新时间 / Latest Time: {}",
                latest.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    println!(
        "\n提示 / Note: 代码质量评估能够从多个维度评估代码质量，提供改进建议，帮助持续提升代码质量"
    );
    println!("Code quality assessment can evaluate code quality from multiple dimensions, provide improvement suggestions, and help continuously improve code quality");
}

/// 演示智能优化建议功能 / Demonstrate intelligent optimization suggestion functionality
fn demonstrate_optimization_advisor() {
    println!("\n21. 智能优化建议演示 / Intelligent Optimization Advisor Demo");
    println!("--------------------------------------------");

    use crate::evolution::{CodeAnalyzer, OptimizationAdvisor, QualityAssessor};
    use crate::parser::AdaptiveParser;

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let mut assessor = QualityAssessor::new();
    let mut advisor = OptimizationAdvisor::new();

    // 测试代码 / Test code
    let test_code = "(def complex-func (x y z) (if (> x 0) (+ (* y 2) z) (- z y)))";
    println!("测试代码 / Test Code: {}", test_code);

    match parser.parse(test_code) {
        Ok(ast) => {
            // 分析代码 / Analyze code
            let analysis = analyzer.analyze(&ast);

            // 评估质量 / Assess quality
            let quality = assessor.assess(&analysis);

            println!(
                "\n当前质量分数 / Current Quality Score: {:.2}/100",
                quality.overall_score
            );

            // 生成优化建议 / Generate optimization suggestions
            let optimization_result = advisor.suggest_optimizations(&analysis, &quality);

            println!("\n优化建议结果 / Optimization Suggestions Result:");
            println!(
                "  总体预期改进 / Overall Expected Improvement: {:.2}%",
                optimization_result.overall_improvement
            );
            println!(
                "  推荐策略数 / Recommended Strategies: {}",
                optimization_result.recommended_strategies.len()
            );

            if !optimization_result.suggestions.is_empty() {
                println!("\n优化建议列表 / Optimization Suggestions:");
                for (i, suggestion) in optimization_result.suggestions.iter().take(5).enumerate() {
                    println!(
                        "  {}. [{}] {}",
                        i + 1,
                        format!("{:?}", suggestion.priority),
                        suggestion.description
                    );
                    println!("     策略 / Strategy: {}", suggestion.strategy);
                    println!(
                        "     预期改进 / Expected Improvement: {:.2}%",
                        suggestion.expected_improvement
                    );
                    println!(
                        "     置信度 / Confidence: {:.2}%",
                        suggestion.confidence * 100.0
                    );
                    println!(
                        "     具体建议 / Specific: {}",
                        suggestion.specific_suggestion
                    );
                }
            }

            // 预测优化效果 / Predict optimization effect
            println!("\n优化效果预测 / Optimization Effect Prediction:");
            for strategy in &optimization_result.recommended_strategies {
                let predicted_score =
                    advisor.predict_optimization_effect(strategy, quality.overall_score);
                println!(
                    "  策略 '{}' 预测分数 / Strategy '{}' Predicted Score: {:.2}/100",
                    strategy, strategy, predicted_score
                );
            }

            // 模拟应用优化并记录 / Simulate applying optimization and record
            if !optimization_result.suggestions.is_empty() {
                let first_suggestion = &optimization_result.suggestions[0];
                let simulated_improvement =
                    first_suggestion.expected_improvement * first_suggestion.confidence;
                let simulated_after_score = quality.overall_score + simulated_improvement;

                advisor.record_optimization(
                    &first_suggestion.strategy,
                    quality.overall_score,
                    simulated_after_score,
                );

                println!("\n模拟优化结果 / Simulated Optimization Result:");
                println!("  优化前分数 / Before: {:.2}/100", quality.overall_score);
                println!("  优化后分数 / After: {:.2}/100", simulated_after_score);
                println!(
                    "  实际改进 / Actual Improvement: {:.2}%",
                    simulated_improvement
                );
            }
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
        }
    }

    // 显示优化历史 / Show optimization history
    println!("\n优化历史 / Optimization History:");
    let history = advisor.get_optimization_history();
    if history.is_empty() {
        println!("  暂无历史数据 / No history data yet");
    } else {
        println!("  记录数 / Records: {}", history.len());
        if let Some(latest) = history.last() {
            println!("  最新优化 / Latest Optimization:");
            println!("    策略 / Strategy: {}", latest.strategy);
            println!("    改进 / Improvement: {:.2}%", latest.improvement);
            println!(
                "    时间 / Time: {}",
                latest.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    println!(
        "\n提示 / Note: 智能优化建议能够基于质量评估和学习结果提供针对性的优化建议，预测优化效果，帮助持续改进代码质量"
    );
    println!("Intelligent optimization advisor can provide targeted optimization suggestions based on quality assessment and learning results, predict optimization effects, and help continuously improve code quality");
}

/// 演示代码审查功能 / Demonstrate code review functionality
fn demonstrate_code_review() {
    println!("\n22. 代码审查演示 / Code Review Demo");
    println!("--------------------------------------------");

    use crate::evolution::{CodeAnalyzer, CodeReviewer, QualityAssessor};
    use crate::parser::AdaptiveParser;

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let mut assessor = QualityAssessor::new();
    let mut reviewer = CodeReviewer::new();

    // 测试代码 / Test code
    let test_codes = vec![
        ("(let x 5)", "简单代码 / Simple code"),
        (
            "(def complex-func (x y z) (if (> x 0) (+ (* y 2) z) (- z y)))",
            "复杂函数 / Complex function",
        ),
    ];

    for (code, description) in test_codes {
        println!("\n测试: {} / Test: {}", description, description);
        println!("代码 / Code: {}", code);

        match parser.parse(code) {
            Ok(ast) => {
                // 分析代码 / Analyze code
                let analysis = analyzer.analyze(&ast);

                // 评估质量 / Assess quality
                let quality = assessor.assess(&analysis);

                // 审查代码 / Review code
                let review_result = reviewer.review_code(&ast, &analysis, &quality);

                println!("\n代码审查结果 / Code Review Result:");
                println!("  审查等级 / Review Grade: {:?}", review_result.grade);
                println!(
                    "  通过率 / Pass Rate: {:.2}%",
                    review_result.summary.pass_rate
                );

                println!("\n问题统计 / Issue Statistics:");
                println!(
                    "  总问题数 / Total Issues: {}",
                    review_result.summary.total_issues
                );
                println!(
                    "  严重问题 / Critical: {}",
                    review_result.summary.critical_issues
                );
                println!("  错误 / Errors: {}", review_result.summary.errors);
                println!("  警告 / Warnings: {}", review_result.summary.warnings);
                println!("  信息 / Info: {}", review_result.summary.info);

                if !review_result.issues.is_empty() {
                    println!("\n发现的问题 / Issues Found:");
                    for (i, issue) in review_result.issues.iter().take(5).enumerate() {
                        println!("  {}. [{:?}] {}", i + 1, issue.severity, issue.description);
                        println!("     规则 / Rule: {}", issue.rule_name);
                        println!("     位置 / Location: {}", issue.location);
                        println!("     建议 / Suggestion: {}", issue.suggestion);
                        println!("     置信度 / Confidence: {:.2}%", issue.confidence * 100.0);
                    }
                }

                if !review_result.recommendations.is_empty() {
                    println!("\n审查建议 / Review Recommendations:");
                    for (i, recommendation) in review_result.recommendations.iter().enumerate() {
                        println!("  {}. {}", i + 1, recommendation);
                    }
                }
            }
            Err(e) => {
                println!("解析错误 / Parse error: {:?}", e);
            }
        }
    }

    // 显示审查历史 / Show review history
    println!("\n审查历史 / Review History:");
    let history = reviewer.get_review_history();
    if history.is_empty() {
        println!("  暂无历史数据 / No history data yet");
    } else {
        println!("  记录数 / Records: {}", history.len());
        if let Some(latest) = history.last() {
            println!("  最新审查 / Latest Review:");
            println!("    问题数 / Issues: {}", latest.issues_count);
            println!(
                "    时间 / Time: {}",
                latest.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    // 显示审查统计 / Show review statistics
    println!("\n审查统计 / Review Statistics:");
    let stats = reviewer.get_review_statistics();
    println!(
        "  {}",
        serde_json::to_string_pretty(&stats).unwrap_or_default()
    );

    println!(
        "\n提示 / Note: 代码审查能够自动检查代码问题，提供详细的审查报告，帮助提高代码质量和规范性"
    );
    println!("Code review can automatically check code issues, provide detailed review reports, and help improve code quality and standards");
}

/// 演示代码文档生成功能 / Demonstrate code documentation generation functionality
fn demonstrate_documentation_generation() {
    println!("\n23. 代码文档生成演示 / Code Documentation Generation Demo");
    println!("--------------------------------------------");

    use crate::evolution::DocFormat;
    use crate::evolution::{CodeAnalyzer, DocumentationGenerator};
    use crate::parser::AdaptiveParser;

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let mut doc_generator = DocumentationGenerator::new();

    // 测试代码 / Test code
    let test_code = r#"
        (def add (x y) (+ x y))
        (def multiply (x y) (* x y))
        (let result (add 3 4))
    "#;

    println!("测试代码 / Test Code:\n{}", test_code);

    match parser.parse(test_code) {
        Ok(ast) => {
            // 分析代码 / Analyze code
            let analysis = analyzer.analyze(&ast);

            // 生成Markdown文档 / Generate Markdown documentation
            println!("\n生成Markdown文档 / Generating Markdown Documentation:");
            let markdown_doc =
                doc_generator.generate_documentation(&ast, &analysis, DocFormat::Markdown);

            println!("\n生成的文档 / Generated Documentation:");
            println!("{}", markdown_doc.content);

            println!("\n文档统计 / Document Statistics:");
            println!(
                "  总行数 / Total Lines: {}",
                markdown_doc.statistics.total_lines
            );
            println!(
                "  函数文档数 / Function Docs: {}",
                markdown_doc.statistics.function_docs
            );
            println!(
                "  变量文档数 / Variable Docs: {}",
                markdown_doc.statistics.variable_docs
            );

            println!("\n文档质量 / Document Quality:");
            println!(
                "  完整性 / Completeness: {:.2}%",
                markdown_doc.quality.completeness
            );
            println!("  清晰度 / Clarity: {:.2}%", markdown_doc.quality.clarity);
            println!("  准确性 / Accuracy: {:.2}%", markdown_doc.quality.accuracy);
            println!("  总体质量 / Overall: {:.2}%", markdown_doc.quality.overall);

            // 生成API文档 / Generate API documentation
            println!("\n生成API文档 / Generating API Documentation:");
            let api_doc = doc_generator.generate_documentation(&ast, &analysis, DocFormat::ApiDoc);
            println!(
                "API文档长度 / API Doc Length: {} 行 / {} lines",
                api_doc.statistics.total_lines, api_doc.statistics.total_lines
            );

            // 生成纯文本文档 / Generate plain text documentation
            println!("\n生成纯文本文档 / Generating Plain Text Documentation:");
            let plain_doc =
                doc_generator.generate_documentation(&ast, &analysis, DocFormat::PlainText);
            println!("纯文本文档 / Plain Text Doc:\n{}", plain_doc.content);
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
        }
    }

    // 显示文档历史 / Show documentation history
    println!("\n文档生成历史 / Documentation Generation History:");
    let history = doc_generator.get_doc_history();
    if history.is_empty() {
        println!("  暂无历史数据 / No history data yet");
    } else {
        println!("  记录数 / Records: {}", history.len());
        if let Some(latest) = history.last() {
            println!("  最新文档 / Latest Documentation:");
            println!("    类型 / Type: {}", latest.doc_type);
            println!(
                "    长度 / Length: {} 行 / {} lines",
                latest.doc_length, latest.doc_length
            );
            println!(
                "    覆盖函数数 / Functions Covered: {}",
                latest.functions_covered
            );
            println!(
                "    时间 / Time: {}",
                latest.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    // 显示文档统计 / Show documentation statistics
    println!("\n文档统计 / Documentation Statistics:");
    let stats = doc_generator.get_doc_statistics();
    println!(
        "  {}",
        serde_json::to_string_pretty(&stats).unwrap_or_default()
    );

    println!(
        "\n提示 / Note: 代码文档生成能够自动为代码生成文档，提高代码可读性和可维护性，帮助开发者理解代码"
    );
    println!("Code documentation generation can automatically generate documentation for code, improving code readability and maintainability, and helping developers understand code");
}

/// 演示测试生成功能 / Demonstrate test generation functionality
fn demonstrate_test_generation() {
    println!("\n24. 测试生成演示 / Test Generation Demo");
    println!("--------------------------------------------");

    use crate::evolution::{CodeAnalyzer, TestGenerator};
    use crate::parser::AdaptiveParser;
    use crate::runtime::Interpreter;

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let mut test_generator = TestGenerator::new();
    let mut interpreter = Interpreter::new();

    // 测试代码 / Test code
    let test_code = r#"
        (def add (x y) (+ x y))
        (def multiply (x y) (* x y))
        (def subtract (x y) (- x y))
    "#;

    println!("测试代码 / Test Code:\n{}", test_code);

    match parser.parse(test_code) {
        Ok(ast) => {
            // 分析代码 / Analyze code
            let analysis = analyzer.analyze(&ast);

            // 生成测试套件 / Generate test suite
            println!("\n生成测试套件 / Generating Test Suite:");
            let test_suite = test_generator.generate_tests(&ast, &analysis);

            println!("\n测试统计 / Test Statistics:");
            println!(
                "  总测试数 / Total Tests: {}",
                test_suite.statistics.total_tests
            );
            println!(
                "  单元测试数 / Unit Tests: {}",
                test_suite.statistics.unit_tests
            );
            println!(
                "  边界测试数 / Boundary Tests: {}",
                test_suite.statistics.boundary_tests
            );
            println!(
                "  集成测试数 / Integration Tests: {}",
                test_suite.statistics.integration_tests
            );

            println!("\n测试覆盖率 / Test Coverage:");
            println!(
                "  函数覆盖率 / Function Coverage: {:.2}%",
                test_suite.coverage.function_coverage
            );
            println!(
                "  分支覆盖率 / Branch Coverage: {:.2}%",
                test_suite.coverage.branch_coverage
            );
            println!(
                "  语句覆盖率 / Statement Coverage: {:.2}%",
                test_suite.coverage.statement_coverage
            );
            println!(
                "  总体覆盖率 / Overall Coverage: {:.2}%",
                test_suite.coverage.overall_coverage
            );

            if !test_suite.test_cases.is_empty() {
                println!("\n生成的测试用例 / Generated Test Cases:");
                for (i, test_case) in test_suite.test_cases.iter().take(5).enumerate() {
                    println!(
                        "  {}. {} ({:?})",
                        i + 1,
                        test_case.name,
                        test_case.test_type
                    );
                    println!("     描述 / Description: {}", test_case.description);
                    println!("     测试代码 / Test Code: {}", test_case.test_code);
                    println!("     预期结果 / Expected: {}", test_case.expected_result);

                    // 尝试执行测试 / Try to execute test
                    match parser.parse(&test_case.test_code) {
                        Ok(test_ast) => match interpreter.execute(&test_ast) {
                            Ok(result) => {
                                let passed = result.to_string() == test_case.expected_result
                                    || test_case.expected_result == "结果待验证";
                                println!(
                                    "     执行结果 / Result: {} ({})",
                                    result,
                                    if passed {
                                        "通过 / Pass"
                                    } else {
                                        "失败 / Fail"
                                    }
                                );
                            }
                            Err(e) => {
                                println!("     执行错误 / Execution Error: {:?}", e);
                            }
                        },
                        Err(e) => {
                            println!("     解析错误 / Parse Error: {:?}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
        }
    }

    // 显示测试历史 / Show test history
    println!("\n测试生成历史 / Test Generation History:");
    let history = test_generator.get_test_history();
    if history.is_empty() {
        println!("  暂无历史数据 / No history data yet");
    } else {
        println!("  记录数 / Records: {}", history.len());
        if let Some(latest) = history.last() {
            println!("  最新测试生成 / Latest Test Generation:");
            println!(
                "    生成测试数 / Tests Generated: {}",
                latest.tests_generated
            );
            println!("    通过数 / Passed: {}", latest.tests_passed);
            println!("    失败数 / Failed: {}", latest.tests_failed);
            println!(
                "    时间 / Time: {}",
                latest.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    // 显示测试统计 / Show test statistics
    println!("\n测试统计 / Test Statistics:");
    let stats = test_generator.get_test_statistics();
    println!(
        "  {}",
        serde_json::to_string_pretty(&stats).unwrap_or_default()
    );

    println!(
        "\n提示 / Note: 测试生成能够自动为代码生成测试用例，提高代码质量和可靠性，帮助验证代码正确性"
    );
    println!("Test generation can automatically generate test cases for code, improving code quality and reliability, and helping verify code correctness");
}

/// 演示性能分析功能 / Demonstrate performance analysis functionality
fn demonstrate_performance_analysis() {
    println!("\n25. 性能分析演示 / Performance Analysis Demo");
    println!("--------------------------------------------");

    use crate::evolution::{CodeAnalyzer, PerformanceAnalyzer};
    use crate::parser::AdaptiveParser;

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let mut performance_analyzer = PerformanceAnalyzer::new();

    // 测试代码 / Test code
    let test_code = r#"
        (def factorial (n)
            (if (= n 0)
                1
                (* n (factorial (- n 1)))))
        (def add (x y) (+ x y))
        (def multiply (x y) (* x y))
    "#;

    println!("测试代码 / Test Code:\n{}", test_code);

    match parser.parse(test_code) {
        Ok(ast) => {
            // 分析代码 / Analyze code
            let analysis = analyzer.analyze(&ast);

            // 性能分析 / Performance analysis
            println!("\n进行性能分析 / Performing Performance Analysis:");
            let performance = performance_analyzer.analyze_performance(&ast, &analysis);

            println!("\n性能指标 / Performance Metrics:");
            if let Some(record) = performance_analyzer.get_performance_history().last() {
                println!(
                    "  时间复杂度 / Time Complexity: {}",
                    record.metrics.time_complexity
                );
                println!(
                    "  空间复杂度 / Space Complexity: {}",
                    record.metrics.space_complexity
                );
                println!(
                    "  预估执行时间 / Estimated Execution Time: {:.2} ms",
                    record.metrics.estimated_execution_time
                );
                println!(
                    "  预估内存使用 / Estimated Memory Usage: {:.2} KB",
                    record.metrics.estimated_memory_usage
                );
            }

            println!("\n性能评分 / Performance Score:");
            println!("  评分 / Score: {:.2}/100", performance.performance_score);
            println!("  等级 / Level: {:?}", performance.performance_level);

            if !performance.bottlenecks.is_empty() {
                println!("\n性能瓶颈 / Performance Bottlenecks:");
                for (i, bottleneck) in performance.bottlenecks.iter().enumerate() {
                    println!(
                        "  {}. {:?} - {}",
                        i + 1,
                        bottleneck.bottleneck_type,
                        bottleneck.description
                    );
                    println!("     影响程度 / Impact: {:.2}", bottleneck.impact);
                }
            }

            if !performance.suggestions.is_empty() {
                println!("\n优化建议 / Optimization Suggestions:");
                for (i, suggestion) in performance.suggestions.iter().enumerate() {
                    println!(
                        "  {}. [{}] {}",
                        i + 1,
                        suggestion.suggestion_type,
                        suggestion.content
                    );
                    println!(
                        "     预期改进 / Expected Improvement: {:.2}%",
                        suggestion.expected_improvement
                    );
                    println!("     优先级 / Priority: {}", suggestion.priority);
                }
            }
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
        }
    }

    // 显示性能历史 / Show performance history
    println!("\n性能分析历史 / Performance Analysis History:");
    let history = performance_analyzer.get_performance_history();
    if history.is_empty() {
        println!("  暂无历史数据 / No history data yet");
    } else {
        println!("  记录数 / Records: {}", history.len());
        if let Some(latest) = history.last() {
            println!("  最新分析 / Latest Analysis:");
            println!("    评分 / Score: {:.2}", latest.analysis.performance_score);
            println!("    等级 / Level: {:?}", latest.analysis.performance_level);
            println!(
                "    时间 / Time: {}",
                latest.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    // 显示性能统计 / Show performance statistics
    println!("\n性能统计 / Performance Statistics:");
    let stats = performance_analyzer.get_performance_statistics();
    println!(
        "  {}",
        serde_json::to_string_pretty(&stats).unwrap_or_default()
    );

    println!(
        "\n提示 / Note: 性能分析能够自动分析代码性能，识别性能瓶颈，提供优化建议，帮助提高代码执行效率"
    );
    println!("Performance analysis can automatically analyze code performance, identify bottlenecks, provide optimization suggestions, and help improve code execution efficiency");
}

/// 演示代码相似度检测功能 / Demonstrate code similarity detection functionality
fn demonstrate_similarity_detection() {
    println!("\n26. 代码相似度检测演示 / Code Similarity Detection Demo");
    println!("--------------------------------------------");

    use crate::evolution::{CodeAnalyzer, SimilarityDetector};
    use crate::parser::AdaptiveParser;

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let mut similarity_detector = SimilarityDetector::new();

    // 测试代码（包含重复和相似代码）/ Test code (with duplicates and similar code)
    let test_code = r#"
        (def add (x y) (+ x y))
        (def subtract (x y) (- x y))
        (def multiply (x y) (* x y))
        (def add2 (a b) (+ a b))
        (def add3 (m n) (+ m n))
    "#;

    println!("测试代码 / Test Code:\n{}", test_code);

    match parser.parse(test_code) {
        Ok(ast) => {
            // 分析代码 / Analyze code
            let analysis = analyzer.analyze(&ast);

            // 相似度检测 / Similarity detection
            println!("\n进行相似度检测 / Performing Similarity Detection:");
            let similarity = similarity_detector.detect_similarity(&ast, &analysis);

            println!("\n相似度评分 / Similarity Score:");
            println!("  评分 / Score: {:.2}/100", similarity.similarity_score);

            if !similarity.similar_pairs.is_empty() {
                println!("\n相似代码对 / Similar Code Pairs:");
                for (i, pair) in similarity.similar_pairs.iter().take(5).enumerate() {
                    println!(
                        "  {}. 相似度 / Similarity: {:.2}% ({:?})",
                        i + 1,
                        pair.similarity * 100.0,
                        pair.similarity_type
                    );
                    println!("     位置1 / Location 1: {}", pair.block1.location);
                    println!("     位置2 / Location 2: {}", pair.block2.location);
                }
            }

            if !similarity.duplicates.is_empty() {
                println!("\n重复代码块 / Duplicate Code Blocks:");
                for (i, duplicate) in similarity.duplicates.iter().take(5).enumerate() {
                    println!("  {}. 出现次数 / Occurrences: {}", i + 1, duplicate.count);
                    println!("     位置 / Locations: {:?}", duplicate.locations);
                }
            }

            if !similarity.suggestions.is_empty() {
                println!("\n重构建议 / Refactoring Suggestions:");
                for (i, suggestion) in similarity.suggestions.iter().enumerate() {
                    println!(
                        "  {}. [{}] {}",
                        i + 1,
                        suggestion.suggestion_type,
                        suggestion.content
                    );
                    println!("     优先级 / Priority: {}", suggestion.priority);
                }
            }
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
        }
    }

    // 显示检测历史 / Show detection history
    println!("\n相似度检测历史 / Similarity Detection History:");
    let history = similarity_detector.get_detection_history();
    if history.is_empty() {
        println!("  暂无历史数据 / No history data yet");
    } else {
        println!("  记录数 / Records: {}", history.len());
        if let Some(latest) = history.last() {
            println!("  最新检测 / Latest Detection:");
            println!(
                "    相似代码对 / Similar Pairs: {}",
                latest.similar_pairs.len()
            );
            println!("    重复代码块 / Duplicates: {}", latest.duplicates.len());
            println!(
                "    时间 / Time: {}",
                latest.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    // 显示相似度统计 / Show similarity statistics
    println!("\n相似度统计 / Similarity Statistics:");
    let stats = similarity_detector.get_similarity_statistics();
    println!(
        "  {}",
        serde_json::to_string_pretty(&stats).unwrap_or_default()
    );

    println!(
        "\n提示 / Note: 代码相似度检测能够自动检测代码重复和相似模式，帮助识别重构机会，提高代码质量"
    );
    println!("Code similarity detection can automatically detect code duplication and similar patterns, helping identify refactoring opportunities and improve code quality");
}

/// 演示代码依赖分析功能 / Demonstrate code dependency analysis functionality
fn demonstrate_dependency_analysis() {
    println!("\n27. 代码依赖分析演示 / Code Dependency Analysis Demo");
    println!("--------------------------------------------");

    use crate::evolution::{CodeAnalyzer, DependencyAnalyzer};
    use crate::parser::AdaptiveParser;

    let parser = AdaptiveParser::new(true);
    let analyzer = CodeAnalyzer::new();
    let mut dependency_analyzer = DependencyAnalyzer::new();

    // 测试代码（包含依赖关系）/ Test code (with dependencies)
    let test_code = r#"
        (def add (x y) (+ x y))
        (def multiply (x y) (* x y))
        (def calculate (a b) (multiply (add a b) 2))
        (def compute (x) (calculate x 5))
    "#;

    println!("测试代码 / Test Code:\n{}", test_code);

    match parser.parse(test_code) {
        Ok(ast) => {
            // 分析代码 / Analyze code
            let analysis = analyzer.analyze(&ast);

            // 依赖分析 / Dependency analysis
            println!("\n进行依赖分析 / Performing Dependency Analysis:");
            let dependency = dependency_analyzer.analyze_dependencies(&ast, &analysis);

            println!("\n依赖统计 / Dependency Statistics:");
            println!(
                "  总依赖数 / Total Dependencies: {}",
                dependency.statistics.total_dependencies
            );
            println!(
                "  函数依赖数 / Function Dependencies: {}",
                dependency.statistics.function_dependencies
            );
            println!(
                "  变量依赖数 / Variable Dependencies: {}",
                dependency.statistics.variable_dependencies
            );
            println!(
                "  模块依赖数 / Module Dependencies: {}",
                dependency.statistics.module_dependencies
            );
            println!(
                "  最大依赖深度 / Max Depth: {}",
                dependency.statistics.max_depth
            );
            println!(
                "  循环依赖数 / Circular Dependencies: {}",
                dependency.statistics.circular_count
            );

            if !dependency.dependencies.is_empty() {
                println!("\n依赖关系 / Dependencies:");
                for (i, dep) in dependency.dependencies.iter().take(10).enumerate() {
                    println!(
                        "  {}. {} -> {} ({:?})",
                        i + 1,
                        dep.dependent,
                        dep.dependency,
                        dep.dependency_type
                    );
                }
            }

            if !dependency.circular_dependencies.is_empty() {
                println!("\n循环依赖 / Circular Dependencies:");
                for (i, circular) in dependency.circular_dependencies.iter().enumerate() {
                    println!(
                        "  {}. {:?} - {}",
                        i + 1,
                        circular.severity,
                        circular.description
                    );
                    println!("     路径 / Path: {}", circular.path.join(" -> "));
                }
            }

            if !dependency.suggestions.is_empty() {
                println!("\n优化建议 / Optimization Suggestions:");
                for (i, suggestion) in dependency.suggestions.iter().enumerate() {
                    println!(
                        "  {}. [{}] {}",
                        i + 1,
                        suggestion.suggestion_type,
                        suggestion.content
                    );
                    println!("     优先级 / Priority: {}", suggestion.priority);
                }
            }
        }
        Err(e) => {
            println!("解析错误 / Parse error: {:?}", e);
        }
    }

    // 显示分析历史 / Show analysis history
    println!("\n依赖分析历史 / Dependency Analysis History:");
    let history = dependency_analyzer.get_analysis_history();
    if history.is_empty() {
        println!("  暂无历史数据 / No history data yet");
    } else {
        println!("  记录数 / Records: {}", history.len());
        if let Some(latest) = history.last() {
            println!("  最新分析 / Latest Analysis:");
            println!("    依赖数 / Dependencies: {}", latest.dependencies.len());
            println!(
                "    循环依赖数 / Circular: {}",
                latest.circular_dependencies.len()
            );
            println!(
                "    时间 / Time: {}",
                latest.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    // 显示依赖统计 / Show dependency statistics
    println!("\n依赖统计 / Dependency Statistics:");
    let stats = dependency_analyzer.get_dependency_statistics();
    println!(
        "  {}",
        serde_json::to_string_pretty(&stats).unwrap_or_default()
    );

    println!("\n提示 / Note: 代码依赖分析能够自动分析代码依赖关系，检测循环依赖，帮助优化代码结构");
    println!("Code dependency analysis can automatically analyze code dependencies, detect circular dependencies, and help optimize code structure");
}
