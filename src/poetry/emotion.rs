// 情感理解 / Emotion understanding
// 分析和理解文本中的情感
// Analyzes and understands emotions in text

use serde::{Deserialize, Serialize};

/// 情感分析器 / Emotion analyzer
pub struct EmotionAnalyzer {
    /// 情感词典 / Emotion dictionary
    emotion_dict: std::collections::HashMap<String, Emotion>,
}

impl EmotionAnalyzer {
    /// 创建新情感分析器 / Create new emotion analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            emotion_dict: std::collections::HashMap::new(),
        };
        analyzer.initialize_emotion_dict();
        analyzer
    }

    /// 初始化情感词典 / Initialize emotion dictionary
    fn initialize_emotion_dict(&mut self) {
        // 初始化基础情感词汇 / Initialize basic emotion vocabulary
        
        // 思乡 / Nostalgia - 《静夜思》核心情感
        self.emotion_dict.insert("思".to_string(), Emotion::Nostalgia);
        self.emotion_dict.insert("故乡".to_string(), Emotion::Nostalgia);
        self.emotion_dict.insert("家乡".to_string(), Emotion::Nostalgia);
        self.emotion_dict.insert("乡".to_string(), Emotion::Nostalgia);
        self.emotion_dict.insert("归".to_string(), Emotion::Nostalgia);
        
        // 孤独 / Loneliness
        self.emotion_dict.insert("孤独".to_string(), Emotion::Loneliness);
        self.emotion_dict.insert("寂寞".to_string(), Emotion::Loneliness);
        self.emotion_dict.insert("独".to_string(), Emotion::Loneliness);
        
        // 宁静 / Tranquility - 《静夜思》夜晚氛围
        self.emotion_dict.insert("静".to_string(), Emotion::Tranquility);
        self.emotion_dict.insert("安静".to_string(), Emotion::Tranquility);
        self.emotion_dict.insert("夜".to_string(), Emotion::Tranquility);
        
        // 忧伤 / Melancholy
        self.emotion_dict.insert("忧伤".to_string(), Emotion::Melancholy);
        self.emotion_dict.insert("愁".to_string(), Emotion::Melancholy);
        self.emotion_dict.insert("悲".to_string(), Emotion::Melancholy);
        self.emotion_dict.insert("哀".to_string(), Emotion::Melancholy);
        
        // 喜悦 / Joy
        self.emotion_dict.insert("喜".to_string(), Emotion::Joy);
        self.emotion_dict.insert("乐".to_string(), Emotion::Joy);
        self.emotion_dict.insert("欢".to_string(), Emotion::Joy);
        
        // 愤怒 / Anger
        self.emotion_dict.insert("怒".to_string(), Emotion::Anger);
        self.emotion_dict.insert("愤".to_string(), Emotion::Anger);
        
        // 惊讶 / Surprise
        self.emotion_dict.insert("惊".to_string(), Emotion::Surprise);
        self.emotion_dict.insert("疑".to_string(), Emotion::Surprise);
    }

    /// 分析情感 / Analyze emotions
    pub fn analyze(&self, text: &str) -> Result<EmotionAnalysis, EmotionError> {
        let mut detected_emotions = Vec::new();
        let mut emotion_scores = std::collections::HashMap::new();

        // 简单的关键词匹配 / Simple keyword matching
        for (keyword, emotion) in &self.emotion_dict {
            if text.contains(keyword) {
                let count = text.matches(keyword).count();
                let score = emotion_scores.entry(*emotion).or_insert(0.0);
                *score += count as f64 * 0.3; // 每个匹配增加0.3分 / Each match adds 0.3 points
                detected_emotions.push(*emotion);
            }
        }

        // 归一化分数 / Normalize scores
        let total_score: f64 = emotion_scores.values().sum();
        if total_score > 0.0 {
            for score in emotion_scores.values_mut() {
                *score /= total_score;
            }
        }

        // 确定主要情感 / Determine primary emotion
        let primary_emotion = emotion_scores.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(emotion, _)| *emotion)
            .unwrap_or(Emotion::Neutral);

        Ok(EmotionAnalysis {
            primary_emotion,
            emotion_scores,
            detected_emotions,
            confidence: if total_score > 0.0 { 0.8 } else { 0.3 },
        })
    }
}

impl Default for EmotionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// 情感类型 / Emotion type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Emotion {
    /// 中性 / Neutral
    Neutral,
    /// 思乡 / Nostalgia
    Nostalgia,
    /// 孤独 / Loneliness
    Loneliness,
    /// 宁静 / Tranquility
    Tranquility,
    /// 忧伤 / Melancholy
    Melancholy,
    /// 喜悦 / Joy
    Joy,
    /// 愤怒 / Anger
    Anger,
    /// 恐惧 / Fear
    Fear,
    /// 惊讶 / Surprise
    Surprise,
}

/// 情感分析结果 / Emotion analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionAnalysis {
    /// 主要情感 / Primary emotion
    pub primary_emotion: Emotion,
    /// 情感分数 / Emotion scores
    pub emotion_scores: std::collections::HashMap<Emotion, f64>,
    /// 检测到的情感 / Detected emotions
    pub detected_emotions: Vec<Emotion>,
    /// 置信度 / Confidence (0.0-1.0)
    pub confidence: f64,
}

/// 情感错误 / Emotion error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmotionError {
    /// 分析错误 / Analysis error
    AnalysisError(String),
}

