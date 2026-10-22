# discrete_math

[![Crates.io](https://img.shields.io/crates/v/discrete_math.svg)](https://crates.io/crates/discrete_math)
[![docs.rs](https://docs.rs/discrete_math/badge.svg)](https://docs.rs/discrete_math)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#许可证)

用 Rust 从公理出发实现离散数学的三大核心结构：**集合（Set）→ 二元关系（Relation）→ 函数（Function）**。

本库把「函数」严格定义为满足**左全性**与**单值性**的特殊二元关系，把「关系」定义为笛卡尔积的子集，
把「集合」实现为泛型有限集，从而让三条主线在类型系统层面彼此衔接、层层递进。

- 不依赖任何第三方 crate（`[dependencies]` 为空）。
- 全程 `Result` 返回，非法构造（越界元素、非等价关系求商集、非双射求逆等）都会被显式拒绝。
- 12 个单元测试 + 7 个文档测试，覆盖德摩根律、闭包、商集、整除偏序、双射求逆等经典结论。

---

## 目录

- [项目结构](#项目结构)
- [安装](#安装)
- [快速开始](#快速开始)
- [核心概念与数学定义](#核心概念与数学定义)
- [API 速查表](#api-速查表)
- [使用示例](#使用示例)
- [错误处理](#错误处理)
- [设计说明与复杂度](#设计说明与复杂度)
- [测试与文档](#测试与文档)
- [演示程序](#演示程序)

---

## 项目结构

```
discrete_math/
├── Cargo.toml            # 包元数据，无外部依赖
├── README.md             # 本文档
├── CHANGELOG.md          # 版本变更记录
├── LICENSE-MIT           # MIT 许可证
├── LICENSE-APACHE        # Apache-2.0 许可证
└── src/
    ├── lib.rs            # Crate 文档、模块声明、公共重导出
    ├── error.rs          # DiscreteError：全库统一错误类型
    ├── set.rs            # MathSet<T>：有限集合与集合代数
    ├── relation.rs       # BinaryRelation<A, B>：二元关系
    ├── function.rs       # DiscreteFunction<A, B>：函数映射
    └── main.rs           # 演示程序（cargo run）
```

模块之间的依赖是单向的，不存在循环引用：

```
error  ←  set  ←  relation  ←  function
```

`lib.rs` 在根路径重导出了主要类型，因此下面两种写法等价：

```rust
use discrete_math::set::MathSet;      // 按模块路径
use discrete_math::MathSet;           // 按根路径重导出
```

---

## 安装

已发布到 [crates.io](https://crates.io/crates/discrete_math)，直接加入依赖即可：

```bash
cargo add discrete_math
```

或在 `Cargo.toml` 中手写：

```toml
[dependencies]
discrete_math = "0.2"
```

本库**零依赖**，不引入任何第三方 crate。

## 快速开始

**环境要求**：Rust 1.85 及以上（本 crate 使用 `edition = "2024"`）。

```bash
# 运行测试
cargo test

# 运行演示程序，查看三大模块的实际输出
cargo run

# 生成并打开 API 文档
cargo doc --open
```

在其他项目中作为本地依赖使用：

```toml
[dependencies]
discrete_math = { path = "../discrete_math" }
```

---

## 核心概念与数学定义

### 1. 集合论（`set`）

集合 `MathSet<T>` 是元素的**无序、去重**聚集，底层由 `BTreeSet<T>` 支撑，因此元素类型需满足 `Ord + Clone`。
用有序容器换来两个关键能力：集合始终按升序迭代，并且 `MathSet<T>` 自身可比较、可哈希——这让
**集合的集合**（幂集 `MathSet<MathSet<T>>`、商集 `A/R`）成为可能。

| 概念 | 数学记号 | 方法 |
|------|---------|------|
| 属于 | $x \in A$ | `contains` |
| 基数 | $\lvert A \rvert$ | `cardinality` |
| 子集 | $A \subseteq B$ | `is_subset` |
| 真子集 | $A \subsetneq B$ | `is_proper_subset` |
| 并集 | $A \cup B$ | `union` |
| 交集 | $A \cap B$ | `intersection` |
| 差集 | $A \setminus B$ | `difference` |
| 对称差 | $A \triangle B$ | `symmetric_difference` |
| 补集 | $A^c = U \setminus A$ | `complement` |
| 幂集 | $\mathcal{P}(A)$ | `power_set` |
| 笛卡尔积 | $A \times B$ | `cartesian_product` |
| 划分验证 | $\{S_1,\dots,S_k\}$ 是否划分 $A$ | `is_valid_partition` |

**划分（Partition）** 需同时满足三条公理，`is_valid_partition` 会逐一校验：

1. 每个块非空：$\forall S \in P,\ S \neq \emptyset$
2. 块间两两不交：$S_i \neq S_j \implies S_i \cap S_j = \emptyset$
3. 并集还原原集合：$\bigcup_{S \in P} S = A$

### 2. 二元关系（`relation`）

二元关系被定义为笛卡尔积的子集：

$$
R \subseteq A \times B
$$

`BinaryRelation<A, B>` 同时持有**前域** $A$、**陪域** $B$ 与有序对集合 $R$。构造时逐对校验
$a \in A \land b \in B$，越界即返回 `ElementOutOfBounds`——关系永远不可能"指向集合之外"。

| 概念 | 数学定义 | 方法 |
|------|---------|------|
| 定义域 | $\text{Dom}(R)=\{a \mid \exists b,(a,b)\in R\}$ | `domain` |
| 值域 | $\text{Ran}(R)=\{b \mid \exists a,(a,b)\in R\}$ | `range` |
| 像集 | $R(S)=\{b \mid \exists a \in S,(a,b)\in R\}$ | `image` |
| 逆关系 | $R^{-1}=\{(b,a) \mid (a,b)\in R\}$ | `inverse` |
| 复合关系 | $S\circ R=\{(a,c) \mid \exists b,(a,b)\in R \land (b,c)\in S\}$ | `compose` |

**同质关系**（$R \subseteq A \times A$，别名 `HomogeneousRelation<T>`）额外提供：

| 性质 | 定义 | 方法 |
|------|------|------|
| 自反 | $\forall x,\ (x,x)\in R$ | `is_reflexive` |
| 反自反 | $\forall x,\ (x,x)\notin R$ | `is_irreflexive` |
| 对称 | $(x,y)\in R \implies (y,x)\in R$ | `is_symmetric` |
| 反对称 | $(x,y),(y,x)\in R \implies x=y$ | `is_antisymmetric` |
| 传递 | $(x,y),(y,z)\in R \implies (x,z)\in R$ | `is_transitive` |
| 自反闭包 | $r(R)=R\cup I_A$ | `reflexive_closure` |
| 对称闭包 | $s(R)=R\cup R^{-1}$ | `symmetric_closure` |
| 传递闭包 | $t(R)=\bigcup_{k\ge 1} R^k$ | `transitive_closure` |
| 等价关系 | 自反 ∧ 对称 ∧ 传递 | `is_equivalence` |
| 等价类 | $[x]_R=\{y \mid (x,y)\in R\}$ | `equivalence_class` |
| 商集 | $A/R=\{[x]_R \mid x \in A\}$ | `quotient_set` |
| 偏序关系 | 自反 ∧ 反对称 ∧ 传递 | `is_partial_order` |
| 全序关系 | 偏序 ∧ 任意两元素可比 | `is_total_order` |
| 极小 / 极大元 | 不存在严格更小 / 更大的元素 | `minimal_elements` / `maximal_elements` |
| 最小 / 最大元 | 与所有元素可比且最小 / 最大 | `least_element` / `greatest_element` |

> **注意**：「极小元」与「最小元」不同。在 $\{2,3,4,6\}$ 的整除偏序中，极小元为 $\{2,3\}$，但最小元不存在；
> 最小元要求与**所有**元素可比，极小元只要求没有元素比它更小。

### 3. 函数映射（`function`）

函数 $f: A \to B$ 在公理化集合论中就是一个特殊二元关系，需同时满足：

$$
\text{左全性：}\ \forall a \in A,\ \exists b \in B,\ (a,b) \in f
\qquad
\text{单值性：}\ (a,b_1),(a,b_2) \in f \implies b_1 = b_2
$$

`DiscreteFunction<A, B>` 内部只持有一个 `BinaryRelation<A, B>`，构造时强制校验上述两条，
因此任意一个 `DiscreteFunction` 实例都**天然是合法函数**，无需调用方额外检查。

| 概念 | 定义 | 方法 |
|------|------|------|
| 求值 | $f(a)$ | `eval` |
| 像集 | $\text{Im}(f)=\{f(a) \mid a \in A\}$ | `image` |
| 子集像 | $f(S)=\{f(a) \mid a \in S\}$ | `image_of` |
| 原像 | $f^{-1}(T)=\{a \mid f(a) \in T\}$ | `preimage_of` |
| 单射 | $f(x_1)=f(x_2) \implies x_1=x_2$ | `is_injective` |
| 满射 | $\text{Im}(f)=B$ | `is_surjective` |
| 双射 | 单射 ∧ 满射 | `is_bijective` |
| 复合 | $(g\circ f)(x)=g(f(x))$ | `compose` |
| 逆函数 | $f^{-1}$，仅双射存在 | `inverse` |

**核心定理**：二元关系 $f^{-1}$ 构成函数，当且仅当 $f$ 是双射。因此 `inverse()` 在非双射时返回
`Err(NotBijective)`，而双射情形下必有 $f^{-1}\circ f = \mathrm{id}_A$ 与 $f\circ f^{-1} = \mathrm{id}_B$。

---

## API 速查表

### `MathSet<T>`

| 分类 | 方法签名 |
|------|---------|
| 构造 | `new()`、`empty()`、`from_slice(&[T])` |
| 增删查 | `insert(T) -> bool`、`remove(&T) -> bool`、`contains(&T) -> bool` |
| 规模 | `cardinality() -> usize`、`len() -> usize`、`is_empty() -> bool`、`iter()` |
| 集合关系 | `is_subset`、`is_proper_subset`、`is_superset`、`is_disjoint` |
| 集合运算 | `union`、`intersection`、`difference`、`symmetric_difference`、`complement(&U)` |
| 构造性运算 | `power_set() -> MathSet<MathSet<T>>`、`cartesian_product(&B) -> MathSet<(T, U)>` |
| 划分 | `is_valid_partition(&MathSet<MathSet<T>>) -> bool` |

派生实现：`Clone`、`PartialEq`、`Eq`、`PartialOrd`、`Ord`、`Hash`、`Default`、`Debug`（打印为 `{1, 2, 3}`）、
`FromIterator<T>`、`IntoIterator`（含 `&MathSet<T>` 版本）。

### `BinaryRelation<A, B>`

| 分类 | 方法签名 |
|------|---------|
| 构造 | `new(A, B, pairs) -> Result<Self>`、`from_predicate(A, B, f) -> Self`、`empty(A, B)`、`universal(A, B)`、`identity(A)` |
| 访问 | `domain_set() -> &MathSet<A>`、`codomain_set() -> &MathSet<B>`、`pairs() -> &MathSet<(A, B)>`、`contains(&A, &B)` |
| 关系代数 | `domain()`、`range()`、`image(&S)`、`inverse() -> BinaryRelation<B, A>`、`compose(&S) -> Result<BinaryRelation<A, C>>` |
| 性质判定 | `is_reflexive`、`is_irreflexive`、`is_symmetric`、`is_antisymmetric`、`is_transitive` |
| 闭包 | `reflexive_closure()`、`symmetric_closure()`、`transitive_closure()` — 均返回 `Result` |
| 等价关系 | `is_equivalence`、`equivalence_class(&x) -> Result<MathSet<T>>`、`quotient_set() -> Result<MathSet<MathSet<T>>>` |
| 偏序关系 | `is_partial_order`、`is_total_order`、`minimal_elements`、`maximal_elements`、`least_element`、`greatest_element` |

### `DiscreteFunction<A, B>`

| 分类 | 方法签名 |
|------|---------|
| 构造 | `from_relation(BinaryRelation) -> Result<Self>`、`from_fn(A, B, \|&A\| -> B) -> Result<Self>`、`identity(A)` |
| 访问 | `as_relation() -> &BinaryRelation<A, B>`、`domain()`、`codomain()` |
| 求值 | `eval(&A) -> Option<&B>`、`image()`、`image_of(&S)`、`preimage_of(&T)` |
| 判定 | `is_injective`、`is_surjective`、`is_bijective` |
| 运算 | `compose(&g) -> Result<DiscreteFunction<A, C>>`、`inverse() -> Result<DiscreteFunction<B, A>>` |

---

## 使用示例

### 集合运算与德摩根律

```rust
use discrete_math::MathSet;

let u = MathSet::from_slice(&[1, 2, 3, 4, 5, 6]);
let a = MathSet::from_slice(&[1, 2, 3]);
let b = MathSet::from_slice(&[3, 4, 5]);

assert_eq!(a.union(&b), MathSet::from_slice(&[1, 2, 3, 4, 5]));
assert_eq!(a.intersection(&b), MathSet::from_slice(&[3]));
assert_eq!(a.difference(&b), MathSet::from_slice(&[1, 2]));

// (A ∪ B)^c = A^c ∩ B^c
let lhs = a.union(&b).complement(&u);
let rhs = a.complement(&u).intersection(&b.complement(&u));
assert_eq!(lhs, rhs);
```

### 幂集与笛卡尔积

```rust
use discrete_math::MathSet;

let a = MathSet::from_slice(&[1, 2, 3]);
assert_eq!(a.power_set().cardinality(), 8);            // 2^3

let b = MathSet::from_slice(&['x', 'y']);
let prod = a.cartesian_product(&b);
assert_eq!(prod.cardinality(), 6);                     // 3 × 2
assert!(prod.contains(&(2, 'y')));
```

### 等价关系与商集

```rust
use discrete_math::{BinaryRelation, MathSet};

let a = MathSet::from_slice(&[0, 1, 2, 3, 4, 5]);
let mod3 = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| x % 3 == y % 3);

assert!(mod3.is_equivalence());
assert_eq!(mod3.equivalence_class(&0).unwrap(), MathSet::from_slice(&[0, 3]));

let quotient = mod3.quotient_set().unwrap();           // {{0,3}, {1,4}, {2,5}}
assert_eq!(quotient.cardinality(), 3);
assert!(a.is_valid_partition(&quotient));              // 商集必为划分
```

### 偏序关系与极值元

```rust
use discrete_math::{BinaryRelation, MathSet};

let a = MathSet::from_slice(&[1, 2, 3, 4, 6, 12]);
let divides = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| y % x == 0);

assert!(divides.is_partial_order());
assert!(!divides.is_total_order());                    // 2 与 3 不可比
assert_eq!(divides.minimal_elements().unwrap(), MathSet::from_slice(&[1]));
assert_eq!(divides.greatest_element().unwrap(), Some(12));
```

### 从关系构造函数

```rust
use discrete_math::{BinaryRelation, DiscreteFunction, DiscreteError, MathSet};

let a = MathSet::from_slice(&[1, 2, 3]);
let b = MathSet::from_slice(&[10, 20, 30]);

// 缺少元素 3 的映射 —— 不满足左全性
let incomplete = BinaryRelation::new(
    a.clone(), b.clone(),
    MathSet::from_slice(&[(1, 10), (2, 20)]),
).unwrap();
assert_eq!(DiscreteFunction::from_relation(incomplete), Err(DiscreteError::NotLeftTotal));

// 一个前域元素对应两个像 —— 不满足单值性
let multi = BinaryRelation::new(
    a.clone(), b.clone(),
    MathSet::from_slice(&[(1, 10), (1, 20), (2, 20), (3, 30)]),
).unwrap();
assert_eq!(DiscreteFunction::from_relation(multi), Err(DiscreteError::NotRightUnique));
```

### 双射与逆函数

```rust
use discrete_math::{DiscreteFunction, MathSet};

let a = MathSet::from_slice(&[1, 2, 3]);
let b = MathSet::from_slice(&[10, 20, 30]);

let g = DiscreteFunction::from_fn(a.clone(), b.clone(), |x| x * 10).unwrap();
assert!(g.is_bijective());

let g_inv = g.inverse().unwrap();
assert_eq!(g_inv.eval(&20), Some(&2));

// 互逆即两侧恒等函数
assert_eq!(g.compose(&g_inv).unwrap(), DiscreteFunction::identity(a));
assert_eq!(g_inv.compose(&g).unwrap(), DiscreteFunction::identity(b));
```

### 用闭包直接构造（`from_fn`）

```rust
use discrete_math::{DiscreteFunction, MathSet};

let d = MathSet::from_slice(&[1, 2, 3, 4]);
let e = MathSet::from_slice(&[0, 1]);
let h = DiscreteFunction::from_fn(d, e, |x| x % 2).unwrap();

assert!(!h.is_injective());
assert!(h.is_surjective());
assert_eq!(h.preimage_of(&MathSet::from_slice(&[0])), MathSet::from_slice(&[2, 4]));
```

---

## 错误处理

所有可能失败的操作都返回 `Result<T, DiscreteError>`，`DiscreteError` 实现了 `Display` 与 `std::error::Error`：

| 变体 | 触发场景 |
|------|---------|
| `ElementOutOfBounds(String)` | 有序对含前域/陪域之外的元素；函数值落在陪域之外；对不存在的元素求等价类 |
| `IncompatibleDomains` | 复合时 `R` 的陪域 ≠ `S` 的前域 |
| `NotHomogeneousRelation` | 对非同质关系（$A \neq B$）求闭包 |
| `NotEquivalenceRelation` | 对非等价关系求等价类或商集 |
| `NotPartialOrder` | 对非偏序关系求极小/极大/最小/最大元 |
| `NotLeftTotal` | 关系存在前域元素无像，不能构成函数 |
| `NotRightUnique` | 关系存在一对多映射，不能构成函数 |
| `NotBijective` | 对非双射函数求逆 |

```rust
use discrete_math::{DiscreteFunction, MathSet};

let a = MathSet::from_slice(&[1, 2]);
let b = MathSet::from_slice(&[2, 4, 6]);
let f = DiscreteFunction::from_fn(a, b, |x| x * 2).unwrap();

// 单射但非满射：没有逆函数
let err = f.inverse().unwrap_err();
assert_eq!(err.to_string(), "函数不是双射，无法求逆函数");
```

---

## 设计说明与复杂度

**为什么用 `BTreeSet` 而不是 `HashSet`？**
`BTreeSet` 要求元素实现 `Ord`，代价是比哈希多一个对数因子；换来的是 `MathSet<T>` 自身实现了
`Ord`/`Hash`，于是幂集 `MathSet<MathSet<T>>`、商集、关系的有序对集合 `MathSet<(A, B)>` 都可以直接
作为集合元素，无需额外包装；同时迭代顺序稳定，调试输出（`{1, 2, 3}`）可预测。

**为什么关系要显式保存前域与陪域？**
若不保存，就无法区分"关系 $R$ 恰好没用到 $B$ 中的某元素"与"$R$ 的陪域更小"，
满射判定 `is_surjective`、恒等函数 `id_A` 以及复合的前域匹配检查都会失去依据。

**为什么构造函数要校验？**
类型系统只能保证"元素可比较"，无法保证"元素属于该集合"。`new` 与 `from_fn` 在构造点集中校验，
使后续所有运算都建立在已知不变式（invariant）之上。

**复杂度一览**（$n=\lvert A \rvert$，$m=\lvert B \rvert$，$r=\lvert R \rvert$，$s=\lvert S \rvert$）：

| 操作 | 复杂度 |
|------|--------|
| `contains` | $O(\log r)$ |
| `union` / `intersection` / `difference` | $O(n+m)$ |
| `power_set` | $O(2^n \cdot n)$ 时间与空间 |
| `cartesian_product` | $O(nm)$ |
| `is_valid_partition` | $O(k^2 \cdot n)$（$k$ 为块数） |
| `compose` | $O(rs)$ |
| `is_transitive` | $O(r^2 \log r)$ |
| `transitive_closure` | 不动点迭代，最坏 $O(n \cdot r^2)$ |
| `quotient_set` | $O(n^2 \log n)$ |

**已知边界**：所有结构都是**有限**集；幂集与传递闭包随规模指数/多项式增长，实际使用建议 $n \le 20$。
关系的复合与传递性判定为朴素双重循环，未做索引优化。

---

## 测试与文档

```bash
cargo test        # 12 个单元测试 + 7 个文档测试
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo doc --open  # 生成并打开 HTML API 文档
```

单元测试按模块就近放置（`src/*.rs` 末尾的 `#[cfg(test)] mod tests`），覆盖：

| 模块 | 测试内容 |
|------|---------|
| `set` | 去重与基数、子集/真子集、并交差对称差、德摩根律、幂集 $\lvert\mathcal{P}(A)\rvert=2^n$、笛卡尔积、划分三公理的合法与非法用例、迭代器往返 |
| `relation` | 五大性质判定、三大闭包、逆关系与 $(R^{-1})^{-1}=R$、复合与陪域不匹配报错、模 3 同余的等价类与商集、整除偏序的极值元、构造越界校验、空关系/全域关系 |
| `function` | 左全性/单值性报错、单射/满射/双射判定、逆函数、$g^{-1}\circ g=\mathrm{id}$、子集像与原像、函数值越界 |

文档测试直接写在公开 API 的 `///` 注释里（`cargo test` 会编译并执行它们），
因此 API 一旦改动、示例失配就会立刻报错；README 中的示例与这些测试用例保持一致。

---

## 演示程序

`cargo run` 会依次打印四个小节，实际输出如下（节选）：

```text
=== 一、集合代数 MathSet<T> ===
全集 U = {1, 2, 3, 4, 5, 6}
A = {1, 2, 3}, |A| = 3
A ∪ B = {1, 2, 3, 4, 5}
A △ B = {1, 2, 4, 5}
|P(A)| = 2^3 = 8
{{1, 2}, {3}} 是 A 的合法划分: true

=== 二、等价关系与商集 BinaryRelation<A, B> ===
A = {0, 1, 2, 3, 4, 5}, R 为模 3 同余关系, |R| = 12
自反: true, 对称: true, 传递: true
商集 A/R = {{0, 3}, {1, 4}, {2, 5}}
商集构成 A 的合法划分: true

=== 三、偏序关系与极值元 ===
A = {1, 2, 3, 4, 6, 12}, R 为整除关系 (x | y)
是否为偏序关系: true
是否为全序关系: false（2 与 3 不可比）
极小元: {1}
极大元: {12}

=== 四、函数、双射与逆函数 DiscreteFunction<A, B> ===
g: {1, 2, 3} -> {10, 20, 30}, g(x) = 10x
  双射: true, g(2) = Some(20)
  g⁻¹(20) = Some(2)
  g⁻¹ ∘ g = id_A: true
```

---

## 可扩展方向

- 集合：实现 `Sub`/`BitAnd`/`BitOr` 运算符重载，支持 `a & b` 这类写法。
- 关系：用邻接表或位矩阵加速复合与传递闭包，把 `O(n·r²)` 降到 $O(n^3/\text{word})$。
- 函数：支持无限/惰性定义域（`Fn` + 按需求值），摆脱"必须枚举全域"的限制。
- 序列化：为 `MathSet`、`BinaryRelation` 实现 `serde`（可作为可选 feature）。

---

## 许可证

本项目采用 `MIT OR Apache-2.0` 双许可，任选其一使用：

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

除非你明确声明，否则任何有意提交以包含在本项目中的贡献，均按上述双许可授权，不附加任何额外条款。
