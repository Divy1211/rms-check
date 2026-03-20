use std::collections::{HashSet};
use crate::parsing::{Identifier, Type};
use crate::r#static::info::{IdInfo};
use crate::utils::warnings_from_str;

#[derive(Debug, Clone)]
pub enum Doc {
    None,
    Ignore(HashSet<u32>),

    #[allow(unused)]
    Desc(String),
}

impl Doc {
    #[allow(unused)]
    pub fn is_none(&self) -> bool {
        matches!(self, Doc::None)
    }

    pub fn  parse(comment: &str) -> Result<Doc, &str> {
        let comment = comment.trim_start();
        if comment.starts_with("/* rms-ignore: ") {
            let comment = comment
                .trim_start_matches("/* rms-ignore: ")
                .trim_end_matches(" */");
            return Ok(Doc::Ignore(warnings_from_str(comment)?));
        }
        if !comment.starts_with("/**") {
            return Ok(Doc::None);
        }
        
        let content = comment
            .trim_start_matches("/**")
            .trim_end_matches("*/")
            .lines()
            .map(|line| {
                line.trim_start()
                    .trim_start_matches('*')
                    .trim_start()
                    .to_string()
            })
            .collect::<Vec<_>>();

        Ok(Doc::Desc(content.join("\n").trim().to_string()))
    }

    #[allow(unused)]
    pub fn render(&self, id: &Identifier, info: &IdInfo) -> String {
        let sign = match &info.type_ {
            Type::Int | Type::Float => {
                format!("```rms\n#const {}\n```", id.0)
            }
            Type::Label => {
                format!("```rms\n#define {}\n```", id.0)
            }
            Type::Str => {
                unreachable!("Internal Error: Type::Str")
            }
        };
        
        match self {
            Doc::None | Doc::Ignore(_) => sign,
            Doc::Desc(desc) => {
                format!("{}\n\n{}", sign, desc.clone())
            },
        }
    }
}