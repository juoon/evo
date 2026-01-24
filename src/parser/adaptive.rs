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
            _ if ch.is_alphabetic() || ch == '_' || ch == '.' => self.read_symbol(None),
            _ if ch == '*' || ch == '/' || ch == '%' => {
                // 处理乘法、除法和取模操作符
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
                        return Err(ParseError::syntax_error(
                            format!(
                                "Invalid escape sequence '\\{}' at line {}, column {}",
                                ch, self.line, self.column
                            ),
                            Some(Location::new(self.line, self.column)),
                        ))
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
                || self.peek() == '?'
                || self.peek() == '.')
        {
            symbol.push(self.advance());
        }

        // 允许 ! 作为标识符的结尾（如 set!, def! 等）
        // Allow ! at the end of identifiers (e.g., set!, def!)
        if !self.is_at_end() && self.peek() == '!' {
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
        // 支持 Atom 和 Expr(Var(...)) 两种形式
        let keyword: Option<&str> = match &first {
            GrammarElement::Atom(s) => Some(s.as_str()),
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    Some(s.as_str())
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(keyword) = keyword {
            match keyword {
                "def" | "function" => {
                    return self.parse_function_def(keyword.to_string());
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
                "for" => {
                    return self.parse_for();
                }
                "while" => {
                    return self.parse_while();
                }
                "try" => {
                    return self.parse_try();
                }
                "list" | "vec" => {
                    return self.parse_list_literal();
                }
                "dict" | "map" => {
                    return self.parse_dict_literal();
                }
                "set!" => {
                    // (set! var value)
                    let var_elem = self.parse_element()?;
                    let var_str = match &var_elem {
                        GrammarElement::Atom(s) => s.clone(),
                        GrammarElement::Expr(boxed_expr) => {
                            if let Expr::Var(s) = boxed_expr.as_ref() {
                                s.clone()
                            } else {
                                return Err(ParseError::syntax_error(
                                    "set! variable must be an atom or variable".to_string(),
                                    None,
                                ));
                            }
                        }
                        _ => {
                            return Err(ParseError::syntax_error(
                                "set! variable must be an atom or variable".to_string(),
                                None,
                            ));
                        }
                    };

                    let value_elem = self.parse_element()?;
                    self.consume(&Token::RightParen, "Expected ')' after set! expression")?;

                    let value_expr = self.element_to_expr(&value_elem)?;
                    return Ok(GrammarElement::Expr(Box::new(Expr::Assign(
                        var_str,
                        Box::new(value_expr),
                    ))));
                }
                _ => {
                    // 函数调用
                    let func_name = keyword.to_string();
                    let mut args = Vec::new();
                    while !self.check(&Token::RightParen) {
                        args.push(self.parse_element()?);
                    }
                    self.consume(&Token::RightParen, "Expected ')'")?;

                    // 转换为函数调用表达式
                    // 检查参数中是否包含 lambda 表达式
                    // Check if arguments contain lambda expressions
                    let has_lambda = args.iter().any(|e| {
                        if let GrammarElement::List(l) = e {
                            !l.is_empty()
                                && match &l[0] {
                                    GrammarElement::Atom(s) => s == "lambda",
                                    GrammarElement::Expr(boxed_expr) => {
                                        if let Expr::Var(s) = boxed_expr.as_ref() {
                                            s == "lambda"
                                        } else {
                                            false
                                        }
                                    }
                                    _ => false,
                                }
                        } else {
                            false
                        }
                    });

                    if has_lambda {
                        // 包含 lambda 表达式，不能转换为 Expr::Call，保持为 GrammarElement::List
                        // Contains lambda expressions, cannot convert to Expr::Call, keep as GrammarElement::List
                        let mut elements = vec![GrammarElement::Atom(func_name)];
                        for arg in args {
                            elements.push(arg);
                        }
                        return Ok(GrammarElement::List(elements));
                    }

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
            GrammarElement::Expr(boxed_expr) => {
                if let crate::grammar::core::Expr::Var(s) = boxed_expr.as_ref() {
                    s.clone()
                } else {
                    return Err(ParseError::syntax_error(
                        "Function name must be an atom or variable".to_string(),
                        None,
                    ));
                }
            }
            _ => {
                return Err(ParseError::syntax_error(
                    "Function name must be an atom or variable".to_string(),
                    None,
                ))
            }
        };

        // 解析参数列表（直接解析，不进行关键字检查）
        let args_list = if self.check(&Token::LeftParen) {
            self.consume(&Token::LeftParen, "Expected '(' for parameter list")?;
            let mut params = Vec::new();
            while !self.check(&Token::RightParen) {
                let param_elem = self.parse_element()?;
                params.push(param_elem);
            }
            self.consume(&Token::RightParen, "Expected ')' after parameter list")?;
            params
        } else {
            Vec::new()
        };

        let _arg_names: Vec<String> = args_list
            .iter()
            .filter_map(|e| match e {
                GrammarElement::Atom(s) => Some(s.clone()),
                GrammarElement::Expr(boxed_expr) => {
                    if let crate::grammar::core::Expr::Var(s) = boxed_expr.as_ref() {
                        Some(s.clone())
                    } else {
                        None
                    }
                }
                _ => None,
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
        // (let name value body...) 或 (let name value) - body 是可选的，但至少需要 name 和 value
        let name = self.parse_element()?;
        let value = self.parse_element()?;

        // 检查是否有 body（如果下一个token是右括号，则没有body）
        let body = if self.check(&Token::RightParen) {
            // 没有body，使用 null 作为默认值
            GrammarElement::Expr(Box::new(Expr::Literal(Literal::Null)))
        } else {
            // 有body，解析所有剩余的表达式作为body
            let mut body_exprs = Vec::new();
            while !self.check(&Token::RightParen) {
                body_exprs.push(self.parse_element()?);
            }

            if body_exprs.is_empty() {
                GrammarElement::Expr(Box::new(Expr::Literal(Literal::Null)))
            } else if body_exprs.len() == 1 {
                body_exprs.into_iter().next().unwrap()
            } else {
                // 多个表达式，包装在一个列表中
                GrammarElement::List(body_exprs)
            }
        };

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
        // 解析参数列表 - 使用专门的参数列表解析函数
        // Parse parameter list - use dedicated parameter list parsing function
        let args_list = if self.check(&Token::LeftParen) {
            self.consume(&Token::LeftParen, "Expected '(' for parameter list")?;
            let mut params = Vec::new();
            while !self.check(&Token::RightParen) {
                let param = self.parse_element()?;
                params.push(param);
            }
            self.consume(&Token::RightParen, "Expected ')' after parameter list")?;
            params
        } else {
            Vec::new()
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
                return Err(ParseError::syntax_error(
                    "Expected '(' for match case".to_string(),
                    None,
                ));
            }
            self.consume(&Token::LeftParen, "Expected '(' for match case")?;

            // 解析模式
            let pattern_elem = self.parse_element()?;
            // 特殊处理：如果模式被解析为 Expr::Call("_", ...)，直接转换为通配符模式
            // 这可以避免 _ 被错误地解析为函数调用
            let pattern = if let GrammarElement::Expr(boxed_expr) = &pattern_elem {
                if let Expr::Call(name, _) = boxed_expr.as_ref() {
                    if name == "_" {
                        use crate::grammar::core::Pattern::Wildcard;
                        Wildcard
                    } else {
                        self.element_to_pattern(&pattern_elem)?
                    }
                } else {
                    self.element_to_pattern(&pattern_elem)?
                }
            } else {
                self.element_to_pattern(&pattern_elem)?
            };

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

    fn parse_for(&mut self) -> Result<GrammarElement, ParseError> {
        // (for var iterable body...)
        let var_elem = self.parse_element()?;
        let iterable_elem = self.parse_element()?;

        // 解析body部分，可能包含多个表达式
        let mut body_elements = Vec::new();
        while !self.check(&Token::RightParen) {
            body_elements.push(self.parse_element()?);
        }
        self.consume(&Token::RightParen, "Expected ')' after for expression")?;

        // 提取变量名
        let var = match &var_elem {
            GrammarElement::Atom(s) => s.clone(),
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    s.clone()
                } else {
                    return Err(ParseError::syntax_error(
                        "For loop variable must be an identifier".to_string(),
                        None,
                    ));
                }
            }
            _ => {
                return Err(ParseError::syntax_error(
                    "For loop variable must be an identifier".to_string(),
                    None,
                ));
            }
        };

        let iterable_expr = self.element_to_expr(&iterable_elem)?;

        // 如果body有多个表达式，将它们包装在begin表达式中
        let body_expr = if body_elements.len() == 1 {
            self.element_to_expr(&body_elements[0])?
        } else {
            // 创建begin表达式
            let body_exprs: Vec<Expr> = body_elements
                .iter()
                .map(|elem| self.element_to_expr(elem))
                .collect::<Result<Vec<_>, _>>()?;
            Expr::Begin(body_exprs)
        };

        Ok(GrammarElement::Expr(Box::new(Expr::For {
            var,
            iterable: Box::new(iterable_expr),
            body: Box::new(body_expr),
        })))
    }

    fn parse_while(&mut self) -> Result<GrammarElement, ParseError> {
        // (while condition body...)
        let condition_elem = self.parse_element()?;

        // 解析body部分，可能包含多个表达式
        let mut body_elements = Vec::new();
        while !self.check(&Token::RightParen) {
            body_elements.push(self.parse_element()?);
        }
        self.consume(&Token::RightParen, "Expected ')' after while expression")?;

        let condition_expr = self.element_to_expr(&condition_elem)?;

        // 如果body有多个表达式，将它们包装在begin表达式中
        let body_expr = if body_elements.len() == 1 {
            self.element_to_expr(&body_elements[0])?
        } else {
            // 创建begin表达式
            let body_exprs: Vec<Expr> = body_elements
                .iter()
                .map(|elem| self.element_to_expr(elem))
                .collect::<Result<Vec<_>, _>>()?;
            Expr::Begin(body_exprs)
        };

        Ok(GrammarElement::Expr(Box::new(Expr::While {
            condition: Box::new(condition_expr),
            body: Box::new(body_expr),
        })))
    }

    fn parse_try(&mut self) -> Result<GrammarElement, ParseError> {
        // (try try_body catch [var] catch_body)
        let try_body_elem = self.parse_element()?;

        // 检查是否有 catch 关键字
        let (catch_var, catch_body_elem) = if self.check(&Token::Symbol("catch".to_string())) {
            self.advance_token(); // 消费 catch

            // 检查是否有 catch 变量名（可选）
            // 如果下一个 token 是标识符（Symbol），且后面还有元素，则它是变量名
            let var_name =
                if !self.check(&Token::RightParen) && matches!(self.peek(), Token::Symbol(_)) {
                    // 可能是变量名，先解析看看
                    let next_elem = self.parse_element()?;
                    // 检查是否是简单的标识符（变量名）
                    match &next_elem {
                        GrammarElement::Atom(name) => {
                            // 检查后面是否还有元素（不是右括号）
                            if !self.check(&Token::RightParen) {
                                // 后面还有元素，name 是变量名
                                Some(name.clone())
                            } else {
                                // 后面是右括号，name 是 catch_body
                                return Ok(GrammarElement::Expr(Box::new(Expr::Try {
                                    try_body: Box::new(self.element_to_expr(&try_body_elem)?),
                                    catch_var: None,
                                    catch_body: Box::new(self.element_to_expr(&next_elem)?),
                                })));
                            }
                        }
                        GrammarElement::Expr(boxed_expr) => {
                            if let Expr::Var(name) = boxed_expr.as_ref() {
                                // 检查后面是否还有元素
                                if !self.check(&Token::RightParen) {
                                    Some(name.clone())
                                } else {
                                    // 后面是右括号，这是 catch_body
                                    return Ok(GrammarElement::Expr(Box::new(Expr::Try {
                                        try_body: Box::new(self.element_to_expr(&try_body_elem)?),
                                        catch_var: None,
                                        catch_body: Box::new(*boxed_expr.clone()),
                                    })));
                                }
                            } else {
                                // 不是变量名，这是 catch_body
                                return Ok(GrammarElement::Expr(Box::new(Expr::Try {
                                    try_body: Box::new(self.element_to_expr(&try_body_elem)?),
                                    catch_var: None,
                                    catch_body: Box::new(*boxed_expr.clone()),
                                })));
                            }
                        }
                        _ => {
                            // 不是变量名，这是 catch_body
                            return Ok(GrammarElement::Expr(Box::new(Expr::Try {
                                try_body: Box::new(self.element_to_expr(&try_body_elem)?),
                                catch_var: None,
                                catch_body: Box::new(self.element_to_expr(&next_elem)?),
                            })));
                        }
                    }
                } else {
                    None
                };

            // 解析 catch_body
            let catch_body = self.parse_element()?;
            (var_name, catch_body)
        } else {
            // 没有 catch 关键字，直接解析 catch_body
            let catch_body = self.parse_element()?;
            (None, catch_body)
        };

        self.consume(&Token::RightParen, "Expected ')' after try expression")?;

        let try_body_expr = self.element_to_expr(&try_body_elem)?;
        let catch_body_expr = self.element_to_expr(&catch_body_elem)?;

        Ok(GrammarElement::Expr(Box::new(Expr::Try {
            try_body: Box::new(try_body_expr),
            catch_var,
            catch_body: Box::new(catch_body_expr),
        })))
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
                Expr::Var(name) => {
                    // 如果变量名是 "_"，这是通配符模式
                    if name == "_" {
                        Ok(Wildcard)
                    } else {
                        Ok(Var(name.clone()))
                    }
                }
                // 允许Call表达式，因为在某些情况下列表可能被解析为Call
                // 如果模式是 (list ...) 这样的，需要特殊处理
                // 如果函数名是 "_"，这可能是错误解析导致的，应该被忽略
                Expr::Call(name, _) => {
                    // 如果函数名是 "_"，这可能是错误解析，应该返回通配符模式
                    if name == "_" {
                        Ok(Wildcard)
                    } else if name == "list" || name == "vec" {
                        // 对于 list/vec 模式，需要在 parse_match 中特殊处理
                        // 这里暂时返回错误，因为它应该已经在 parse_list 中处理
                        Err(ParseError::syntax_error(
                            "List pattern should not be parsed as function call".to_string(),
                            None,
                        ))
                    } else {
                        Err(ParseError::syntax_error(
                            "Invalid pattern in match expression".to_string(),
                            None,
                        ))
                    }
                }
                _ => Err(ParseError::syntax_error(
                    "Invalid pattern in match expression".to_string(),
                    None,
                )),
            },
            GrammarElement::List(list) => {
                let mut patterns = Vec::new();
                for item in list {
                    patterns.push(self.element_to_pattern(item)?);
                }
                Ok(List(patterns))
            }
            _ => Err(ParseError::syntax_error(
                "Invalid pattern in match expression".to_string(),
                None,
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
                return Err(ParseError::syntax_error(
                    "Dictionary requires key-value pairs".to_string(),
                    None,
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
                        return Err(ParseError::syntax_error(
                            "Dictionary key must be a string or identifier".to_string(),
                            None,
                        ));
                    }
                }
                _ => {
                    return Err(ParseError::syntax_error(
                        "Dictionary key must be a string or identifier".to_string(),
                        None,
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
            _ => Err(ParseError::syntax_error(
                "Expected string".to_string(),
                None,
            )),
        }
    }

    fn parse_number(&mut self) -> Result<GrammarElement, ParseError> {
        match self.advance_token() {
            Token::Number(n) => {
                // 尝试解析为整数或浮点数
                if n.contains('.') {
                    n.parse::<f64>()
                        .map(|f| GrammarElement::Expr(Box::new(Expr::Literal(Literal::Float(f)))))
                        .map_err(|_| {
                            ParseError::syntax_error(format!("Invalid float: {}", n), None)
                        })
                } else {
                    n.parse::<i64>()
                        .map(|i| GrammarElement::Expr(Box::new(Expr::Literal(Literal::Int(i)))))
                        .map_err(|_| {
                            ParseError::syntax_error(format!("Invalid integer: {}", n), None)
                        })
                }
            }
            _ => Err(ParseError::syntax_error(
                "Expected number".to_string(),
                None,
            )),
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
            _ => Err(ParseError::syntax_error(
                "Expected symbol".to_string(),
                None,
            )),
        }
    }

    fn parse_binop(&self, op: &str) -> Option<BinOp> {
        match op {
            "+" => Some(BinOp::Add),
            "-" => Some(BinOp::Sub),
            "*" => Some(BinOp::Mul),
            "/" => Some(BinOp::Div),
            "%" => Some(BinOp::Mod),
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
                    // 操作符可以作为函数参数使用，将其转换为变量
                    let op_name = s.strip_prefix("op:").unwrap_or(s);
                    Ok(Expr::Var(op_name.to_string()))
                } else {
                    Ok(Expr::Var(s.clone()))
                }
            }
            GrammarElement::List(l) => {
                if l.is_empty() {
                    Ok(Expr::Literal(Literal::Null))
                } else {
                    // 检查是否是 list 或 vec 字面量
                    let is_list_literal = match &l[0] {
                        GrammarElement::Atom(s) => s == "list" || s == "vec",
                        GrammarElement::Expr(boxed_expr) => {
                            if let Expr::Var(s) = boxed_expr.as_ref() {
                                s == "list" || s == "vec"
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    if is_list_literal {
                        // 列表字面量：转换为 Literal::List
                        let expr_items: Vec<Expr> = l[1..]
                            .iter()
                            .map(|e| self.element_to_expr(e))
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(Expr::Literal(Literal::List(expr_items)))
                    } else {
                        // 检查是否是 dict 或 map 字面量
                        let is_dict_literal = match &l[0] {
                            GrammarElement::Atom(s) => s == "dict" || s == "map",
                            GrammarElement::Expr(boxed_expr) => {
                                if let Expr::Var(s) = boxed_expr.as_ref() {
                                    s == "dict" || s == "map"
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        };

                        if is_dict_literal {
                            // 字典字面量：转换为 Literal::Dict
                            if l.len() % 2 != 1 {
                                return Err(ParseError::syntax_error(
                                    "Dictionary literal requires even number of key-value pairs"
                                        .to_string(),
                                    None,
                                ));
                            }
                            let mut pairs = Vec::new();
                            for i in (1..l.len()).step_by(2) {
                                let key_elem = &l[i];
                                let value_elem = &l[i + 1];
                                let key = match key_elem {
                                    GrammarElement::Atom(s) => s.clone(),
                                    GrammarElement::Expr(boxed_expr) => {
                                        if let Expr::Var(s) = boxed_expr.as_ref() {
                                            s.clone()
                                        } else if let Expr::Literal(Literal::String(s)) =
                                            boxed_expr.as_ref()
                                        {
                                            s.clone()
                                        } else {
                                            return Err(ParseError::syntax_error(
                                                "Dictionary key must be a string or atom"
                                                    .to_string(),
                                                None,
                                            ));
                                        }
                                    }
                                    _ => {
                                        return Err(ParseError::syntax_error(
                                            "Dictionary key must be a string or atom".to_string(),
                                            None,
                                        ));
                                    }
                                };
                                let value_expr = self.element_to_expr(value_elem)?;
                                pairs.push((key, value_expr));
                            }
                            Ok(Expr::Literal(Literal::Dict(pairs)))
                        } else {
                            // 检查是否是 lambda 表达式（特殊形式，不能转换为 Expr::Call）
                            // Check if this is a lambda expression (special form, cannot convert to Expr::Call)
                            let is_lambda = match &l[0] {
                                GrammarElement::Atom(s) => s == "lambda",
                                GrammarElement::Expr(boxed_expr) => {
                                    if let Expr::Var(s) = boxed_expr.as_ref() {
                                        s == "lambda"
                                    } else {
                                        false
                                    }
                                }
                                _ => false,
                            };

                            if is_lambda {
                                // lambda 表达式不能转换为 Expr，返回错误让调用者直接评估 GrammarElement
                                // Lambda expressions cannot be converted to Expr, return error to let caller evaluate GrammarElement directly
                                // 注意：这个错误会被调用者捕获，然后直接评估 GrammarElement
                                // Note: This error will be caught by caller, which will then evaluate GrammarElement directly
                                return Err(ParseError::syntax_error(
                                    "Lambda expressions must be evaluated as GrammarElement, not converted to Expr".to_string(),
                                    None,
                                ));
                            }

                            // 函数调用
                            let func_name = match &l[0] {
                                GrammarElement::Atom(s) => s.clone(),
                                GrammarElement::Expr(boxed_expr) => {
                                    if let Expr::Var(s) = boxed_expr.as_ref() {
                                        s.clone()
                                    } else {
                                        return Err(ParseError::syntax_error(
                                            "Function name must be an atom or variable".to_string(),
                                            None,
                                        ));
                                    }
                                }
                                _ => {
                                    return Err(ParseError::syntax_error(
                                        "Function name must be an atom or variable".to_string(),
                                        None,
                                    ));
                                }
                            };
                            let args: Vec<Expr> = l[1..]
                                .iter()
                                .map(|e| self.element_to_expr(e))
                                .collect::<Result<Vec<_>, _>>()?;
                            Ok(Expr::Call(func_name, args))
                        }
                    }
                }
            }
            GrammarElement::NaturalLang(_) => Err(ParseError::syntax_error(
                "Natural language not supported in expressions".to_string(),
                None,
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
