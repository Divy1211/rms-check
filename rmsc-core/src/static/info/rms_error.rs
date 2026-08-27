use ariadne::ReportKind;

use crate::parsing::Identifier;
use crate::parsing::Span;

#[derive(Debug, Clone)]
pub enum RmsError {
    // type errors
    TypeMismatch { actual: String, expected: String, span: Span, note: Option<String> },
    OpMismatch { op: String, type1: String, type2: String, span: Span, note: Option<String> },

    // name errors
    UndefinedName { name: String, span: Span },
    DeadName { name: String, span: Span },
    MaybeDeadName { name: String, span: Span },


    UnresolvedInclude { inc_filename: String, span: Span },
    
    Syntax { span: Span, msg: String, keywords: Vec<String> },

    Warning { span: Span, msg: String, keywords: Vec<String>, kind: WarningKind, ignored: bool },
}

#[derive(Debug, Clone)]
pub enum WarningKind {
    ShadowedVarName = 100,

    UnknownWarningName = 1000,
}

impl RmsError {
    pub fn type_mismatch(actual: &str, expected: &str, span: &Span, note: Option<&str>) -> RmsError {
        RmsError::TypeMismatch {
            actual: String::from(actual),
            expected: String::from(expected),
            span: *span,
            note: note.map(String::from),
        }
    }

    pub fn op_mismatch(op: &str, type1: &str, type2: &str, span: &Span, note: Option<&str>) -> RmsError {
        RmsError::OpMismatch {
            op: String::from(op),
            type1: String::from(type1),
            type2: String::from(type2),
            span: *span,
            note: note.map(String::from),
        }
    }

    pub fn undefined_name(name: &Identifier, span: &Span) -> RmsError {
        RmsError::UndefinedName {
            name: String::from(&name.0),
            span: *span,
        }
    }

    pub fn dead_name(name: &Identifier, span: &Span) -> RmsError {
        RmsError::DeadName {
            name: String::from(&name.0),
            span: *span,
        }
    }

    pub fn maybe_dead_name(name: &Identifier, span: &Span) -> RmsError {
        RmsError::MaybeDeadName {
            name: String::from(&name.0),
            span: *span,
        }
    }

    pub fn unresolved_include(inc_filename: &str, span: &Span) -> RmsError {
        RmsError::UnresolvedInclude {
            inc_filename: inc_filename.into(),
            span: *span,
        }
    }
    
    pub fn syntax(span: &Span, msg: &str, keywords: Vec<&str>) -> RmsError {
        RmsError::Syntax {
            span: *span,
            msg: String::from(msg),
            keywords: keywords.into_iter().map(String::from).collect(),
        }
    }

    pub fn warning(span: &Span, msg: &str, keywords: Vec<&str>, kind: WarningKind) -> RmsError {
        RmsError::Warning {
            span: *span,
            msg: String::from(msg),
            keywords: keywords.into_iter().map(String::from).collect(),
            kind,
            ignored: false,
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            RmsError::TypeMismatch { span, .. } => { span }
            RmsError::OpMismatch { span, .. } => { span }
            RmsError::UndefinedName { span, .. } => { span }
            RmsError::DeadName { span, .. } => { span }
            RmsError::MaybeDeadName { span, .. } => { span }
            RmsError::UnresolvedInclude { span, .. } => { span }
            RmsError::Syntax { span, .. } => { span }
            RmsError::Warning { span, .. } => { span }
        }
    }

    pub fn report_kind(&self) -> ReportKind<'_> {
        match self {
            RmsError::Warning { .. } => { ReportKind::Warning }
            _ => { ReportKind::Error }
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            RmsError::TypeMismatch { .. } => { "TypeError" }
            RmsError::OpMismatch { .. } => { "TypeError" }

            RmsError::UndefinedName { .. } => { "NameError" }
            RmsError::DeadName { .. } => { "NameError" }
            RmsError::MaybeDeadName { .. } => { "NameError" }

            RmsError::UnresolvedInclude { .. } => { "UnresolvedInclude" }
            
            RmsError::Syntax { .. } => { "SyntaxError" }

            RmsError::Warning { kind: type_, .. } => { type_.as_str() }
        }
    }

    pub fn is_warning(&self) -> bool {
        matches!(self, RmsError::Warning { .. })
    }
    
    pub fn is_ignored(&self) -> bool {
        match self {
            RmsError::Warning { ignored, .. } => *ignored,
            _ => false,
        }
    }
    
    pub fn code(&self) -> u32 {
        match self {
            RmsError::TypeMismatch { .. } => { 0 }
            RmsError::OpMismatch { .. } => { 1 }
            RmsError::UndefinedName { .. } => { 2 }
            RmsError::DeadName { .. } => { 3 }
            RmsError::MaybeDeadName { .. } => { 4 }
            RmsError::UnresolvedInclude { .. } => { 5 }
            RmsError::Syntax { .. } => { 6 }
            RmsError::Warning { kind, .. } => { kind.as_u32() }
        }
    }
}

impl WarningKind {
    pub fn as_u32(&self) -> u32 {
        self.clone() as u32
    }

    pub fn as_str(&self) -> &str {
        match self {
            WarningKind::ShadowedVarName => { "TopStrInit" }
            WarningKind::UnknownWarningName => { "UnknownWarningName" }
        }
    }

    pub fn from_name(name: &str) -> Option<WarningKind> {
        match name {
            "ShadowedVarName"          => { Some(WarningKind::ShadowedVarName) }
            
            // UnknownWarningName cannot be ignored, so it is excluded here
            _                     => None
        }
    }
}