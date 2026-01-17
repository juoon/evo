// 自适应解析器 / Adaptive parser
// 能够根据扩展的语法规则动态调整解析行为
// Can dynamically adjust parsing behavior based on extended grammar rules

use crate::grammar::core::{BinOp, Expr, GrammarElement, Literal, Pattern};
use crate::grammar::rule::GrammarRule;

/// 自适应解析器 / Adaptive parser
pub struct AdaptiveParser {
    /// 语法规则列表 / List of grammar rules
    rules: Vec<GrammarRule>,
    /// 是否允许实验性语法 / Whether experimental syntax is allowed
    allow_experimental: bool,
}

impl AdaptiveParser {
    /// 创建新解析器 / Create new parser
    pub fn new(allow_experimental: bool) -> Self {
        Self {
            rules: Vec::new(),
            allow_experimental,
        }
    }

    /// 添加语法规则 / Add grammar rule
    pub fn add_rule(&mut self, rule: GrammarRule) {
        self.rules.push(rule);
    }

    /// 解析源代码 / Parse source code
    pub fn parse(&self, source: &str) -> Result<Vec<GrammarElement>, ParseError> {
        let mut tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.tokenize()?;
        let mut parser = ParserState::new(tokens);
        parser.parse_all()
    }

    /// 检查未知语法 / Check for unknown syntax
    pub fn found_unknown_syntax(&self, ast: &[GrammarElement]) -> bool {
        // 检查是否有未识别的语法元素
        for element in ast {
            if let GrammarElement::Atom(ref atom) = element {
                // 检查是否是未知的关键字或操作符
                if self.is_unknown_keyword(atom) {
                    return true;
                }
            }
        }
        false
    }

    /// 检查是否是未知关键字 / Check if unknown keyword
    fn is_unknown_keyword(&self, atom: &str) -> bool {
        // 基础关键字列表
        let known_keywords = ["def", "let", "if", "then", "else", "function", "return"];
        !known_keywords.contains(&atom)
            && !atom.parse::<i64>().is_ok()
            && !atom.parse::<f64>().is_ok()
    }

    /// 提议语法扩展 / Propose syntax expansion
    pub fn propose_syntax_expansion(&self, _ast: &[GrammarElement]) -> Vec<GrammarRule> {
        // TODO: 实现语法扩展提议 / Implement syntax expansion proposal
        Vec::new()
    }
}

