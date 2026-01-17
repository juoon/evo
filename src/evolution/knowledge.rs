// 进化知识图谱 / Evolution knowledge graph
// 构建和维护进化知识图谱，支持进化预测和学习
// Builds and maintains evolution knowledge graph, supports evolution prediction and learning

use crate::evolution::tracker::EvolutionEvent;
use serde::{Deserialize, Serialize};

/// 进化知识图谱 / Evolution knowledge graph
pub struct EvolutionKnowledgeGraph {
    /// 知识图谱数据 / Knowledge graph data
    graph: std::collections::HashMap<String, KnowledgeNode>,
    /// 模式挖掘器 / Pattern miner
    pattern_miner: PatternMiner,
}

impl EvolutionKnowledgeGraph {
    /// 创建新知识图谱 / Create new knowledge graph
    pub fn new() -> Self {
        Self {
            graph: std::collections::HashMap::new(),
            pattern_miner: PatternMiner::new(),
        }
    }

    /// 从历史构建知识 / Build knowledge from history
    pub fn build_from_history(&mut self, history: &[EvolutionEvent]) {
        for event in history {
            // 提取实体和关系 / Extract entities and relations
            let entities = self.extract_entities(event);
            let relations = self.extract_relations(event);

            // 添加到知识图谱 / Add to knowledge graph
            for entity in &entities {
                let node = self.graph.entry(entity.clone()).or_insert_with(|| {
                    let mut n = KnowledgeNode::new(entity.clone());
                    // 根据实体类型设置节点类型 / Set node type based on entity type
                    if entity.starts_with("rule:") {
                        n.node_type = NodeType::GrammarRule;
                    } else if entity.starts_with("trigger:") {
                        n.node_type = NodeType::Context;
                    }
                    n
                });
                node.update_from_event(event);
            }

            // 存储关系（简化：只记录在节点属性中） / Store relations (simplified: only in node attributes)
            for rel in &relations {
                if let Some(node) = self.graph.get_mut(&rel.from) {
                    let rel_json = serde_json::json!({
                        "to": rel.to,
                        "type": format!("{:?}", rel.relation_type),
                        "weight": rel.weight
                    });
                    let rels = node
                        .attributes
                        .entry("relations".to_string())
                        .or_insert_with(|| serde_json::json!([]));
                    if let Some(rels_array) = rels.as_array_mut() {
                        rels_array.push(rel_json);
                    }
                }
            }
        }

        // 挖掘模式 / Mine patterns after building graph
        let _ = self.pattern_miner.mine_from_graph(&self.graph);
    }

    /// 提取实体 / Extract entities
    fn extract_entities(&self, event: &EvolutionEvent) -> Vec<String> {
        let mut entities = vec![event.id.to_string()];

        // 从语法规则中提取实体 / Extract entities from grammar rules
        for rule in &event.delta.added_rules {
            entities.push(format!("rule:{}", rule.name));
            entities.push(format!("pattern:{}", rule.pattern.elements.len()));
            entities.push(format!(
                "production:{}",
                serde_json::to_string(&rule.production.target).unwrap_or_default()
            ));
        }

        // 从触发源提取实体 / Extract entities from trigger source
        match &event.trigger.source {
            crate::evolution::tracker::TriggerSource::NaturalLanguageInstruction => {
                entities.push("trigger:natural_language".to_string());
            }
            crate::evolution::tracker::TriggerSource::UserRequest => {
                entities.push("trigger:user".to_string());
            }
            crate::evolution::tracker::TriggerSource::UsagePatternAnalysis => {
                entities.push("trigger:usage_pattern".to_string());
            }
            _ => {}
        }

        entities
    }

    /// 提取关系 / Extract relations
    fn extract_relations(&self, event: &EvolutionEvent) -> Vec<Relation> {
        let mut relations = Vec::new();

        // 新规则与旧规则的关系 / Relations between new and old rules
        for new_rule in &event.delta.added_rules {
            for old_rule in &event.before_state.grammar_rules {
                // 计算相似度 / Calculate similarity
                let similarity = self.calculate_rule_similarity(old_rule, new_rule);
                if similarity > 0.3 {
                    relations.push(Relation {
                        from: format!("rule:{}", old_rule.name),
                        to: format!("rule:{}", new_rule.name),
                        relation_type: RelationType::EvolvedFrom,
                        weight: similarity,
                    });
                }
            }
        }

        // 事件之间的时间关系 / Temporal relations between events
        if let Some(prev_event_id) = self.graph.values().flat_map(|n| n.events.last()).next() {
            relations.push(Relation {
                from: prev_event_id.to_string(),
                to: event.id.to_string(),
                relation_type: RelationType::Influences,
                weight: 0.5,
            });
        }

        relations
    }

