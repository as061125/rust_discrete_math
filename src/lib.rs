//! # `discrete_math` — Rust 离散数学核心库（从集合到关系到函数）
//!
//! 本 Crate 以严格的公理化构造方式，循序渐进地实现了离散数学的三大核心支柱：
//!
//! 1. **集合论（[`set`]）**：泛型有限集合 [`MathSet<T>`]、子集/真子集判定、并/交/差/对称差/补运算、
//!    幂集 $\mathcal{P}(A)$、笛卡尔积 $A \times B$ 以及集合划分（Partition）验证。
//! 2. **二元关系（[`relation`]）**：建立在笛卡尔积子集 $R \subseteq A \times B$ 上的二元关系
//!    [`BinaryRelation<A, B>`]，支持定义域/值域/像集、逆关系 $R^{-1}$、关系复合 $S \circ R$；
//!    在同质关系上支持五大性质判定（自反、反自反、对称、反对称、传递）、三大闭包
//!    （自反闭包、对称闭包、传递闭包）、等价关系与商集 $A/R$、偏序关系与全序关系
//!    （极小/极大元、最小/最大元）。
//! 3. **函数映射（[`function`]）**：将函数 $f: A \to B$ 严格定义为满足**左全性（全域性）**与
//!    **右唯一性（单值性）**的特殊二元关系 [`DiscreteFunction<A, B>`]，支持函数求值、像与原像、
//!    单射/满射/双射判定、函数复合 $g \circ f$ 以及双射函数的逆函数 $f^{-1}$。
//!
//! ## 模块结构
//!
//! | 模块 | 文件 | 主要内容 |
//! |------|------|----------|
//! | [`set`] | `src/set.rs` | 有限集合 `MathSet<T>` 与集合代数 |
//! | [`relation`] | `src/relation.rs` | 二元关系 `BinaryRelation<A, B>` |
//! | [`function`] | `src/function.rs` | 函数 `DiscreteFunction<A, B>` |
//! | [`error`] | `src/error.rs` | 统一错误类型 [`DiscreteError`] |
//!
//! ## 快速示例
//!
//! ```
//! use discrete_math::{BinaryRelation, DiscreteFunction, MathSet};
//!
//! // 1. 集合：幂集大小满足 |P(A)| = 2^|A|
//! let a = MathSet::from_slice(&[1, 2, 3]);
//! assert_eq!(a.power_set().cardinality(), 8);
//!
//! // 2. 关系：模 2 同余是等价关系，其商集构成原集合的划分
//! let r = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| x % 2 == y % 2);
//! assert!(r.is_equivalence());
//! let quotient = r.quotient_set().unwrap();
//! assert_eq!(quotient.cardinality(), 2);
//! assert!(a.is_valid_partition(&quotient));
//!
//! // 3. 函数：双射可求逆函数
//! let b = MathSet::from_slice(&[10, 20, 30]);
//! let f = DiscreteFunction::from_fn(a, b, |x| x * 10).unwrap();
//! assert!(f.is_bijective());
//! assert_eq!(f.inverse().unwrap().eval(&20), Some(&2));
//! ```
//!
//! ## 设计约定
//!
//! - 所有集合均为**有限集**，底层使用 `BTreeSet<T>`，因此元素类型必须实现 `Ord + Clone`。
//! - 集合、关系、函数均实现 `Clone`、`PartialEq`、`Eq`，可直接比较，也可作为哈希键或集合元素。
//! - 可能失败的运算返回 `Result<_, DiscreteError>`，错误语义见 [`DiscreteError`]。
//!
//! 更完整的使用说明、API 速查表与设计取舍见项目根目录的 `README.md`。

pub mod error;
pub mod function;
pub mod relation;
pub mod set;

pub use error::DiscreteError;
pub use function::DiscreteFunction;
pub use relation::{BinaryRelation, HomogeneousRelation};
pub use set::MathSet;
