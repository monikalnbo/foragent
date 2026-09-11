#[derive(Debug, PartialEq, Clone)]
pub enum HealingAction {
    RetryWithFuzzy,
    InjectSyntaxFeedback(String),
    TriggerDynamicReplan(String),
    RollbackToLKGS,
}

#[derive(Debug, Clone)]
pub struct DiagnosticReport {
    pub error_msg: String,
    pub recommended_action: HealingAction,
}

pub struct HealingDiagnoser;

impl HealingDiagnoser {
    pub fn diagnose(error_str: &str, retry_count: usize) -> DiagnosticReport {
        if retry_count >= 3 {
            return DiagnosticReport {
                error_msg: "已达最大重试上限，自动熔断回滚".to_string(),
                recommended_action: HealingAction::RollbackToLKGS,
            };
        }

        if error_str.contains("未定位到待替换") || error_str.contains("PatchMismatch") {
            DiagnosticReport {
                error_msg: error_str.to_string(),
                recommended_action: HealingAction::RetryWithFuzzy,
            }
        } else if error_str.contains("语法错误") || error_str.contains("Syntax") {
            DiagnosticReport {
                error_msg: error_str.to_string(),
                recommended_action: HealingAction::InjectSyntaxFeedback(error_str.to_string()),
            }
        } else {
            DiagnosticReport {
                error_msg: error_str.to_string(),
                recommended_action: HealingAction::TriggerDynamicReplan(error_str.to_string()),
            }
        }
    }
}
