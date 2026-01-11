// Aevolang - 自进化编程语言 / Self-evolving programming language
// 该语言的核心特点是能够根据使用和需求自我进化
// The core feature of this language is the ability to self-evolve based on usage and needs
// 终极目标：理解人类思想，促进人类与智能生命和谐共生
// Ultimate goal: Understand human thoughts and promote harmonious coexistence between humans and intelligent life

mod grammar;
mod parser;
mod evolution;
mod runtime;
mod python;
mod poetry;

use grammar::*;
use parser::*;
use evolution::*;
use runtime::*;
use poetry::*;

fn main() {
    println!("Aevolang - 自进化编程语言 / Self-evolving Programming Language");
    println!("============================================================");
    
    // 演示《静夜思》的解析和理解 / Demonstrate parsing and understanding of "Quiet Night Thoughts"
    demonstrate_poetry_understanding();
    
    // 演示语法定义 / Demonstrate grammar definition
    demonstrate_grammar_definition();
    
    // 演示进化引擎 / Demonstrate evolution engine
    demonstrate_evolution_engine();
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
            println!("主要情感 / Primary Emotion: {:?}", analysis.emotion_analysis.primary_emotion);
            println!("置信度 / Confidence: {:.2}", analysis.emotion_analysis.confidence);
            println!("\n检测到的情感 / Detected Emotions:");
            for (emotion, score) in &analysis.emotion_analysis.emotion_scores {
                if *score > 0.1 {
                    println!("  {:?}: {:.2}", emotion, score);
                }
            }
            println!("\n主题 / Themes:");
            for theme in &analysis.themes {
                println!("  {}: {} (置信度: {:.2})", theme.name, theme.description, theme.confidence);
            }
            println!("\n意象 / Imagery:");
            for img in &analysis.imagery {
                println!("  {}: {} (出现{}次)", img.element, img.meaning, img.frequency);
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
    println!("稳定性 / Stability: {:?}", syntax_def_rule.rule.meta.stability);
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
