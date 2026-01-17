// 进化引擎核心 / Evolution engine core
// 驱动语言的自进化过程
// Drives the self-evolution process of the language

use crate::evolution::tracker::{EvolutionEvent, EvolutionTracker, EvolutionType, TriggerSource};
use crate::grammar::core::GrammarElement;
use crate::grammar::rule::{
    DefinitionMethod, GrammarRule, Pattern, PatternElement, Production, RuleMetadata, Stability,
};
use crate::parser::nlu::NLUParser;
use crate::parser::AdaptiveParser;
use crate::poetry::PoetryParser;
use crate::runtime::interpreter::{Interpreter, Value};
use std::collections::HashMap;

/// 进化引擎 / Evolution engine
pub struct EvolutionEngine {
    /// 语法变异记录 / Syntax mutation records
    syntax_mutations: Vec<GrammarRule>,
    /// 语义适应记录 / Semantic adaptation records
    semantic_adaptations: Vec<GrammarRule>,
    /// 进化追踪器 / Evolution tracker
    tracker: EvolutionTracker,
    /// NLU解析器 / NLU parser
    nlu_parser: NLUParser,
    /// 诗歌解析器 / Poetry parser
    poetry_parser: PoetryParser,
    /// 知识图谱 / Knowledge graph
    knowledge_graph: crate::evolution::knowledge::EvolutionKnowledgeGraph,
}

impl EvolutionEngine {
    /// 创建新进化引擎 / Create new evolution engine
    pub fn new() -> Self {
        let bootstrap_rules = Self::load_bootstrap_rules();
        let mut engine = Self {
            syntax_mutations: bootstrap_rules,
            semantic_adaptations: Vec::new(),
            tracker: EvolutionTracker::new(),
            nlu_parser: NLUParser::new(crate::parser::nlu::ModelType::LocalLightweight, true),
            poetry_parser: PoetryParser::new(),
            knowledge_graph: crate::evolution::knowledge::EvolutionKnowledgeGraph::new(),
        };

        // 从历史构建知识图谱 / Build knowledge graph from history
        engine.rebuild_knowledge();

        engine
    }

    /// 重建知识图谱 / Rebuild knowledge graph
    fn rebuild_knowledge(&mut self) {
        let history = self.tracker.get_history();
        self.knowledge_graph.build_from_history(history);
    }

    /// 从自然语言进化 / Evolve from natural language
    pub fn evolve_from_natural_language(
        &mut self,
        nl_input: &str,
    ) -> Result<Vec<GrammarRule>, EvolutionError> {
        // 解析自然语言意图 / Parse natural language intent
        let intent = self
            .nlu_parser
            .extract_intent(nl_input)
            .map_err(|e| EvolutionError::NLUError(format!("{:?}", e)))?;

        // 生成语法变体 / Generate syntax variants
        let syntax_variants = self.generate_syntax_variants(&intent)?;

        // 测试并选择最优变体 / Test and select optimal variant
        let optimal = self.test_variants(syntax_variants)?;

        // 集成新特性 / Integrate new feature
        self.integrate_new_feature(optimal.clone())?;

        Ok(vec![optimal])
    }

    /// 生成语法变体 / Generate syntax variants
    fn generate_syntax_variants(
        &self,
        intent: &crate::parser::nlu::ProgrammingIntent,
    ) -> Result<Vec<GrammarRule>, EvolutionError> {
        let aevo_intent = self.intent_to_aevo_dict(intent);
        let code = format!(
            "(import \"evolution\")\n(evolution.generate_variants {})",
            aevo_intent
        );
        let value = self.execute_aevo_code(&code)?;
        self.rules_from_value(&value)
    }

    /// 测试变体 / Test variants
    fn test_variants(&self, variants: Vec<GrammarRule>) -> Result<GrammarRule, EvolutionError> {
        // TODO: 实现变体测试逻辑 / Implement variant testing logic
        // 暂时返回第一个变体 / Temporarily return first variant
        variants
            .into_iter()
            .next()
            .ok_or(EvolutionError::NoVariants)
    }