    /// 预测可能的进化 / Predict possible evolutions
    pub fn predict_evolutions(&self, context: &EvolutionContext) -> Vec<EvolutionPrediction> {
        let mut predictions = Vec::new();

        // 基于历史模式进行预测 / Predict based on historical patterns
        let patterns = self.pattern_miner.mine_from_graph_static(&self.graph);

        // 根据目标和约束匹配模式 / Match patterns based on goals and constraints
        for goal in &context.goals {
            for pattern in &patterns {
                if pattern.description.contains(goal) {
                    predictions.push(EvolutionPrediction {
                        predicted_evolution: format!("基于模式 '{}' 的进化", pattern.description),
                        confidence: pattern.confidence * 0.8,
                        reasoning: format!("目标 '{}' 与历史模式匹配", goal),
                    });
                }
            }
        }

        // 基于相似实体的预测 / Predict based on similar entities
        for (entity_id, node) in &self.graph {
            if node.events.len() > 1 {
                predictions.push(EvolutionPrediction {
                    predicted_evolution: format!("实体 '{}' 可能再次进化", entity_id),
                    confidence: 0.6,
                    reasoning: format!("该实体已有 {} 次进化历史", node.events.len()),
                });
            }
        }

        // 按置信度排序 / Sort by confidence
        predictions.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        predictions.truncate(5); // 返回前5个预测 / Return top 5 predictions

        predictions
    }

    /// 计算规则相似度 / Calculate rule similarity
    fn calculate_rule_similarity(
        &self,
        rule1: &crate::grammar::rule::GrammarRule,
        rule2: &crate::grammar::rule::GrammarRule,
    ) -> f64 {
        let mut similarity = 0.0;
        let mut factors = 0;

        // 名称相似度 / Name similarity
        if rule1.name == rule2.name {
            similarity += 1.0;
        } else if rule1.name.contains(&rule2.name) || rule2.name.contains(&rule1.name) {
            similarity += 0.6;
        } else {
            // 简单的编辑距离相似度 / Simple edit distance similarity
            let name_sim = self.string_similarity(&rule1.name, &rule2.name);
            similarity += name_sim * 0.4;
        }
        factors += 1;

        // 模式相似度 / Pattern similarity
        if rule1.pattern.elements.len() == rule2.pattern.elements.len() {
            let mut pattern_match = 0;
            for (e1, e2) in rule1
                .pattern
                .elements
                .iter()
                .zip(rule2.pattern.elements.iter())
            {
                if self.pattern_elements_similar(e1, e2) {
                    pattern_match += 1;
                }
            }
            if rule1.pattern.elements.len() > 0 {
                similarity += (pattern_match as f64 / rule1.pattern.elements.len() as f64) * 0.4;
            }
        }
        factors += 1;

        // 产生式相似度 / Production similarity
        let prod_sim = self.production_similarity(&rule1.production, &rule2.production);
        similarity += prod_sim * 0.2;
        factors += 1;

        // 归一化 / Normalize
        if factors > 0 {
            similarity / factors as f64
        } else {
            0.0
        }
    }

