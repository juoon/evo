// 诗歌解析器 / Poetry parser
// 解析诗歌文本，提取情感和意境
// Parses poetry text, extracts emotions and artistic conception

use serde::{Deserialize, Serialize};
use crate::poetry::emotion::{Emotion, EmotionAnalysis};

/// 诗歌解析器 / Poetry parser
pub struct PoetryParser {
    /// 情感分析器 / Emotion analyzer
    emotion_analyzer: crate::poetry::emotion::EmotionAnalyzer,
}

impl PoetryParser {
    /// 创建新诗歌解析器 / Create new poetry parser
    pub fn new() -> Self {
        Self {
            emotion_analyzer: crate::poetry::emotion::EmotionAnalyzer::new(),
        }
    }

    /// 解析诗歌 / Parse poetry
    pub fn parse(&self, poem: &str) -> Result<PoemAnalysis, PoetryError> {
        // 提取诗句 / Extract verses
        let verses = self.extract_verses(poem);
        
        // 分析情感 / Analyze emotions
        let emotion_analysis = self.emotion_analyzer.analyze(poem)?;
        
        // 提取主题 / Extract themes
        let themes = self.extract_themes(&verses);
        
        // 提取意象 / Extract imagery
        let imagery = self.extract_imagery(&verses);

        Ok(PoemAnalysis {
            verses,
            emotion_analysis,
            themes,
            imagery,
        })
    }

    /// 提取诗句 / Extract verses
    fn extract_verses(&self, poem: &str) -> Vec<Verse> {
        poem.lines()
            .filter(|line| !line.trim().is_empty())
            .enumerate()
            .map(|(idx, line)| Verse {
                text: line.trim().to_string(),
                line_number: idx + 1,
            })
            .collect()
    }

    /// 提取主题 / Extract themes
    fn extract_themes(&self, verses: &[Verse]) -> Vec<Theme> {
        let mut themes = Vec::new();
        let text: String = verses.iter().map(|v| v.text.clone()).collect::<Vec<_>>().join("");
        
        // 主题关键词映射 / Theme keyword mapping
        let theme_patterns: Vec<(&str, Vec<&str>, &str)> = vec![
            ("思乡", vec!["思", "故乡"], "思念故乡的情感，表达对家乡的深切怀念"),
            ("孤独", vec!["孤独", "寂寞"], "孤独感，缺少陪伴和归属感"),
            ("宁静", vec!["静", "月", "夜"], "夜晚的宁静，内心的平和"),
            ("思念", vec!["思", "望"], "对远方或过去的人和事的思念"),
            ("离别", vec!["离", "别"], "与亲人朋友的分离"),
        ];
        
        for (theme_name, keywords, description) in theme_patterns.iter() {
            let matches: usize = keywords.iter()
                .map(|kw| text.matches(kw).count())
                .sum();
            
            if matches > 0 {
                let confidence = (matches as f64 / verses.len() as f64).min(1.0);
                themes.push(Theme {
                    name: theme_name.to_string(),
                    description: description.to_string(),
                    confidence,
                });
            }
        }
        
        // 如果没找到主题，基于情感分析推断 / If no theme found, infer from emotion analysis
        if themes.is_empty() {
            // 从诗句中提取情感关键词 / Extract emotion keywords from verses
            if text.contains("思") || text.contains("故乡") {
                themes.push(Theme {
                    name: "思乡".to_string(),
                    description: "思念故乡的情感".to_string(),
                    confidence: 0.7,
                });
            }
        }
        
        themes
    }

    /// 提取意象 / Extract imagery
    fn extract_imagery(&self, verses: &[Verse]) -> Vec<Imagery> {
        let text: String = verses.iter().map(|v| v.text.clone()).collect::<Vec<_>>().join("");
        
        // 意象元素词典 / Imagery element dictionary
        let imagery_dict: std::collections::HashMap<&str, &str> = [
            ("明月", "明亮的月光，象征思乡和团圆"),
            ("月", "月亮，常象征思念、孤独、美好"),
            ("光", "光芒，象征希望和指引"),
            ("霜", "霜花，比喻月光，营造清冷氛围"),
            ("地", "大地，代表现实世界"),
            ("床", "床铺，代表休息和私密空间"),
            ("头", "头部，代表思考和观察"),
            ("故乡", "家乡，代表思念和归属"),
        ]
        .iter()
        .cloned()
        .collect();
        
        let mut imagery_map: std::collections::HashMap<String, (String, usize)> = std::collections::HashMap::new();
        
        // 统计意象出现频率 / Count imagery frequency
        for (element, meaning) in imagery_dict.iter() {
            let count = text.matches(element).count();
            if count > 0 {
                // 处理子串匹配问题（如"明月"和"月"） / Handle substring matching issue
                if element == &"月" && text.contains("明月") {
                    // 如果已经有"明月"，跳过单独的"月" / Skip single "月" if "明月" exists
                    continue;
                }
                imagery_map.insert(element.to_string(), (meaning.to_string(), count));
            }
        }
        
        // 转换为Imagery列表 / Convert to Imagery list
        let mut imagery: Vec<Imagery> = imagery_map
            .into_iter()
            .map(|(element, (meaning, frequency))| Imagery {
                element,
                meaning,
                frequency,
            })
            .collect();
        
        // 按频率排序 / Sort by frequency
        imagery.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        
        imagery
    }
}

impl Default for PoetryParser {
    fn default() -> Self {
        Self::new()
    }
}

/// 诗歌分析结果 / Poem analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoemAnalysis {
    /// 诗句 / Verses
    pub verses: Vec<Verse>,
    /// 情感分析 / Emotion analysis
    pub emotion_analysis: EmotionAnalysis,
    /// 主题 / Themes
    pub themes: Vec<Theme>,
    /// 意象 / Imagery
    pub imagery: Vec<Imagery>,
}

/// 诗句 / Verse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verse {
    /// 文本 / Text
    pub text: String,
    /// 行号 / Line number
    pub line_number: usize,
}

/// 主题 / Theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// 主题名称 / Theme name
    pub name: String,
    /// 描述 / Description
    pub description: String,
    /// 置信度 / Confidence (0.0-1.0)
    pub confidence: f64,
}

/// 意象 / Imagery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Imagery {
    /// 意象元素 / Imagery element
    pub element: String,
    /// 含义 / Meaning
    pub meaning: String,
    /// 出现频率 / Frequency
    pub frequency: usize,
}

/// 诗歌错误 / Poetry error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoetryError {
    /// 解析错误 / Parse error
    ParseError(String),
    /// 情感分析错误 / Emotion analysis error
    EmotionAnalysisError(String),
}

impl From<crate::poetry::emotion::EmotionError> for PoetryError {
    fn from(err: crate::poetry::emotion::EmotionError) -> Self {
        match err {
            crate::poetry::emotion::EmotionError::AnalysisError(msg) => {
                PoetryError::EmotionAnalysisError(msg)
            }
        }
    }
}

