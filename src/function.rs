//! 函数模块：在二元关系的基础上，将函数 $f: A \to B$ 严格定义为满足**左全性**与**单值性**的二元关系，
//! 支持函数求值、像/原像计算、单射/满射/双射判定、函数复合与逆函数求解。

use std::fmt;

use crate::DiscreteError;
use crate::relation::BinaryRelation;
use crate::set::MathSet;

/// 离散数学中的函数 $f: A \to B$。
///
/// 在公理化集合论中，函数 $f: A \to B$ 本质上是一个二元关系 $f \subseteq A \times B$，且满足：
/// 1. **左全性（Left-total / 全域性）**：$\forall a \in A, \exists b \in B, (a, b) \in f$
/// 2. **右唯一性（Right-unique / 单值性）**：$\forall a \in A, \forall b_1, b_2 \in B, ((a, b_1) \in f \land (a, b_2) \in f) \implies b_1 = b_2$
///
/// # Examples
///
/// ```
/// use discrete_math::{DiscreteFunction, MathSet};
///
/// let a = MathSet::from_slice(&[1, 2, 3]);
/// let b = MathSet::from_slice(&[10, 20, 30]);
/// let f = DiscreteFunction::from_fn(a, b, |x| x * 10).unwrap();
///
/// assert!(f.is_bijective());
/// assert_eq!(f.eval(&2), Some(&20));
/// assert_eq!(f.inverse().unwrap().eval(&20), Some(&2));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscreteFunction<A: Ord + Clone, B: Ord + Clone> {
    relation: BinaryRelation<A, B>,
}

impl<A: Ord + Clone, B: Ord + Clone> DiscreteFunction<A, B> {
    /// 从二元关系 $R \subseteq A \times B$ 构造一等函数 $f: A \to B$。
    ///
    /// 若关系不满足左全性则返回 `Err(DiscreteError::NotLeftTotal)`；
    /// 若关系不满足单值性则返回 `Err(DiscreteError::NotRightUnique)`。
    pub fn from_relation(relation: BinaryRelation<A, B>) -> Result<Self, DiscreteError> {
        for a in relation.domain_set() {
            let mut matched_count = 0usize;
            for (x, _) in relation.pairs() {
                if x == a {
                    matched_count += 1;
                }
            }
            if matched_count == 0 {
                return Err(DiscreteError::NotLeftTotal);
            }
            if matched_count > 1 {
                return Err(DiscreteError::NotRightUnique);
            }
        }
        Ok(Self { relation })
    }

    /// 使用 Rust 闭包 `f: Fn(&A) -> B` 在有限定义域 $A$ 与陪域 $B$ 上构造函数
    pub fn from_fn<F>(
        domain: MathSet<A>,
        codomain: MathSet<B>,
        mut f: F,
    ) -> Result<Self, DiscreteError>
    where
        B: fmt::Debug,
        A: fmt::Debug,
        F: FnMut(&A) -> B,
    {
        let mut pairs = MathSet::new();
        for a in &domain {
            let b = f(a);
            if !codomain.contains(&b) {
                return Err(DiscreteError::ElementOutOfBounds(format!(
                    "函数值 f({:?}) = {:?} 不在陪域 B 中",
                    a, b
                )));
            }
            pairs.insert((a.clone(), b));
        }
        let relation = BinaryRelation::new(domain, codomain, pairs)?;
        Self::from_relation(relation)
    }

    /// 获取底层的二元关系表示 $R \subseteq A \times B$
    pub fn as_relation(&self) -> &BinaryRelation<A, B> {
        &self.relation
    }

    /// 获取函数的定义域（Domain）$A$
    pub fn domain(&self) -> &MathSet<A> {
        self.relation.domain_set()
    }

    /// 获取函数的陪域（Codomain）$B$
    pub fn codomain(&self) -> &MathSet<B> {
        self.relation.codomain_set()
    }

    /// 计算函数在元素 $a \in A$ 处的函数值 $f(a)$
    pub fn eval(&self, a: &A) -> Option<&B> {
        self.relation
            .pairs()
            .iter()
            .find(|(x, _)| x == a)
            .map(|(_, b)| b)
    }

