/// 繁體中文(Traditional Chinese)或簡體中文(Simple Chinese)。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChineseVariant {
    /// 繁體中文(Traditional Chinese)。
    Traditional,
    /// 簡體中文(Simple Chinese)。
    Simple,
}

impl ChineseVariant {
    /// 是否為簡體中文(Is this simple?)
    pub fn is_simple(&self) -> bool {
        *self == ChineseVariant::Simple
    }

    /// 是否為繁體中文(Is this traditional?)
    pub fn is_traditional(&self) -> bool {
        *self == ChineseVariant::Traditional
    }
}