//! 错误类型定义。
//!
//! 本模块集中定义库中所有可能失败的离散数学操作所返回的错误类型 [`DiscreteError`]，
//! 使 `set`、`relation`、`function` 三个模块共享同一套错误语义。

use std::fmt;

/// 离散数学运算与构造过程中的错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscreteError {
    /// 关系中包含不属于前域（Domain Set）或陪域（Codomain Set）的元素
    ElementOutOfBounds(String),
    /// 两个关系或函数的陪域与前域不匹配，无法进行复合运算
    IncompatibleDomains,
    /// 该操作要求同质关系（前域与陪域必须相等，即 $A = B$）
    NotHomogeneousRelation,
    /// 该关系不是等价关系（不满足自反、对称或传递性）
    NotEquivalenceRelation,
    /// 该关系不是偏序关系（不满足自反、反对称或传递性）
    NotPartialOrder,
    /// 二元关系不满足左全性（存在前域元素没有对应的像），无法构成函数
    NotLeftTotal,
    /// 二元关系不满足单值性（存在前域元素对应多个像），无法构成函数
    NotRightUnique,
    /// 函数不是双射（既非单射或非满射），不存在逆函数
    NotBijective,
}

impl fmt::Display for DiscreteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscreteError::ElementOutOfBounds(msg) => write!(f, "元素超出集合范围: {}", msg),
            DiscreteError::IncompatibleDomains => {
                write!(f, "前域与陪域不匹配，无法执行复合运算")
            }
            DiscreteError::NotHomogeneousRelation => {
                write!(f, "该性质或运算仅适用于集合 A 上的同质二元关系 (A = B)")
            }
            DiscreteError::NotEquivalenceRelation => write!(f, "当前关系不是等价关系"),
            DiscreteError::NotPartialOrder => write!(f, "当前关系不是偏序关系"),
            DiscreteError::NotLeftTotal => {
                write!(f, "关系不满足左全性（定义域不等于前域），不能构成函数")
            }
            DiscreteError::NotRightUnique => {
                write!(f, "关系不满足单值性（存在一对多映射），不能构成函数")
            }
            DiscreteError::NotBijective => write!(f, "函数不是双射，无法求逆函数"),
        }
    }
}

impl std::error::Error for DiscreteError {}