    /// 求函数的像集（Image / Range）：$\text{Im}(f) = \{f(a) \mid a \in A\}$
    pub fn image(&self) -> MathSet<B> {
        self.relation.range()
    }

    /// 求子集 $S \subseteq A$ 的像：$f(S) = \{f(a) \mid a \in S\}$
    pub fn image_of(&self, subset: &MathSet<A>) -> MathSet<B> {
        self.relation.image(subset)
    }

    /// 求子集 $T \subseteq B$ 的原像（逆像，Preimage）：$f^{-1}(T) = \{a \in A \mid f(a) \in T\}$
    pub fn preimage_of(&self, subset: &MathSet<B>) -> MathSet<A> {
        self.relation
            .pairs()
            .iter()
            .filter(|(_, b)| subset.contains(b))
            .map(|(a, _)| a.clone())
            .collect()
    }

    /// 判定函数是否为**单射（Injective / One-to-One）**
    ///
    /// 数学定义：$\forall x_1, x_2 \in A, f(x_1) = f(x_2) \implies x_1 = x_2$
    /// 在有限集上等价于 $|\text{Im}(f)| = |A|$。
    pub fn is_injective(&self) -> bool {
        self.image().cardinality() == self.domain().cardinality()
    }

    /// 判定函数是否为**满射（Surjective / Onto）**
    ///
    /// 数学定义：$\forall b \in B, \exists a \in A, f(a) = b$（即 $\text{Im}(f) = B$）
    pub fn is_surjective(&self) -> bool {
        &self.image() == self.codomain()
    }

    /// 判定函数是否为**双射（Bijective / One-to-One Correspondence）**
    ///
    /// 数学定义：既是单射又是满射。
    pub fn is_bijective(&self) -> bool {
        self.is_injective() && self.is_surjective()
    }

    /// 函数的复合运算：给定 $f: A \to B$（`self`）与 $g: B \to C$（`other`），
    /// 构造复合函数 $g \circ f: A \to C$，满足 $(g \circ f)(x) = g(f(x))$。
    ///
    /// 底层直接复用二元关系的复合运算 `self.relation.compose(&other.relation)`，
    /// 体现了“两个函数的复合作为二元关系的复合依然是函数”这一公理性质。
    pub fn compose<C: Ord + Clone>(
        &self,
        other: &DiscreteFunction<B, C>,
    ) -> Result<DiscreteFunction<A, C>, DiscreteError> {
        let composed_rel = self.relation.compose(&other.relation)?;
        DiscreteFunction::from_relation(composed_rel)
    }

    /// 求双射函数的**逆函数（Inverse Function）** $f^{-1}: B \to A$
    ///
    /// 定理：二元关系 $f^{-1}$ 构成函数当且仅当 $f$ 是双射（Bijection）。
    pub fn inverse(&self) -> Result<DiscreteFunction<B, A>, DiscreteError> {
        if !self.is_bijective() {
            return Err(DiscreteError::NotBijective);
        }
        let inv_rel = self.relation.inverse();
        DiscreteFunction::from_relation(inv_rel)
    }
}