    /// 字符串相似度（简单的编辑距离） / String similarity (simple edit distance)
    fn string_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        // 简单的共同字符比例 / Simple common character ratio
        let common_chars: usize = s1.chars().filter(|c| s2.contains(*c)).count();
        let max_len = s1.len().max(s2.len());
        if max_len > 0 {
            (common_chars as f64 / max_len as f64).min(1.0)
        } else {
            0.0
        }
    }

    /// 模式元素相似度 / Pattern element similarity
    fn pattern_elements_similar(
        &self,
        e1: &crate::grammar::rule::PatternElement,
        e2: &crate::grammar::rule::PatternElement,
    ) -> bool {
        match (e1, e2) {
            (
                crate::grammar::rule::PatternElement::Keyword(k1),
                crate::grammar::rule::PatternElement::Keyword(k2),
            ) => k1 == k2,
            (
                crate::grammar::rule::PatternElement::Identifier(i1),
                crate::grammar::rule::PatternElement::Identifier(i2),
            ) => i1 == i2,
            (
                crate::grammar::rule::PatternElement::NaturalLang(n1),
                crate::grammar::rule::PatternElement::NaturalLang(n2),
            ) => n1 == n2 || self.string_similarity(n1, n2) > 0.7,
            _ => false,
        }
    }

    /// 产生式相似度 / Production similarity
    fn production_similarity(
        &self,
        p1: &crate::grammar::rule::Production,
        p2: &crate::grammar::rule::Production,
    ) -> f64 {
        // 简化：比较目标结构 / Simplified: compare target structure
        let t1 = serde_json::to_string(&p1.target).unwrap_or_default();
        let t2 = serde_json::to_string(&p2.target).unwrap_or_default();
        if t1 == t2 {
            1.0
        } else {
            self.string_similarity(&t1, &t2) * 0.5
        }
    }

    /// 查找相似实体 / Find similar entities
    pub fn find_similar_entities(&self, entity_id: &str, threshold: f64) -> Vec<(String, f64)> {
        let mut similar = Vec::new();

        if let Some(node) = self.graph.get(entity_id) {
            for (other_id, other_node) in &self.graph {
                if other_id != entity_id {
                    let similarity = self.calculate_node_similarity(node, other_node);
                    if similarity >= threshold {
                        similar.push((other_id.clone(), similarity));
                    }
                }
            }
        }

        similar.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similar
    }

    /// 计算节点相似度 / Calculate node similarity
    fn calculate_node_similarity(&self, node1: &KnowledgeNode, node2: &KnowledgeNode) -> f64 {
        let mut similarity = 0.0;
        let mut factors = 0;

        // 节点类型相似度 / Node type similarity
        if node1.node_type == node2.node_type {
            similarity += 0.3;
        }
        factors += 1;

        // ID相似度 / ID similarity
        let id_sim = self.string_similarity(&node1.id, &node2.id);
        similarity += id_sim * 0.4;
        factors += 1;

        // 事件重叠度 / Event overlap
        if !node1.events.is_empty() && !node2.events.is_empty() {
            let common_events: usize = node1
                .events
                .iter()
                .filter(|e| node2.events.contains(e))
                .count();
            let max_events = node1.events.len().max(node2.events.len());
            if max_events > 0 {
                similarity += (common_events as f64 / max_events as f64) * 0.3;
            }
        }
        factors += 1;

        if factors > 0 {
            similarity / factors as f64
        } else {
            0.0
        }
    }
}

impl EvolutionKnowledgeGraph {
    /// 获取节点数量 / Get node count
    pub fn get_node_count(&self) -> usize {
        self.graph.len()
    }

    /// 获取模式数量 / Get patterns count
    pub fn get_patterns_count(&self) -> usize {
        self.pattern_miner.patterns.len()
    }

    /// 添加实体和关系 / Add entities and relations
    pub fn add_entities_and_relations(&mut self, entities: &[String], relations: &[Relation]) {
        // 添加实体节点 / Add entity nodes
        for entity in entities {
            let node = self.graph.entry(entity.clone()).or_insert_with(|| {
                let mut n = KnowledgeNode::new(entity.clone());
                // 根据实体类型设置节点类型 / Set node type based on entity type
                if entity.starts_with("emotion:") {
                    n.node_type = NodeType::Concept;
                } else if entity.starts_with("theme:") {
                    n.node_type = NodeType::Concept;
                } else if entity.starts_with("imagery:") {
                    n.node_type = NodeType::Concept;
                }
                n
            });
            // 可以在这里更新节点属性 / Can update node attributes here
        }

        // 存储关系 / Store relations
        for rel in relations {
            if let Some(node) = self.graph.get_mut(&rel.from) {
                let rel_json = serde_json::json!({
                    "to": rel.to,
                    "type": format!("{:?}", rel.relation_type),
                    "weight": rel.weight
                });
                let rels = node
                    .attributes
                    .entry("relations".to_string())
                    .or_insert_with(|| serde_json::json!([]));
                if let Some(rels_array) = rels.as_array_mut() {
                    rels_array.push(rel_json);
                }
            }
        }
    }
}

impl Default for EvolutionKnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// 知识节点 / Knowledge node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    /// 节点ID / Node ID
    pub id: String,
    /// 节点类型 / Node type
    pub node_type: NodeType,
    /// 属性 / Attributes
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
    /// 关联的事件 / Associated events
    pub events: Vec<uuid::Uuid>,
}

