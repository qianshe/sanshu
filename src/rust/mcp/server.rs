use super::tools::{InteractionTool, MemoryTool, AcemcpTool, Context7Tool, IconTool, UiuxTool, EnhanceTool};

// 技能运行时工具 - 动态发现 skills 并追加工具
        let project_root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        // tools.extend(SkillsTool::list_dynamic_tools(&project_root)); // 已禁用
            "skill_run" | _ if name.starts_with("skill_") => {
                // SkillsTool 动态注册已禁用，skill_* 工具不再公开暴露
                return Err(ErrorData::method_not_found(format!(
                    "{} 未启用。SkillsTool 动态注册已禁用。",
                    name
                ), None));
            }