#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    LiteralInt = 1,
    LiteralFloat = 2,
    BinaryAdd = 10,
    BinarySub = 11,
    Conditional = 20,
    BinaryEq = 30,
    BinaryGt = 31,
    PerformIOWriteLine = 100,
    // Добавляй новые типы сюда по мере необходимости
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    DataInput = 1,
    Condition = 2,
    ThenBranch = 3,
    ElseBranch = 4,
    // Новые типы рёбер можно добавлять здесь
}

impl NodeType {
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            1 => Some(NodeType::LiteralInt),
            2 => Some(NodeType::LiteralFloat),
            10 => Some(NodeType::BinaryAdd),
            11 => Some(NodeType::BinarySub),
            20 => Some(NodeType::Conditional),
            30 => Some(NodeType::BinaryEq),
            31 => Some(NodeType::BinaryGt),
            100 => Some(NodeType::PerformIOWriteLine),
            _ => None,
        }
    }
    pub fn code(self) -> u64 {
        self as u64
    }
}

impl EdgeType {
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            1 => Some(EdgeType::DataInput),
            2 => Some(EdgeType::Condition),
            3 => Some(EdgeType::ThenBranch),
            4 => Some(EdgeType::ElseBranch),
            _ => None,
        }
    }
    pub fn code(self) -> u64 {
        self as u64
    }
}