    /// 集成新特性 / Integrate new feature
    fn integrate_new_feature(&mut self, rule: GrammarRule) -> Result<(), EvolutionError> {
        // 记录进化事件 / Record evolution event
        let event = EvolutionEvent {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_type: EvolutionType::SyntaxEvolution,
            before_state: crate::evolution::tracker::StateSnapshot {
                grammar_rules: self.syntax_mutations.clone(),
                version: "0.1.0".to_string(),
                metadata: serde_json::json!({}),
            },
            after_state: crate::evolution::tracker::StateSnapshot {
                grammar_rules: {
                    let mut rules = self.syntax_mutations.clone();
                    rules.push(rule.clone());
                    rules
                },
                version: "0.1.1".to_string(),
                metadata: serde_json::json!({}),
            },
            delta: crate::evolution::tracker::EvolutionDelta {
                added_rules: vec![rule.clone()],
                modified_rules: Vec::new(),
                removed_rules: Vec::new(),
                description: "Added new grammar rule from natural language".to_string(),
            },
            trigger: crate::evolution::tracker::TriggerContext {
                source: TriggerSource::NaturalLanguageInstruction,
                conditions: Vec::new(),
                environment: serde_json::json!({}),
            },
            author: None,
            success_metrics: None,
        };

        self.tracker.record(event.clone());
        self.syntax_mutations.push(rule);

        // 更新知识图谱 / Update knowledge graph
        self.knowledge_graph.build_from_history(&[event]);

        Ok(())
    }

    /// 预测可能的进化 / Predict possible evolutions
    pub fn predict_evolutions(
        &self,
        goals: Vec<String>,
    ) -> Vec<crate::evolution::knowledge::EvolutionPrediction> {
        let context = crate::evolution::knowledge::EvolutionContext {
            current_state: serde_json::json!({
                "rules_count": self.syntax_mutations.len(),
                "adaptations_count": self.semantic_adaptations.len(),
            }),
            goals,
            constraints: Vec::new(),
        };
        self.knowledge_graph.predict_evolutions(&context)
    }

