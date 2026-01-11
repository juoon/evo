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