impl KnowledgeNode {
    /// 创建新节点 / Create new node
    pub fn new(id: String) -> Self {
        Self {
            id,
            node_type: NodeType::Concept,
            attributes: std::collections::HashMap::new(),
            events: Vec::new(),
        }
    }

    /// 从事件更新 / Update from event
    pub fn update_from_event(&mut self, event: &EvolutionEvent) {
        self.events.push(event.id);
    }
}

/// 节点类型 / Node type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// 概念 / Concept
    Concept,
    /// 语法规则 / Grammar rule
    GrammarRule,
    /// 用户 / User
    User,
    /// 上下文 / Context
    Context,
}

/// 关系 / Relation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    /// 源节点 / Source node
    pub from: String,
    /// 目标节点 / Target node
    pub to: String,
    /// 关系类型 / Relation type
    pub relation_type: RelationType,
    /// 权重 / Weight
    pub weight: f64,
}

/// 关系类型 / Relation type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationType {
    /// 演变自 / Evolved from
    EvolvedFrom,
    /// 影响 / Influences
    Influences,
    /// 相似 / Similar,
    Similar,
    /// 冲突 / Conflicts,
    Conflicts,
}

/// 模式挖掘器 / Pattern miner
pub struct PatternMiner {
    /// 发现的模式 / Discovered patterns
    patterns: Vec<EvolutionPattern>,
}

impl PatternMiner {
    /// 创建新模式挖掘器 / Create new pattern miner
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// 挖掘模式 / Mine patterns
    pub fn mine(&mut self, events: &[EvolutionEvent]) -> Vec<EvolutionPattern> {
        let mut patterns = Vec::new();

        // 简单模式：频繁出现的语法规则进化 / Simple pattern: frequent grammar rule evolution
        let mut rule_counts: std::collections::HashMap<String, Vec<uuid::Uuid>> =
            std::collections::HashMap::new();
        for event in events {
            for rule in &event.delta.added_rules {
                rule_counts
                    .entry(rule.name.clone())
                    .or_insert_with(Vec::new)
                    .push(event.id);
            }
        }

        for (rule_name, event_ids) in rule_counts {
            if event_ids.len() > 1 {
                patterns.push(EvolutionPattern {
                    id: format!("pattern:{}", rule_name),
                    description: format!("规则 '{}' 多次进化", rule_name),
                    confidence: (event_ids.len() as f64 / events.len() as f64).min(1.0),
                    related_events: event_ids,
                });
            }
        }

        self.patterns = patterns.clone();
        patterns
    }

    /// 从知识图谱挖掘模式（非可变版本） / Mine patterns from knowledge graph (immutable version)
    pub fn mine_from_graph_static(
        &self,
        graph: &std::collections::HashMap<String, KnowledgeNode>,
    ) -> Vec<EvolutionPattern> {
        let mut patterns = Vec::new();

        // 查找频繁演变的实体 / Find frequently evolving entities
        for (entity_id, node) in graph {
            if node.events.len() > 2 {
                patterns.push(EvolutionPattern {
                    id: format!("pattern:{}", entity_id),
                    description: format!("实体 '{}' 经历多次进化", entity_id),
                    confidence: (node.events.len() as f64 / 10.0).min(1.0),
                    related_events: node.events.clone(),
                });
            }
        }

        patterns
    }

    /// 从知识图谱挖掘模式 / Mine patterns from knowledge graph
    pub fn mine_from_graph(
        &mut self,
        graph: &std::collections::HashMap<String, KnowledgeNode>,
    ) -> Vec<EvolutionPattern> {
        let patterns = self.mine_from_graph_static(graph);
        self.patterns = patterns.clone();
        patterns
    }
}

/// 进化模式 / Evolution pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPattern {
    /// 模式ID / Pattern ID
    pub id: String,
    /// 模式描述 / Pattern description
    pub description: String,
    /// 置信度 / Confidence
    pub confidence: f64,
    /// 相关事件 / Related events
    pub related_events: Vec<uuid::Uuid>,
}

/// 进化上下文 / Evolution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionContext {
    /// 当前状态 / Current state
    pub current_state: serde_json::Value,
    /// 目标 / Goals
    pub goals: Vec<String>,
    /// 约束 / Constraints
    pub constraints: Vec<String>,
}

/// 进化预测 / Evolution prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPrediction {
    /// 预测的进化 / Predicted evolution
    pub predicted_evolution: String,
    /// 置信度 / Confidence
    pub confidence: f64,
    /// 理由 / Reasoning
    pub reasoning: String,
}