    /// 获取知识图谱统计 / Get knowledge graph statistics
    pub fn get_knowledge_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "nodes_count": self.knowledge_graph.get_node_count(),
            "patterns_count": self.knowledge_graph.get_patterns_count(),
        })
    }

    /// 分析代码并提供优化建议 / Analyze code and provide optimization suggestions
    pub fn analyze_code(&self, ast: &[GrammarElement]) -> crate::evolution::analyzer::CodeAnalysis {
        let analyzer = crate::evolution::analyzer::CodeAnalyzer::new();
        analyzer.analyze(ast)
    }

    /// 自动重构代码 / Automatically refactor code
    pub fn refactor_code(&self, ast: &[GrammarElement]) -> Vec<GrammarElement> {
        // 先分析代码 / First analyze code
        let analysis = self.analyze_code(ast);
        
        // 根据分析结果重构 / Refactor based on analysis
        let refactorer = crate::evolution::analyzer::CodeRefactorer::new();
        refactorer.refactor(ast, &analysis)
    }

    /// 自我反思：评估进化效果 / Self-reflection: evaluate evolution effectiveness
    pub fn self_reflect(&self) -> serde_json::Value {
        let history = self.tracker.get_history();
        let mut reflection = serde_json::json!({});

        // 统计进化频率 / Statistics on evolution frequency
        let evolution_count = history.len();
        let recent_evolutions = history.iter()
            .filter(|e| {
                let days_ago = (chrono::Utc::now() - e.timestamp).num_days();
                days_ago <= 7
            })
            .count();

        // 分析进化趋势 / Analyze evolution trends
        let mut syntax_evolutions = 0;
        let mut semantic_evolutions = 0;
        for event in history {
            match event.event_type {
                crate::evolution::tracker::EvolutionType::SyntaxEvolution => syntax_evolutions += 1,
                crate::evolution::tracker::EvolutionType::SemanticEvolution => semantic_evolutions += 1,
                _ => {}
            }
        }

        // 评估知识图谱的丰富度 / Evaluate knowledge graph richness
        let stats = self.get_knowledge_stats();
        let knowledge_richness = if stats["nodes_count"].as_u64().unwrap_or(0) > 10 {
            "丰富"
        } else if stats["nodes_count"].as_u64().unwrap_or(0) > 5 {
            "中等"
        } else {
            "基础"
        };

        reflection = serde_json::json!({
            "total_evolutions": evolution_count,
            "recent_evolutions_7days": recent_evolutions,
            "syntax_evolutions": syntax_evolutions,
            "semantic_evolutions": semantic_evolutions,
            "knowledge_richness": knowledge_richness,
            "rules_count": self.syntax_mutations.len(),
            "knowledge_nodes": stats["nodes_count"],
            "patterns_discovered": stats["patterns_count"],
            "self_assessment": if evolution_count > 5 && stats["nodes_count"].as_u64().unwrap_or(0) > 5 {
                "语言正在积极进化，知识图谱不断丰富"
            } else if evolution_count > 0 {
                "语言开始进化，知识图谱正在构建"
            } else {
                "语言处于初始状态，等待进化触发"
            },
        });

        reflection
    }

    /// 查找相似规则 / Find similar rules
    pub fn find_similar_rules(&self, rule_name: &str) -> Vec<(String, f64)> {
        let entity_id = format!("rule:{}", rule_name);
        self.knowledge_graph.find_similar_entities(&entity_id, 0.3)
    }

    /// 回滚到指定事件 / Rollback to specified event
    pub fn rollback_to_event(&mut self, event_id: uuid::Uuid) -> Result<(), EvolutionError> {
        // 回滚到指定事件之前的状态 / Rollback to state before specified event
        let rollback_state = self.tracker.rollback_to(event_id)
            .map_err(|e| EvolutionError::IntegrationFailed(e))?;
        
        // 恢复语法规则 / Restore grammar rules
        self.syntax_mutations = rollback_state.grammar_rules.clone();
        
        // 重建知识图谱 / Rebuild knowledge graph
        self.rebuild_knowledge();
        
        Ok(())
    }

    /// 获取进化谱系树 / Get evolution genealogy tree
    pub fn get_genealogy_tree(&self, root_id: Option<uuid::Uuid>) -> serde_json::Value {
        let genealogy = self.tracker.get_genealogy();
        
        if let Some(root) = root_id {
            genealogy.get_tree_structure(root)
        } else {
            // 如果没有指定根，使用第一个事件 / If no root specified, use first event
            if let Some(first_event) = self.tracker.get_history().first() {
                genealogy.get_tree_structure(first_event.id)
            } else {
                serde_json::json!({})
            }
        }
    }

    /// 获取事件的祖先链 / Get ancestor chain of an event
    pub fn get_event_ancestors(&self, event_id: uuid::Uuid) -> Vec<uuid::Uuid> {
        self.tracker.get_ancestors(event_id)
    }

    /// 获取事件的后代 / Get descendants of an event
    pub fn get_event_descendants(&self, event_id: uuid::Uuid) -> Vec<uuid::Uuid> {
        self.tracker.get_descendants(event_id)
    }

    /// 从诗歌理解中学习并进化 / Learn and evolve from poetry understanding
    pub fn evolve_from_poetry(&mut self, poem: &str) -> Result<Vec<GrammarRule>, EvolutionError> {
        // 解析诗歌 / Parse poetry
        let analysis = self.poetry_parser.parse(poem).map_err(|e| {
            EvolutionError::IntegrationFailed(format!("Failed to parse poetry: {:?}", e))
        })?;

        // 从诗歌分析中提取知识并添加到知识图谱 / Extract knowledge from poetry analysis and add to knowledge graph
        let mut new_entities = Vec::new();
        let mut new_relations = Vec::new();

        // 添加情感作为知识节点 / Add emotions as knowledge nodes
        let emotion_entity = format!("emotion:{:?}", analysis.emotion_analysis.primary_emotion);
        new_entities.push(emotion_entity.clone());

        // 添加主题作为知识节点 / Add themes as knowledge nodes
        for theme in &analysis.themes {
            let theme_entity = format!("theme:{}", theme.name);
            new_entities.push(theme_entity.clone());
            
            // 情感与主题的关系 / Relation between emotion and theme
            new_relations.push(crate::evolution::knowledge::Relation {
                from: emotion_entity.clone(),
                to: theme_entity,
                relation_type: crate::evolution::knowledge::RelationType::Influences,
                weight: theme.confidence,
            });
        }

        // 添加意象作为知识节点 / Add imagery as knowledge nodes
        for img in &analysis.imagery {
            let imagery_entity = format!("imagery:{}", img.element);
            new_entities.push(imagery_entity.clone());
            
            // 意象与主题的关系 / Relation between imagery and themes
            for theme in &analysis.themes {
                if theme.confidence > 0.5 {
                    new_relations.push(crate::evolution::knowledge::Relation {
                        from: format!("theme:{}", theme.name),
                        to: imagery_entity.clone(),
                        relation_type: crate::evolution::knowledge::RelationType::Similar,
                        weight: img.frequency as f64 / 10.0,
                    });
                }
            }
        }

        // 将新知识添加到知识图谱 / Add new knowledge to knowledge graph
        self.knowledge_graph.add_entities_and_relations(&new_entities, &new_relations);

        // 基于诗歌理解生成可能的语法规则 / Generate possible grammar rules based on poetry understanding
        let generated_rules = self.generate_rules_from_poetry(&analysis)?;

        // 记录进化事件 / Record evolution event
        if !generated_rules.is_empty() {
            let event = EvolutionEvent {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                event_type: EvolutionType::SemanticEvolution,
                before_state: crate::evolution::tracker::StateSnapshot {
                    grammar_rules: self.syntax_mutations.clone(),
                    version: "0.1.0".to_string(),
                    metadata: serde_json::json!({}),
                },
                after_state: crate::evolution::tracker::StateSnapshot {
                    grammar_rules: {
                        let mut rules = self.syntax_mutations.clone();
                        rules.extend(generated_rules.iter().cloned());
                        rules
                    },
                    version: "0.1.2".to_string(),
                    metadata: serde_json::json!({
                        "poetry_analysis": serde_json::json!({
                            "primary_emotion": format!("{:?}", analysis.emotion_analysis.primary_emotion),
                            "themes": analysis.themes.iter().map(|t| t.name.clone()).collect::<Vec<_>>(),
                        }),
                    }),
                },
                delta: crate::evolution::tracker::EvolutionDelta {
                    added_rules: generated_rules.clone(),
                    modified_rules: Vec::new(),
                    removed_rules: Vec::new(),
                    description: format!(
                        "Evolution from poetry understanding: emotion {:?}, themes: {}",
                        analysis.emotion_analysis.primary_emotion,
                        analysis.themes.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", ")
                    ),
                },
                trigger: crate::evolution::tracker::TriggerContext {
                    source: TriggerSource::NaturalLanguageInstruction,
                    conditions: vec!["poetry_understanding".to_string()],
                    environment: serde_json::json!({
                        "poem_length": poem.len(),
                        "verses_count": analysis.verses.len(),
                    }),
                },
                author: None,
                success_metrics: None,
            };

            self.tracker.record(event.clone());
            self.knowledge_graph.build_from_history(&[event]);
            
            // 集成生成的规则 / Integrate generated rules
            for rule in &generated_rules {
                self.syntax_mutations.push(rule.clone());
            }
        }

        Ok(generated_rules)
    }

    /// 从诗歌分析生成规则 / Generate rules from poetry analysis
    fn generate_rules_from_poetry(&self, analysis: &crate::poetry::PoemAnalysis) -> Result<Vec<GrammarRule>, EvolutionError> {
        let mut rules = Vec::new();

        // 基于情感生成规则 / Generate rules based on emotion
        let emotion_name = format!("{:?}", analysis.emotion_analysis.primary_emotion).to_lowercase();
        
        // 为每种情感生成对应的语法规则 / Generate corresponding syntax rules for each emotion
        match analysis.emotion_analysis.primary_emotion {
            crate::poetry::emotion::Emotion::Nostalgia => {
                // 思乡情感 -> 生成思念相关的代码结构 / Nostalgia emotion -> generate code structures related to missing
                let rule = GrammarRule::new(
                    format!("poetry_emotion_{}", emotion_name),
                    Pattern {
                        elements: vec![PatternElement::NaturalLang("思乡".to_string())],
                        variadic: false,
                    },
                    Production {
                        target: GrammarElement::List(vec![
                            GrammarElement::Atom("def".to_string()),
                            GrammarElement::Atom("nostalgia".to_string()),
                            GrammarElement::List(vec![]),
                            GrammarElement::Expr(Box::new(crate::grammar::core::Expr::Literal(
                                crate::grammar::core::Literal::String("思念故乡".to_string())
                            ))),
                        ]),
                        transform: Vec::new(),
                        conditions: Vec::new(),
                    },
                    RuleMetadata {
                        version: "0.1.0".to_string(),
                        defined_by: DefinitionMethod::Evolutionary,
                        stability: Stability::Experimental,
                        description: format!("Generated from poetry understanding: {}", emotion_name),
                        examples: vec!["思乡".to_string()],
                        natural_lang_synonyms: vec!["思乡".to_string(), "怀念".to_string(), "思念".to_string()],
                    },
                );
                rules.push(rule);
            }
            crate::poetry::emotion::Emotion::Tranquility => {
                // 宁静情感 -> 生成平静相关的代码结构 / Tranquility emotion -> generate code structures related to peace
                let rule = GrammarRule::new(
                    format!("poetry_emotion_{}", emotion_name),
                    Pattern {
                        elements: vec![PatternElement::NaturalLang("宁静".to_string())],
                        variadic: false,
                    },
                    Production {
                        target: GrammarElement::List(vec![
                            GrammarElement::Atom("def".to_string()),
                            GrammarElement::Atom("tranquility".to_string()),
                            GrammarElement::List(vec![]),
                            GrammarElement::Expr(Box::new(crate::grammar::core::Expr::Literal(
                                crate::grammar::core::Literal::String("内心平静".to_string())
                            ))),
                        ]),
                        transform: Vec::new(),
                        conditions: Vec::new(),
                    },
                    RuleMetadata {
                        version: "0.1.0".to_string(),
                        defined_by: DefinitionMethod::Evolutionary,
                        stability: Stability::Experimental,
                        description: format!("Generated from poetry understanding: {}", emotion_name),
                        examples: vec!["宁静".to_string()],
                        natural_lang_synonyms: vec!["宁静".to_string(), "安静".to_string(), "平和".to_string()],
                    },
                );
                rules.push(rule);
            }
            _ => {}
        }

        // 基于主题生成规则 / Generate rules based on themes
        for theme in &analysis.themes {
            if theme.confidence > 0.7 {
                let rule = GrammarRule::new(
                    format!("poetry_theme_{}", theme.name),
                    Pattern {
                        elements: vec![PatternElement::NaturalLang(theme.name.clone())],
                        variadic: false,
                    },
                    Production {
                        target: GrammarElement::List(vec![
                            GrammarElement::Atom("def".to_string()),
                            GrammarElement::Atom(theme.name.to_lowercase()),
                            GrammarElement::List(vec![]),
                            GrammarElement::Expr(Box::new(crate::grammar::core::Expr::Literal(
                                crate::grammar::core::Literal::String(theme.description.clone())
                            ))),
                        ]),
                        transform: Vec::new(),
                        conditions: Vec::new(),
                    },
                    RuleMetadata {
                        version: "0.1.0".to_string(),
                        defined_by: DefinitionMethod::Evolutionary,
                        stability: Stability::Experimental,
                        description: format!("Generated from poetry theme: {}", theme.name),
                        examples: vec![theme.name.clone()],
                        natural_lang_synonyms: vec![theme.name.clone()],
                    },
                );
                rules.push(rule);
            }
        }

        // 基于意象生成代码结构 / Generate code structures based on imagery
        for img in &analysis.imagery {
            if img.frequency > 1 {
                // 高频意象 -> 生成对应的数据结构 / High-frequency imagery -> generate corresponding data structures
                let rule = GrammarRule::new(
                    format!("poetry_imagery_{}", img.element),
                    Pattern {
                        elements: vec![PatternElement::NaturalLang(img.element.clone())],
                        variadic: false,
                    },
                    Production {
                        target: GrammarElement::List(vec![
                            GrammarElement::Atom("dict".to_string()),
                            GrammarElement::Atom("\"element\"".to_string()),
                            GrammarElement::Expr(Box::new(crate::grammar::core::Expr::Literal(
                                crate::grammar::core::Literal::String(img.element.clone())
                            ))),
                            GrammarElement::Atom("\"meaning\"".to_string()),
                            GrammarElement::Expr(Box::new(crate::grammar::core::Expr::Literal(
                                crate::grammar::core::Literal::String(img.meaning.clone())
                            ))),
                        ]),
                        transform: Vec::new(),
                        conditions: Vec::new(),
                    },
                    RuleMetadata {
                        version: "0.1.0".to_string(),
                        defined_by: DefinitionMethod::Evolutionary,
                        stability: Stability::Experimental,
                        description: format!("Generated from poetry imagery: {}", img.element),
                        examples: vec![img.element.clone()],
                        natural_lang_synonyms: vec![img.element.clone()],
                    },
                );
                rules.push(rule);
            }
        }

        Ok(rules)
    }

    /// 从诗歌理解生成可执行代码 / Generate executable code from poetry understanding
    pub fn generate_code_from_poetry(&self, poem: &str) -> Result<String, EvolutionError> {
        // 解析诗歌 / Parse poetry
        let analysis = self.poetry_parser.parse(poem).map_err(|e| {
            EvolutionError::IntegrationFailed(format!("Failed to parse poetry: {:?}", e))
        })?;

        // 生成代码片段 / Generate code snippets
        let mut code_parts = Vec::new();

        // 基于情感生成代码 / Generate code based on emotion
        let emotion_code = match analysis.emotion_analysis.primary_emotion {
            crate::poetry::emotion::Emotion::Nostalgia => {
                format!("(def nostalgia () \"思念故乡的情感\")")
            }
            crate::poetry::emotion::Emotion::Tranquility => {
                format!("(def tranquility () \"夜晚的宁静，内心的平和\")")
            }
            crate::poetry::emotion::Emotion::Loneliness => {
                format!("(def loneliness () \"孤独感，缺少陪伴\")")
            }
            _ => String::new(),
        };
        if !emotion_code.is_empty() {
            code_parts.push(emotion_code);
        }

        // 基于主题生成代码 / Generate code based on themes
        for theme in &analysis.themes {
            if theme.confidence > 0.7 {
                code_parts.push(format!(
                    "(def {} () \"{}\")",
                    theme.name.to_lowercase(),
                    theme.description
                ));
            }
        }

        // 基于意象生成数据结构 / Generate data structures based on imagery
        for img in &analysis.imagery {
            if img.frequency > 0 {
                code_parts.push(format!(
                    "(let {} (dict \"element\" \"{}\" \"meaning\" \"{}\" \"frequency\" {}))",
                    img.element.to_lowercase(),
                    img.element,
                    img.meaning,
                    img.frequency
                ));
            }
        }

        Ok(code_parts.join("\n"))
    }

    /// 获取进化历史 / Get evolution history
    pub fn get_history(&self) -> &[EvolutionEvent] {
        self.tracker.get_history()
    }

    /// 获取语法规则 / Get syntax rules
    pub fn get_syntax_rules(&self) -> &[GrammarRule] {
        &self.syntax_mutations
    }

    /// 加载自举规则 / Load bootstrap rules
    fn load_bootstrap_rules() -> Vec<GrammarRule> {
        let code = "(import \"evolution\")\n(evolution.bootstrap_rules)";
        let mut interpreter = Interpreter::new();
        let parser = AdaptiveParser::new(true);
        let ast = match parser.parse(code) {
            Ok(ast) => ast,
            Err(_) => return Vec::new(),
        };
        let value = match interpreter.execute(&ast) {
            Ok(value) => value,
            Err(_) => return Vec::new(),
        };
        Self::rules_from_value_static(&value).unwrap_or_default()
    }

    /// 执行Aevolang代码 / Execute Aevolang code
    fn execute_aevo_code(&self, code: &str) -> Result<Value, EvolutionError> {
        let mut interpreter = Interpreter::new();
        let parser = AdaptiveParser::new(true);
        let ast = parser.parse(code).map_err(|e| {
            EvolutionError::IntegrationFailed(format!("Failed to parse aevo code: {:?}", e))
        })?;
        interpreter.execute(&ast).map_err(|e| {
            EvolutionError::IntegrationFailed(format!("Failed to execute aevo code: {:?}", e))
        })
    }

    /// Intent 转换为 Aevolang 字典文本 / Convert intent to Aevolang dict literal
    fn intent_to_aevo_dict(&self, intent: &crate::parser::nlu::ProgrammingIntent) -> String {
        let entities = self.list_to_aevo(&intent.entities);
        let params = self.pairs_to_aevo(&intent.parameters);
        let context = match &intent.context {
            Some(value) => self.string_to_aevo(value),
            None => "null".to_string(),
        };
        format!(
            "(dict \"action\" {} \"entities\" {} \"parameters\" {} \"context\" {})",
            self.string_to_aevo(&intent.action),
            entities,
            params,
            context
        )
    }

    fn string_to_aevo(&self, input: &str) -> String {
        let escaped = input.replace('\\', "\\\\").replace('"', "\\\"");
        format!("\"{}\"", escaped)
    }

    fn list_to_aevo(&self, items: &[String]) -> String {
        let values: Vec<String> = items.iter().map(|item| self.string_to_aevo(item)).collect();
        format!("(list {})", values.join(" "))
    }

    fn pairs_to_aevo(&self, pairs: &[(String, String)]) -> String {
        let values: Vec<String> = pairs
            .iter()
            .flat_map(|(k, v)| vec![self.string_to_aevo(k), self.string_to_aevo(v)])
            .collect();
        format!("(list {})", values.join(" "))
    }

    fn rules_from_value(&self, value: &Value) -> Result<Vec<GrammarRule>, EvolutionError> {
        Self::rules_from_value_static(value)
    }

    fn rules_from_value_static(value: &Value) -> Result<Vec<GrammarRule>, EvolutionError> {
        match value {
            Value::List(list) => list
                .iter()
                .map(Self::rule_from_value)
                .collect::<Result<Vec<_>, _>>(),
            _ => Err(EvolutionError::IntegrationFailed(
                "Expected a list of rules".to_string(),
            )),
        }
    }

    fn rule_from_value(value: &Value) -> Result<GrammarRule, EvolutionError> {
        let dict = match value {
            Value::Dict(map) => map,
            _ => {
                return Err(EvolutionError::IntegrationFailed(
                    "Rule must be a dict".to_string(),
                ))
            }
        };
        Self::rule_from_dict(dict)
    }

    fn rule_from_dict(dict: &HashMap<String, Value>) -> Result<GrammarRule, EvolutionError> {
        let name = Self::dict_string(dict, "name").unwrap_or_else(|| "unnamed".to_string());
        let production =
            Self::dict_string(dict, "production").unwrap_or_else(|| "Unknown".to_string());
        let description = Self::dict_string(dict, "description").unwrap_or_default();
        let variadic = Self::dict_bool(dict, "variadic").unwrap_or(false);
        let keywords = Self::dict_string_list(dict, "keywords");
        let examples = Self::dict_string_list(dict, "examples");
        let synonyms = Self::dict_string_list(dict, "synonyms");

        let elements = if keywords.is_empty() {
            vec![PatternElement::NaturalLang(name.clone())]
        } else {
            keywords
                .iter()
                .map(|keyword| PatternElement::NaturalLang(keyword.clone()))
                .collect()
        };

        let pattern = Pattern { elements, variadic };
        let production = Production {
            target: GrammarElement::Atom(production),
            transform: Vec::new(),
            conditions: Vec::new(),
        };
        let meta = RuleMetadata {
            version: "0.1.0".to_string(),
            defined_by: DefinitionMethod::Evolutionary,
            stability: Stability::Experimental,
            description,
            examples,
            natural_lang_synonyms: synonyms,
        };

        Ok(GrammarRule::new(name, pattern, production, meta))
    }

    fn dict_string(dict: &HashMap<String, Value>, key: &str) -> Option<String> {
        match dict.get(key) {
            Some(Value::String(value)) => Some(value.clone()),
            _ => None,
        }
    }

    fn dict_bool(dict: &HashMap<String, Value>, key: &str) -> Option<bool> {
        match dict.get(key) {
            Some(Value::Bool(value)) => Some(*value),
            _ => None,
        }
    }

    fn dict_string_list(dict: &HashMap<String, Value>, key: &str) -> Vec<String> {
        match dict.get(key) {
            Some(Value::List(items)) => items
                .iter()
                .filter_map(|item| match item {
                    Value::String(value) => Some(value.clone()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}

impl Default for EvolutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 进化错误 / Evolution error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvolutionError {
    /// NLU错误 / NLU error
    NLUError(String),
    /// 无变体 / No variants
    NoVariants,
    /// 测试失败 / Test failed
    TestFailed(String),
    /// 集成失败 / Integration failed
    IntegrationFailed(String),
}
