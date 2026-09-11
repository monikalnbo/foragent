pub struct SemanticGuard;

impl SemanticGuard {
    /// 语义防偷删看门狗：防止模型为了自愈语法通过而把原有函数或业务逻辑大面积清空
    pub fn is_safe_patch(original: &str, patched: &str) -> Result<(), String> {
        let orig_lines = original.lines().count();
        let patch_lines = patched.lines().count();

        // 原代码超过 15 行，但修改后断崖式缩减超过 50% 行数，触发看门狗拦截
        if orig_lines > 15 && patch_lines < orig_lines / 2 {
            return Err(format!(
                "语义看门狗拦截: 修改后代码行数由 {} 行骤降至 {} 行，判定为破坏性伪自愈",
                orig_lines, patch_lines
            ));
        }

        Ok(())
    }
}