impl<T: Ord + Clone> DiscreteFunction<T, T> {
    /// 构造集合 $A$ 上的恒等函数 $\text{id}_A: A \to A, \text{id}_A(x) = x$
    pub fn identity(set: MathSet<T>) -> Self {
        Self {
            relation: BinaryRelation::identity(set),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DiscreteFunction;
    use crate::DiscreteError;
    use crate::relation::BinaryRelation;
    use crate::set::MathSet;

    #[test]
    fn test_relation_to_function_validation() {
        let a = MathSet::from_slice(&[1, 2, 3]);
        let b = MathSet::from_slice(&[10, 20, 30]);

        // 1) 缺少元素 3 的映射 -> 不满足左全性 (NotLeftTotal)
        let incomplete_rel = BinaryRelation::new(
            a.clone(),
            b.clone(),
            MathSet::from_slice(&[(1, 10), (2, 20)]),
        )
        .unwrap();
        assert_eq!(
            DiscreteFunction::from_relation(incomplete_rel),
            Err(DiscreteError::NotLeftTotal)
        );

        // 2) 元素 1 对应 10 和 20 -> 不满足单值性 (NotRightUnique)
        let multi_valued_rel = BinaryRelation::new(
            a.clone(),
            b.clone(),
            MathSet::from_slice(&[(1, 10), (1, 20), (2, 20), (3, 30)]),
        )
        .unwrap();
        assert_eq!(
            DiscreteFunction::from_relation(multi_valued_rel),
            Err(DiscreteError::NotRightUnique)
        );

        // 3) 合法函数
        let valid_rel = BinaryRelation::new(
            a.clone(),
            b.clone(),
            MathSet::from_slice(&[(1, 10), (2, 20), (3, 30)]),
        )
        .unwrap();
        let func = DiscreteFunction::from_relation(valid_rel).unwrap();
        assert_eq!(func.eval(&2), Some(&20));
    }

    #[test]
    fn test_function_properties_composition_and_inverse() {
        let a = MathSet::from_slice(&[1, 2, 3]);
        let b = MathSet::from_slice(&[2, 4, 6, 8]);

        // f: {1, 2, 3} -> {2, 4, 6, 8}, f(x) = 2x （单射但非满射）
        let f = DiscreteFunction::from_fn(a.clone(), b.clone(), |x| x * 2).unwrap();
        assert!(f.is_injective());
        assert!(!f.is_surjective());
        assert!(!f.is_bijective());
        assert_eq!(f.inverse(), Err(DiscreteError::NotBijective));

        // g: {1, 2, 3} -> {10, 20, 30}, g(x) = 10x （双射）
        let c = MathSet::from_slice(&[10, 20, 30]);
        let g = DiscreteFunction::from_fn(a.clone(), c.clone(), |x| x * 10).unwrap();
        assert!(g.is_injective());
        assert!(g.is_surjective());
        assert!(g.is_bijective());

        // 求双射函数 g 的逆函数 g^{-1}: {10, 20, 30} -> {1, 2, 3}
        let g_inv = g.inverse().unwrap();
        assert_eq!(g_inv.eval(&20), Some(&2));

        // 验证 g^{-1} ∘ g = id_A 且 g ∘ g^{-1} = id_C
        let id_a = g.compose(&g_inv).unwrap();
        let id_c = g_inv.compose(&g).unwrap();
        assert_eq!(id_a, DiscreteFunction::identity(a.clone()));
        assert_eq!(id_c, DiscreteFunction::identity(c.clone()));

        // 原像测试：h: {1, 2, 3, 4} -> {0, 1}, h(x) = x % 2 （满射但非单射）
        let d = MathSet::from_slice(&[1, 2, 3, 4]);
        let e = MathSet::from_slice(&[0, 1]);
        let h = DiscreteFunction::from_fn(d, e, |x| x % 2).unwrap();
        assert!(!h.is_injective());
        assert!(h.is_surjective());
        assert_eq!(
            h.preimage_of(&MathSet::from_slice(&[0])),
            MathSet::from_slice(&[2, 4])
        );
    }

    #[test]
    fn test_function_image_of_and_out_of_bounds() {
        let a = MathSet::from_slice(&[1, 2, 3, 4]);
        let b = MathSet::from_slice(&[0, 1]);
        let f = DiscreteFunction::from_fn(a.clone(), b.clone(), |x| x % 2).unwrap();

        // 子集上的像：f({1, 3}) = {1}
        assert_eq!(
            f.image_of(&MathSet::from_slice(&[1, 3])),
            MathSet::from_slice(&[1])
        );
        assert_eq!(f.image(), MathSet::from_slice(&[0, 1]));

        // 函数值落在陪域之外 -> 构造失败
        let out_of_bounds = DiscreteFunction::from_fn(a, MathSet::from_slice(&[0]), |x| x * 10);
        assert!(matches!(
            out_of_bounds,
            Err(DiscreteError::ElementOutOfBounds(_))
        ));
    }
}