/// 词法分析器 / Tokenizer
struct Tokenizer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    LeftParen,       // (
    RightParen,      // )
    String(String),  // "string"
    Number(String),  // 数字（整数或浮点数）
    Symbol(String),  // 标识符或关键字
    Quote,           // '
    Comment(String), // ; 注释
    EOF,
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let token = self.next_token()?;
            match token {
                Token::Comment(_) => {
                    // 跳过注释
                    continue;
                }
                _ => tokens.push(token),
            }
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        let ch = self.peek();

        match ch {
            '(' => {
                self.advance();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParen)
            }
            '\'' => {
                self.advance();
                Ok(Token::Quote)
            }
            '"' => {
                self.advance();
                self.read_string()
            }
            ';' => {
                self.advance();
                self.read_comment()
            }
            '-' | '+' => {
                if self.position + 1 < self.input.len()
                    && self.input[self.position + 1].is_ascii_digit()
                {
                    let ch = self.advance();
                    self.read_number(Some(ch))
                } else {
                    Ok(Token::Symbol(self.advance().to_string()))
                }
            }
            _ if ch.is_ascii_digit() => self.read_number(None),
            _ if ch.is_alphabetic() || ch == '_' => self.read_symbol(None),
            _ if ch == '*' || ch == '/' => {
                // 处理乘法和除法操作符
                Ok(Token::Symbol(self.advance().to_string()))
            }
            _ if ch == '>' || ch == '<' || ch == '=' || ch == '!' => {
                // 处理比较操作符
                let op = self.advance().to_string();
                if (ch == '>' || ch == '<' || ch == '=' || ch == '!') && self.peek() == '=' {
                    let op = format!("{}{}", op, self.advance());
                    Ok(Token::Symbol(op))
                } else {
                    Ok(Token::Symbol(op))
                }
            }
            _ => {
                let ch = self.advance();
                let location = Location::new(self.line, self.column);
                Err(ParseError::syntax_error(
                    format!("Unexpected character '{}'", ch),
                    Some(location),
                ))
            }
        }
    }

    fn read_string(&mut self) -> Result<Token, ParseError> {
        let mut string = String::new();
        let start_line = self.line;
        let start_column = self.column;

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance(); // 跳过反斜杠
                if self.is_at_end() {
                    let location = Location::new(self.line, self.column);
                    return Err(ParseError::syntax_error(
                        "Unterminated escape sequence".to_string(),
                        Some(location),
                    ));
                }
                match self.advance() {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '\\' => string.push('\\'),
                    '"' => string.push('"'),
                    ch => {
                        return Err(ParseError::SyntaxError(format!(
                            "Invalid escape sequence '\\{}' at line {}, column {}",
                            ch, self.line, self.column
                        )))
                    }
                }
            } else {
                string.push(self.advance());
            }
        }

        if self.is_at_end() {
            let location = Location::new(start_line, start_column);
            return Err(ParseError::syntax_error(
                "Unterminated string".to_string(),
                Some(location),
            ));
        }

        self.advance(); // 跳过结束引号
        Ok(Token::String(string))
    }

    fn read_number(&mut self, first_char: Option<char>) -> Result<Token, ParseError> {
        let mut number = String::new();
        if let Some(ch) = first_char {
            number.push(ch);
        }

        while self.peek().is_ascii_digit() {
            number.push(self.advance());
        }

        // 处理浮点数
        if self.peek() == '.' {
            number.push(self.advance());
            while self.peek().is_ascii_digit() {
                number.push(self.advance());
            }
        }

        Ok(Token::Number(number))
    }

    fn read_symbol(&mut self, first_char: Option<char>) -> Result<Token, ParseError> {
        let mut symbol = String::new();
        if let Some(ch) = first_char {
            symbol.push(ch);
        } else {
            symbol.push(self.advance());
        }

        while !self.is_at_end()
            && (self.peek().is_alphanumeric()
                || self.peek() == '_'
                || self.peek() == '-'
                || self.peek() == '?')
        {
            symbol.push(self.advance());
        }

        Ok(Token::Symbol(symbol))
    }

    fn read_comment(&mut self) -> Result<Token, ParseError> {
        let mut comment = String::new();
        while !self.is_at_end() && self.peek() != '\n' {
            comment.push(self.advance());
        }
        Ok(Token::Comment(comment))
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.input[self.position];
        self.position += 1;
        self.column += 1;
        ch
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

/// 解析器状态 / Parser state
struct ParserState {
    tokens: Vec<Token>,
    current: usize,
}

impl ParserState {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse_all(&mut self) -> Result<Vec<GrammarElement>, ParseError> {
        let mut elements = Vec::new();

        while !self.is_at_end() {
            if self.check(&Token::EOF) {
                break;
            }
            elements.push(self.parse_element()?);
        }

        Ok(elements)
    }

    fn parse_element(&mut self) -> Result<GrammarElement, ParseError> {
        match self.peek() {
            Token::LeftParen => self.parse_list(),
            Token::Quote => self.parse_quoted(),
            Token::String(_) => self.parse_string(),
            Token::Number(_) => self.parse_number(),
            Token::Symbol(_) => self.parse_symbol(),
            _ => Err(ParseError::syntax_error(
                format!("Unexpected token: {:?}", self.peek()),
                None, // ParserState没有位置信息，需要从Token中获取
            )),
        }
    }

    fn parse_list(&mut self) -> Result<GrammarElement, ParseError> {
        self.consume(&Token::LeftParen, "Expected '('")?;

        if self.check(&Token::RightParen) {
            self.advance_token();
            return Ok(GrammarElement::List(Vec::new()));
        }

        let first = self.parse_element()?;

        // 检查是否是特殊形式（如 def, let, if, list, dict 等）
        if let GrammarElement::Atom(ref atom) = &first {
            match atom.as_str() {
                "def" | "function" => {
                    return self.parse_function_def(atom.clone());
                }
                "let" => {
                    return self.parse_let();
                }
                "if" => {
                    return self.parse_if();
                }
                "lambda" => {
                    return self.parse_lambda();
                }
                "match" => {
                    return self.parse_match();
                }
                "list" | "vec" => {
                    return self.parse_list_literal();
                }
                "dict" | "map" => {
                    return self.parse_dict_literal();
                }
                _ => {
                    // 函数调用
                    let func_name = atom.clone();
                    let mut args = Vec::new();
                    while !self.check(&Token::RightParen) {
                        args.push(self.parse_element()?);
                    }
                    self.consume(&Token::RightParen, "Expected ')'")?;

                    // 转换为函数调用表达式
                    let expr_args: Vec<Expr> = args
                        .iter()
                        .map(|e| self.element_to_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(GrammarElement::Expr(Box::new(Expr::Call(
                        func_name, expr_args,
                    ))));
                }
            }
        }

        // 普通列表
        let mut elements = vec![first];
        while !self.check(&Token::RightParen) {
            elements.push(self.parse_element()?);
        }
        self.consume(&Token::RightParen, "Expected ')'")?;

        Ok(GrammarElement::List(elements))
    }

    fn parse_function_def(&mut self, keyword: String) -> Result<GrammarElement, ParseError> {
        // (def name (args...) body)
        // 或 (function name (args...) body)
        let name = self.parse_element()?;
        let name_str = match name {
            GrammarElement::Atom(s) => s,
            _ => {
                return Err(ParseError::syntax_error(
                    "Function name must be an atom".to_string(),
                    None,
                ))
            }
        };

        // 解析参数列表
        let args = if self.check(&Token::LeftParen) {
            self.parse_list()?
        } else {
            GrammarElement::List(Vec::new())
        };

        let args_list = match args {
            GrammarElement::List(l) => l,
            _ => Vec::new(),
        };

        let arg_names: Vec<String> = args_list
            .iter()
            .filter_map(|e| {
                if let GrammarElement::Atom(s) = e {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        // 解析函数体
        let body = self.parse_element()?;

        // 消费结束括号
        self.consume(&Token::RightParen, "Expected ')' after function definition")?;

        // 转换为表达式（这里简化处理，实际应该创建函数定义节点）
        Ok(GrammarElement::List(vec![
            GrammarElement::Atom(keyword),
            GrammarElement::Atom(name_str),
            GrammarElement::List(args_list),
            body,
        ]))
    }

    fn parse_let(&mut self) -> Result<GrammarElement, ParseError> {
        // (let name value body)
        let name = self.parse_element()?;
        let value = self.parse_element()?;
        let body = self.parse_element()?;

        // 消费结束括号
        self.consume(&Token::RightParen, "Expected ')' after let expression")?;

        Ok(GrammarElement::List(vec![
            GrammarElement::Atom("let".to_string()),
            name,
            value,
            body,
        ]))
    }

    fn parse_if(&mut self) -> Result<GrammarElement, ParseError> {
        // (if condition then_expr else_expr)
        let condition = self.parse_element()?;
        let then_expr = self.parse_element()?;
        let else_expr = if !self.check(&Token::RightParen) {
            self.parse_element()?
        } else {
            GrammarElement::Expr(Box::new(Expr::Literal(Literal::Null)))
        };

        // 消费结束括号
        self.consume(&Token::RightParen, "Expected ')' after if expression")?;

        let cond_expr = self.element_to_expr(&condition)?;
        let then_expr_parsed = self.element_to_expr(&then_expr)?;
        let else_expr_parsed = self.element_to_expr(&else_expr)?;

        Ok(GrammarElement::Expr(Box::new(Expr::If(
            Box::new(cond_expr),
            Box::new(then_expr_parsed),
            Box::new(else_expr_parsed),
        ))))
    }

    fn parse_lambda(&mut self) -> Result<GrammarElement, ParseError> {
        // (lambda (params...) body)
        // 解析参数列表
        let args = if self.check(&Token::LeftParen) {
            self.parse_list()?
        } else {
            GrammarElement::List(Vec::new())
        };

        let args_list = match args {
            GrammarElement::List(l) => l,
            _ => Vec::new(),
        };

        // 解析函数体
        let body = self.parse_element()?;

        // 消费结束括号
        self.consume(&Token::RightParen, "Expected ')' after lambda expression")?;

        // 转换为lambda列表格式，供解释器使用
        Ok(GrammarElement::List(vec![
            GrammarElement::Atom("lambda".to_string()),
            GrammarElement::List(args_list),
            body,
        ]))
    }

    fn parse_match(&mut self) -> Result<GrammarElement, ParseError> {
        // (match value (pattern1 expr1) (pattern2 expr2) ...)
        let value_elem = self.parse_element()?;
        let value_expr = self.element_to_expr(&value_elem)?;

        let mut cases = Vec::new();
        while !self.check(&Token::RightParen) {
            // 每个case是一个列表: (pattern expr)
            if !self.check(&Token::LeftParen) {
                return Err(ParseError::SyntaxError(
                    "Expected '(' for match case".to_string(),
                ));
            }
            self.consume(&Token::LeftParen, "Expected '(' for match case")?;

            // 解析模式
            let pattern_elem = self.parse_element()?;
            let pattern = self.element_to_pattern(&pattern_elem)?;

            // 解析表达式
            let expr_elem = self.parse_element()?;
            let expr = self.element_to_expr(&expr_elem)?;

            self.consume(&Token::RightParen, "Expected ')' after match case")?;

            cases.push((pattern, expr));
        }

        self.consume(&Token::RightParen, "Expected ')' after match expression")?;

        Ok(GrammarElement::Expr(Box::new(Expr::Match(
            Box::new(value_expr),
            cases,
        ))))
    }

    fn element_to_pattern(&self, elem: &GrammarElement) -> Result<Pattern, ParseError> {
        use crate::grammar::core::Pattern::*;
        match elem {
            GrammarElement::Atom(s) => {
                if s == "_" {
                    Ok(Wildcard)
                } else {
                    Ok(Var(s.clone()))
                }
            }
            GrammarElement::Expr(boxed_expr) => match boxed_expr.as_ref() {
                Expr::Literal(lit) => Ok(Literal(lit.clone())),
                Expr::Var(name) => Ok(Var(name.clone())),
                _ => Err(ParseError::SyntaxError(
                    "Invalid pattern in match expression".to_string(),
                )),
            },
            GrammarElement::List(list) => {
                let mut patterns = Vec::new();
                for item in list {
                    patterns.push(self.element_to_pattern(item)?);
                }
                Ok(List(patterns))
            }
            _ => Err(ParseError::SyntaxError(
                "Invalid pattern in match expression".to_string(),
            )),
        }
    }

    fn parse_list_literal(&mut self) -> Result<GrammarElement, ParseError> {
        // (list item1 item2 ...) 或 (vec item1 item2 ...)
        let mut items = Vec::new();
        while !self.check(&Token::RightParen) {
            items.push(self.parse_element()?);
        }
        self.consume(&Token::RightParen, "Expected ')' after list literal")?;

        // 转换为表达式列表
        let expr_items: Vec<Expr> = items
            .iter()
            .map(|e| self.element_to_expr(e))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(GrammarElement::Expr(Box::new(Expr::Literal(
            Literal::List(expr_items),
        ))))
    }

    fn parse_dict_literal(&mut self) -> Result<GrammarElement, ParseError> {
        // (dict key1 value1 key2 value2 ...) 或 (map key1 value1 key2 value2 ...)
        let mut pairs = Vec::new();
        while !self.check(&Token::RightParen) {
            let key_elem = self.parse_element()?;
            let value_elem = if !self.check(&Token::RightParen) {
                self.parse_element()?
            } else {
                return Err(ParseError::SyntaxError(
                    "Dictionary requires key-value pairs".to_string(),
                ));
            };

            // 提取键（必须是字符串或标识符）
            let key = match &key_elem {
                GrammarElement::Atom(s) => s.clone(),
                GrammarElement::Expr(boxed_expr) => {
                    if let Expr::Literal(Literal::String(s)) = boxed_expr.as_ref() {
                        s.clone()
                    } else if let Expr::Var(s) = boxed_expr.as_ref() {
                        s.clone()
                    } else {
                        return Err(ParseError::SyntaxError(
                            "Dictionary key must be a string or identifier".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(ParseError::SyntaxError(
                        "Dictionary key must be a string or identifier".to_string(),
                    ));
                }
            };

            let value_expr = self.element_to_expr(&value_elem)?;
            pairs.push((key, value_expr));
        }
        self.consume(&Token::RightParen, "Expected ')' after dict literal")?;

        Ok(GrammarElement::Expr(Box::new(Expr::Literal(
            Literal::Dict(pairs),
        ))))
    }

    fn parse_quoted(&mut self) -> Result<GrammarElement, ParseError> {
        self.consume(&Token::Quote, "Expected quote")?;
        let element = self.parse_element()?;
        Ok(element)
    }

    fn parse_string(&mut self) -> Result<GrammarElement, ParseError> {
        match self.advance_token() {
            Token::String(s) => Ok(GrammarElement::Expr(Box::new(Expr::Literal(
                Literal::String(s),
            )))),
            _ => Err(ParseError::SyntaxError("Expected string".to_string())),
        }
    }

    fn parse_number(&mut self) -> Result<GrammarElement, ParseError> {
        match self.advance_token() {
            Token::Number(n) => {
                // 尝试解析为整数或浮点数
                if n.contains('.') {
                    n.parse::<f64>()
                        .map(|f| GrammarElement::Expr(Box::new(Expr::Literal(Literal::Float(f)))))
                        .map_err(|_| ParseError::SyntaxError(format!("Invalid float: {}", n)))
                } else {
                    n.parse::<i64>()
                        .map(|i| GrammarElement::Expr(Box::new(Expr::Literal(Literal::Int(i)))))
                        .map_err(|_| ParseError::SyntaxError(format!("Invalid integer: {}", n)))
                }
            }
            _ => Err(ParseError::SyntaxError("Expected number".to_string())),
        }
    }

    fn parse_symbol(&mut self) -> Result<GrammarElement, ParseError> {
        match self.advance_token() {
            Token::Symbol(s) => {
                // 检查是否是布尔值或特殊值
                match s.as_str() {
                    "true" => Ok(GrammarElement::Expr(Box::new(Expr::Literal(
                        Literal::Bool(true),
                    )))),
                    "false" => Ok(GrammarElement::Expr(Box::new(Expr::Literal(
                        Literal::Bool(false),
                    )))),
                    "null" | "nil" => {
                        Ok(GrammarElement::Expr(Box::new(Expr::Literal(Literal::Null))))
                    }
                    _ => {
                        // 检查是否是操作符
                        if self.parse_binop(&s).is_some() {
                            Ok(GrammarElement::Atom(format!("op:{}", s)))
                        } else {
                            Ok(GrammarElement::Expr(Box::new(Expr::Var(s))))
                        }
                    }
                }
            }
            _ => Err(ParseError::SyntaxError("Expected symbol".to_string())),
        }
    }

    fn parse_binop(&self, op: &str) -> Option<BinOp> {
        match op {
            "+" => Some(BinOp::Add),
            "-" => Some(BinOp::Sub),
            "*" => Some(BinOp::Mul),
            "/" => Some(BinOp::Div),
            "=" | "==" => Some(BinOp::Eq),
            "!=" | "<>" => Some(BinOp::Ne),
            "<" => Some(BinOp::Lt),
            ">" => Some(BinOp::Gt),
            "<=" => Some(BinOp::Le),
            ">=" => Some(BinOp::Ge),
            _ => None,
        }
    }

    fn element_to_expr(&self, element: &GrammarElement) -> Result<Expr, ParseError> {
        match element {
            GrammarElement::Expr(e) => Ok(*e.clone()),
            GrammarElement::Atom(s) => {
                // 尝试解析为变量或操作符
                if s.starts_with("op:") {
                    Err(ParseError::SyntaxError(
                        "Operator in wrong context".to_string(),
                    ))
                } else {
                    Ok(Expr::Var(s.clone()))
                }
            }
            GrammarElement::List(l) => {
                if l.is_empty() {
                    Ok(Expr::Literal(Literal::Null))
                } else {
                    // 获取函数名（支持 Atom 和 Expr(Var(...)) 两种形式）
                    let func_name = match &l[0] {
                        GrammarElement::Atom(s) => s.clone(),
                        GrammarElement::Expr(boxed_expr) => {
                            if let Expr::Var(s) = boxed_expr.as_ref() {
                                s.clone()
                            } else {
                                return Err(ParseError::SyntaxError(
                                    "Function name must be an atom or variable".to_string(),
                                ));
                            }
                        }
                        _ => {
                            return Err(ParseError::SyntaxError(
                                "Function name must be an atom or variable".to_string(),
                            ));
                        }
                    };
                    // 函数调用
                    let args: Vec<Expr> = l[1..]
                        .iter()
                        .map(|e| self.element_to_expr(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(Expr::Call(func_name, args))
                }
            }
            GrammarElement::NaturalLang(_) => Err(ParseError::SyntaxError(
                "Natural language not supported in expressions".to_string(),
            )),
        }
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            match (token, &self.tokens[self.current]) {
                (Token::String(_), Token::String(_)) => true,
                (Token::Number(_), Token::Number(_)) => true,
                (Token::Symbol(_), Token::Symbol(_)) => true,
                (a, b) => a == b,
            }
        }
    }

    fn advance_token(&mut self) -> Token {
        if !self.is_at_end() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            token
        } else {
            Token::EOF
        }
    }

    fn consume(&mut self, token: &Token, message: &str) -> Result<(), ParseError> {
        if self.check(token) {
            self.advance_token();
            Ok(())
        } else {
            Err(ParseError::syntax_error(
                format!("{}: expected {:?}, got {:?}", message, token, self.peek()),
                None,
            ))
        }
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            &Token::EOF
        } else {
            &self.tokens[self.current]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.tokens[self.current], Token::EOF)
    }
}

/// 解析错误 / Parse error
#[derive(Debug, Clone, PartialEq, Eq)]
/// 源代码位置 / Source code location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// 行号（从1开始）/ Line number (1-based)
    pub line: usize,
    /// 列号（从1开始）/ Column number (1-based)
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn format(&self) -> String {
        format!("line {}, column {}", self.line, self.column)
    }
}

pub enum ParseError {
    /// 未实现 / Not implemented
    NotImplemented,
    /// 语法错误 / Syntax error
    SyntaxError {
        message: String,
        location: Option<Location>,
    },
    /// 未知语法 / Unknown syntax
    UnknownSyntax {
        message: String,
        location: Option<Location>,
    },
    /// 规则冲突 / Rule conflict
    RuleConflict {
        message: String,
        location: Option<Location>,
    },
}

impl ParseError {
    /// 创建语法错误 / Create syntax error
    pub fn syntax_error(message: String, location: Option<Location>) -> Self {
        Self::SyntaxError { message, location }
    }

    /// 创建未知语法错误 / Create unknown syntax error
    pub fn unknown_syntax(message: String, location: Option<Location>) -> Self {
        Self::UnknownSyntax { message, location }
    }

    /// 获取错误消息 / Get error message
    pub fn message(&self) -> &str {
        match self {
            Self::NotImplemented => "Not implemented",
            Self::SyntaxError { message, .. } => message,
            Self::UnknownSyntax { message, .. } => message,
            Self::RuleConflict { message, .. } => message,
        }
    }

    /// 获取位置信息 / Get location
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::SyntaxError { location, .. } => *location,
            Self::UnknownSyntax { location, .. } => *location,
            Self::RuleConflict { location, .. } => *location,
            _ => None,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented => write!(f, "Not implemented"),
            Self::SyntaxError { message, location } => {
                if let Some(loc) = location {
                    write!(f, "Syntax error at {}: {}", loc.format(), message)
                } else {
                    write!(f, "Syntax error: {}", message)
                }
            }
            Self::UnknownSyntax { message, location } => {
                if let Some(loc) = location {
                    write!(f, "Unknown syntax at {}: {}", loc.format(), message)
                } else {
                    write!(f, "Unknown syntax: {}", message)
                }
            }
            Self::RuleConflict { message, location } => {
                if let Some(loc) = location {
                    write!(f, "Rule conflict at {}: {}", loc.format(), message)
                } else {
                    write!(f, "Rule conflict: {}", message)
                }
            }
        }
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
