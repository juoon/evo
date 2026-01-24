// 代码解释器 / Code explainer
// 将代码结构转换为自然语言描述
// Converts code structures to natural language descriptions

use crate::grammar::core::{BinOp, Expr, GrammarElement, Literal};

/// 代码解释器 / Code explainer
pub struct CodeExplainer {
    /// 语言偏好 / Language preference
    language: Language,
}

/// 语言偏好 / Language preference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// 中文 / Chinese
    Chinese,
    /// 英文 / English
    English,
}

impl CodeExplainer {
    /// 创建新代码解释器 / Create new code explainer
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    /// 解释代码结构 / Explain code structure
    pub fn explain(&self, element: &GrammarElement) -> String {
        match element {
            GrammarElement::Expr(expr) => self.explain_expr(expr),
            GrammarElement::List(list) => self.explain_list(list),
            GrammarElement::Atom(atom) => self.explain_atom(atom),
            GrammarElement::NaturalLang(nl) => nl.clone(),
        }
    }

    /// 解释表达式 / Explain expression
    fn explain_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(lit) => self.explain_literal(lit),
            Expr::Var(name) => self.explain_variable(name),
            Expr::Call(name, args) => self.explain_call(name, args),
            Expr::Binary(op, left, right) => self.explain_binary(op, left, right),
            Expr::If(cond, then_expr, else_expr) => {
                self.explain_conditional(cond, then_expr, else_expr)
            }
            Expr::Match(_, _) => "Match expression".to_string(),
            Expr::For { .. } => "For loop".to_string(),
            Expr::While { .. } => "While loop".to_string(),
            Expr::Try { .. } => "Try-catch expression".to_string(),
            Expr::Lambda { .. } => "Lambda expression".to_string(),
            Expr::Begin(_) => "Begin expression".to_string(),
            Expr::Assign(_, _) => "Assignment expression".to_string(),
        }
    }

    /// 解释列表 / Explain list
    fn explain_list(&self, list: &[GrammarElement]) -> String {
        if list.is_empty() {
            return match self.language {
                Language::Chinese => "空列表".to_string(),
                Language::English => "empty list".to_string(),
            };
        }

        // 检查是否是特殊形式 / Check if special form
        if let Some(GrammarElement::Atom(first)) = list.first() {
            match first.as_str() {
                "def" | "function" => {
                    return self.explain_function_definition(list);
                }
                "let" => {
                    return self.explain_variable_definition(list);
                }
                "if" => {
                    return self.explain_if_statement(list);
                }
                _ => {}
            }
        }

        // 解释为函数调用 / Explain as function call
        let mut parts = Vec::new();
        for (i, elem) in list.iter().enumerate() {
            if i == 0 {
                parts.push(self.explain(elem));
            } else {
                parts.push(self.explain(elem));
            }
        }

        match self.language {
            Language::Chinese => format!("调用 {}，参数：{}", parts[0], parts[1..].join("、")),
            Language::English => format!(
                "call {} with arguments: {}",
                parts[0],
                parts[1..].join(", ")
            ),
        }
    }

    /// 解释函数定义 / Explain function definition
    fn explain_function_definition(&self, list: &[GrammarElement]) -> String {
        if list.len() < 3 {
            return match self.language {
                Language::Chinese => "不完整的函数定义".to_string(),
                Language::English => "incomplete function definition".to_string(),
            };
        }

        let name = match &list[1] {
            GrammarElement::Atom(name) => name.clone(),
            _ => "unknown".to_string(),
        };

        let params = match &list[2] {
            GrammarElement::List(params) => params
                .iter()
                .filter_map(|p| match p {
                    GrammarElement::Atom(name) => Some(name.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            _ => vec![],
        };

        let body = if list.len() > 3 {
            self.explain(&list[3])
        } else {
            match self.language {
                Language::Chinese => "空函数体".to_string(),
                Language::English => "empty function body".to_string(),
            }
        };

        match self.language {
            Language::Chinese => {
                if params.is_empty() {
                    format!("定义函数 {}，函数体：{}", name, body)
                } else {
                    format!(
                        "定义函数 {}，参数：{}，函数体：{}",
                        name,
                        params.join("、"),
                        body
                    )
                }
            }
            Language::English => {
                if params.is_empty() {
                    format!("define function {} with body: {}", name, body)
                } else {
                    format!(
                        "define function {} with parameters {} and body: {}",
                        name,
                        params.join(", "),
                        body
                    )
                }
            }
        }
    }

    /// 解释变量定义 / Explain variable definition
    fn explain_variable_definition(&self, list: &[GrammarElement]) -> String {
        if list.len() < 3 {
            return match self.language {
                Language::Chinese => "不完整的变量定义".to_string(),
                Language::English => "incomplete variable definition".to_string(),
            };
        }

        let name = match &list[1] {
            GrammarElement::Atom(name) => name.clone(),
            _ => "unknown".to_string(),
        };

        let value = self.explain(&list[2]);

        match self.language {
            Language::Chinese => format!("定义变量 {}，值为 {}", name, value),
            Language::English => format!("define variable {} with value {}", name, value),
        }
    }

    /// 解释if语句 / Explain if statement
    fn explain_if_statement(&self, list: &[GrammarElement]) -> String {
        if list.len() < 4 {
            return match self.language {
                Language::Chinese => "不完整的条件语句".to_string(),
                Language::English => "incomplete conditional statement".to_string(),
            };
        }

        let condition = self.explain(&list[1]);
        let then_part = self.explain(&list[2]);
        let else_part = if list.len() > 3 {
            self.explain(&list[3])
        } else {
            match self.language {
                Language::Chinese => "无".to_string(),
                Language::English => "none".to_string(),
            }
        };

        match self.language {
            Language::Chinese => {
                format!("如果 {}，则 {}，否则 {}", condition, then_part, else_part)
            }
            Language::English => format!("if {} then {} else {}", condition, then_part, else_part),
        }
    }

    /// 解释原子 / Explain atom
    fn explain_atom(&self, atom: &str) -> String {
        // 检查是否是关键字 / Check if keyword
        match atom {
            "def" | "function" => match self.language {
                Language::Chinese => "定义函数".to_string(),
                Language::English => "define function".to_string(),
            },
            "let" => match self.language {
                Language::Chinese => "定义变量".to_string(),
                Language::English => "define variable".to_string(),
            },
            "if" => match self.language {
                Language::Chinese => "如果".to_string(),
                Language::English => "if".to_string(),
            },
            _ => atom.to_string(),
        }
    }

    /// 解释字面量 / Explain literal
    fn explain_literal(&self, lit: &Literal) -> String {
        match lit {
            Literal::Int(n) => n.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Bool(b) => match self.language {
                Language::Chinese => {
                    if *b {
                        "真".to_string()
                    } else {
                        "假".to_string()
                    }
                }
                Language::English => {
                    if *b {
                        "true".to_string()
                    } else {
                        "false".to_string()
                    }
                }
            },
            Literal::Null => match self.language {
                Language::Chinese => "空值".to_string(),
                Language::English => "null".to_string(),
            },
            Literal::List(items) => {
                let items_str: Vec<String> = items.iter().map(|e| self.explain_expr(e)).collect();
                match self.language {
                    Language::Chinese => format!("列表[{}]", items_str.join("、")),
                    Language::English => format!("list[{}]", items_str.join(", ")),
                }
            }
            Literal::Dict(pairs) => {
                let pairs_str: Vec<String> = pairs
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.explain_expr(v)))
                    .collect();
                match self.language {
                    Language::Chinese => format!("字典{{{}}}", pairs_str.join("、")),
                    Language::English => format!("dict{{{}}}", pairs_str.join(", ")),
                }
            }
        }
    }

    /// 解释变量 / Explain variable
    fn explain_variable(&self, name: &str) -> String {
        match self.language {
            Language::Chinese => format!("变量 {}", name),
            Language::English => format!("variable {}", name),
        }
    }

    /// 解释函数调用 / Explain function call
    fn explain_call(&self, name: &str, args: &[Expr]) -> String {
        let args_str: Vec<String> = args.iter().map(|a| self.explain_expr(a)).collect();
        match self.language {
            Language::Chinese => {
                if args.is_empty() {
                    format!("调用函数 {}", name)
                } else {
                    format!("调用函数 {}，参数：{}", name, args_str.join("、"))
                }
            }
            Language::English => {
                if args.is_empty() {
                    format!("call function {}", name)
                } else {
                    format!(
                        "call function {} with arguments: {}",
                        name,
                        args_str.join(", ")
                    )
                }
            }
        }
    }

    /// 解释二元运算 / Explain binary operation
    fn explain_binary(&self, op: &BinOp, left: &Expr, right: &Expr) -> String {
        let op_str = match (op, self.language) {
            (BinOp::Add, Language::Chinese) => "加",
            (BinOp::Add, Language::English) => "plus",
            (BinOp::Sub, Language::Chinese) => "减",
            (BinOp::Sub, Language::English) => "minus",
            (BinOp::Mul, Language::Chinese) => "乘",
            (BinOp::Mul, Language::English) => "times",
            (BinOp::Div, Language::Chinese) => "除",
            (BinOp::Div, Language::English) => "divided by",
            (BinOp::Mod, Language::Chinese) => "取模",
            (BinOp::Mod, Language::English) => "modulo",
            (BinOp::Eq, Language::Chinese) => "等于",
            (BinOp::Eq, Language::English) => "equals",
            (BinOp::Ne, Language::Chinese) => "不等于",
            (BinOp::Ne, Language::English) => "not equals",
            (BinOp::Lt, Language::Chinese) => "小于",
            (BinOp::Lt, Language::English) => "less than",
            (BinOp::Gt, Language::Chinese) => "大于",
            (BinOp::Gt, Language::English) => "greater than",
            (BinOp::Le, Language::Chinese) => "小于等于",
            (BinOp::Le, Language::English) => "less than or equal",
            (BinOp::Ge, Language::Chinese) => "大于等于",
            (BinOp::Ge, Language::English) => "greater than or equal",
        };

        let left_str = self.explain_expr(left);
        let right_str = self.explain_expr(right);

        match self.language {
            Language::Chinese => format!("{} {} {}", left_str, op_str, right_str),
            Language::English => format!("{} {} {}", left_str, op_str, right_str),
        }
    }

    /// 解释条件表达式 / Explain conditional expression
    fn explain_conditional(&self, cond: &Expr, then_expr: &Expr, else_expr: &Expr) -> String {
        let cond_str = self.explain_expr(cond);
        let then_str = self.explain_expr(then_expr);
        let else_str = self.explain_expr(else_expr);

        match self.language {
            Language::Chinese => format!("如果 {}，则 {}，否则 {}", cond_str, then_str, else_str),
            Language::English => format!("if {} then {} else {}", cond_str, then_str, else_str),
        }
    }

    /// 解释完整的AST / Explain complete AST
    pub fn explain_ast(&self, ast: &[GrammarElement]) -> String {
        let explanations: Vec<String> = ast.iter().map(|e| self.explain(e)).collect();
        explanations.join("\n")
    }
}
