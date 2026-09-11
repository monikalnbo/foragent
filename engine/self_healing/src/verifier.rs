use tree_sitter::Parser;

pub struct AstVerifier {
    parser: Parser,
}

impl AstVerifier {
    pub fn new_rust() -> Result<Self, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .map_err(|e| e.to_string())?;
        Ok(Self { parser })
    }

    /// 执行毫秒级 AST 语法干跑自检
    pub fn check_syntax(&mut self, code: &str) -> Result<(), String> {
        let tree = self.parser.parse(code, None).ok_or("AST 解析失败")?;
        let root = tree.root_node();

        if root.has_error() {
            let mut cursor = root.walk();
            for child in root.children(&mut cursor) {
                if child.is_error() || child.is_missing() {
                    let pos = child.start_position();
                    return Err(format!(
                        "Rust 语法错误: 第 {} 行第 {} 列附近存在不合法代码",
                        pos.row + 1,
                        pos.column
                    ));
                }
            }
            return Err("发现未闭合的代码块或语法错误".to_string());
        }
        Ok(())
    }
}
