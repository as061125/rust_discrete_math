//! 二元关系模块：将从集合 $A$ 到集合 $B$ 的二元关系定义为笛卡尔积的子集 $R \subseteq A \times B$，
//! 支持关系代数、五大性质判定、三大闭包、等价关系与商集、偏序关系与极值元分析。

use std::fmt;

use crate::DiscreteError;
use crate::set::MathSet;

/// 从前域（Domain Set）$A$ 到陪域（Codomain Set）$B$ 的二元关系 $R \subseteq A \times B$。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryRelation<A: Ord + Clone, B: Ord + Clone> {
    domain_set: MathSet<A>,
    codomain_set: MathSet<B>,
    pairs: MathSet<(A, B)>,
}

/// 集合 $A$ 上的同质二元关系（$R \subseteq A \times A$）类型别名
pub type HomogeneousRelation<T> = BinaryRelation<T, T>;

impl<A: Ord + Clone + fmt::Debug, B: Ord + Clone + fmt::Debug> BinaryRelation<A, B> {
    /// 构造二元关系 $R \subseteq A \times B$，并严格验证所有有序对 $(a, b)$ 均满足 $a \in A \land b \in B$
    pub fn new(
        domain_set: MathSet<A>,
        codomain_set: MathSet<B>,
        pairs: MathSet<(A, B)>,
    ) -> Result<Self, DiscreteError> {
        for (a, b) in &pairs {
            if !domain_set.contains(a) {
                return Err(DiscreteError::ElementOutOfBounds(format!(
                    "前域元素 {:?} 不在给定前域集合中",
                    a
                )));
            }
            if !codomain_set.contains(b) {
                return Err(DiscreteError::ElementOutOfBounds(format!(
                    "陪域元素 {:?} 不在给定陪域集合中",
                    b
                )));
            }
        }
        Ok(Self {
            domain_set,
            codomain_set,
            pairs,
        })
    }
}

impl<A: Ord + Clone, B: Ord + Clone> BinaryRelation<A, B> {
    /// 通过二元谓词 $P(a, b)$ 构造关系：$R = \{(a, b) \in A \times B \mid P(a, b)\}$
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete_math::{BinaryRelation, MathSet};
    ///
    /// let a = MathSet::from_slice(&[1, 2, 3, 4]);
    /// // 整除关系：x | y
    /// let divides = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| y % x == 0);
    ///
    /// assert!(divides.contains(&2, &4));
    /// assert!(!divides.contains(&4, &2));
    /// assert_eq!(divides.domain(), MathSet::from_slice(&[1, 2, 3, 4]));
    /// assert_eq!(divides.range(), MathSet::from_slice(&[1, 2, 3, 4]));
    /// ```
    pub fn from_predicate<F>(
        domain_set: MathSet<A>,
        codomain_set: MathSet<B>,
        mut predicate: F,
    ) -> Self
    where
        F: FnMut(&A, &B) -> bool,
    {
        let mut pairs = MathSet::new();
        for a in &domain_set {
            for b in &codomain_set {
                if predicate(a, b) {
                    pairs.insert((a.clone(), b.clone()));
                }
            }
        }
        Self {
            domain_set,
            codomain_set,
            pairs,
        }
    }

    /// 构造从 $A$ 到 $B$ 的空关系 $\emptyset$
    pub fn empty(domain_set: MathSet<A>, codomain_set: MathSet<B>) -> Self {
        Self {
            domain_set,
            codomain_set,
            pairs: MathSet::new(),
        }
    }

    /// 构造从 $A$ 到 $B$ 的全域关系 $A \times B$
    pub fn universal(domain_set: MathSet<A>, codomain_set: MathSet<B>) -> Self {
        let pairs = domain_set.cartesian_product(&codomain_set);
        Self {
            domain_set,
            codomain_set,
            pairs,
        }
    }

    /// 获取前域集合 $A$
    pub fn domain_set(&self) -> &MathSet<A> {
        &self.domain_set
    }

    /// 获取陪域集合 $B$
    pub fn codomain_set(&self) -> &MathSet<B> {
        &self.codomain_set
    }

    /// 获取关系的有序对集合 $R \subseteq A \times B$
    pub fn pairs(&self) -> &MathSet<(A, B)> {
        &self.pairs
    }

    /// 判定元素 $a$ 与 $b$ 是否满足关系 $aRb$（即 $(a, b) \in R$）
    pub fn contains(&self, a: &A, b: &B) -> bool {
        self.pairs.contains(&(a.clone(), b.clone()))
    }

    /// 求关系的定义域：$\text{Dom}(R) = \{a \in A \mid \exists b \in B, (a, b) \in R\}$
    pub fn domain(&self) -> MathSet<A> {
        self.pairs.iter().map(|(a, _)| a.clone()).collect()
    }

    /// 求关系的值域：$\text{Ran}(R) = \{b \in B \mid \exists a \in A, (a, b) \in R\}$
    pub fn range(&self) -> MathSet<B> {
        self.pairs.iter().map(|(_, b)| b.clone()).collect()
    }

    /// 求子集 $S \subseteq A$ 在关系 $R$ 下的像集：$R(S) = \{b \in B \mid \exists a \in S, (a, b) \in R\}$
    pub fn image(&self, subset: &MathSet<A>) -> MathSet<B> {
        self.pairs
            .iter()
            .filter(|(a, _)| subset.contains(a))
            .map(|(_, b)| b.clone())
            .collect()
    }

    /// 求逆关系：$R^{-1} = \{(b, a) \in B \times A \mid (a, b) \in R\}$
    pub fn inverse(&self) -> BinaryRelation<B, A> {
        let inv_pairs = self
            .pairs
            .iter()
            .map(|(a, b)| (b.clone(), a.clone()))
            .collect();
        BinaryRelation {
            domain_set: self.codomain_set.clone(),
            codomain_set: self.domain_set.clone(),
            pairs: inv_pairs,
        }
    }

    /// 关系的复合运算（$S \circ R$，其中 $R \subseteq A \times B$ 为 `self`，$S \subseteq B \times C$ 为 `other`）
    ///
    /// 数学定义：$S \circ R = \{(a, c) \in A \times C \mid \exists b \in B, (a, b) \in R \land (b, c) \in S\}$
    pub fn compose<C: Ord + Clone>(
        &self,
        other: &BinaryRelation<B, C>,
    ) -> Result<BinaryRelation<A, C>, DiscreteError> {
        if self.codomain_set != other.domain_set {
            return Err(DiscreteError::IncompatibleDomains);
        }
        let mut composed_pairs = MathSet::new();
        for (a, b1) in &self.pairs {
            for (b2, c) in &other.pairs {
                if b1 == b2 {
                    composed_pairs.insert((a.clone(), c.clone()));
                }
            }
        }
        Ok(BinaryRelation {
            domain_set: self.domain_set.clone(),
            codomain_set: other.codomain_set.clone(),
            pairs: composed_pairs,
        })
    }
}

// ------------------------------------------------------------------------
// 同质二元关系 (Homogeneous Relation: R ⊆ A × A) 的专有方法
// ------------------------------------------------------------------------

impl<T: Ord + Clone> BinaryRelation<T, T> {
    /// 构造集合 $A$ 上的恒等关系：$I_A = \{(x, x) \mid x \in A\}$
    pub fn identity(set: MathSet<T>) -> Self {
        let pairs = set.iter().map(|x| (x.clone(), x.clone())).collect();
        Self {
            domain_set: set.clone(),
            codomain_set: set,
            pairs,
        }
    }

    /// 校验前域与陪域是否一致（同质关系要求 $A = B$）
    fn ensure_homogeneous(&self) -> Result<(), DiscreteError> {
        if self.domain_set != self.codomain_set {
            Err(DiscreteError::NotHomogeneousRelation)
        } else {
            Ok(())
        }
    }

    /// 判定**自反性（Reflexive）**：$\forall x \in A, (x, x) \in R$
    pub fn is_reflexive(&self) -> bool {
        if self.domain_set != self.codomain_set {
            return false;
        }
        self.domain_set.iter().all(|x| self.contains(x, x))
    }

    /// 判定**反自反性（Irreflexive）**：$\forall x \in A, (x, x) \notin R$
    pub fn is_irreflexive(&self) -> bool {
        if self.domain_set != self.codomain_set {
            return false;
        }
        self.domain_set.iter().all(|x| !self.contains(x, x))
    }

    /// 判定**对称性（Symmetric）**：$\forall x, y \in A, (x, y) \in R \implies (y, x) \in R$
    pub fn is_symmetric(&self) -> bool {
        if self.domain_set != self.codomain_set {
            return false;
        }
        self.pairs.iter().all(|(x, y)| self.contains(y, x))
    }

    /// 判定**反对称性（Antisymmetric）**：$\forall x, y \in A, ((x, y) \in R \land (y, x) \in R) \implies x = y$
    pub fn is_antisymmetric(&self) -> bool {
        if self.domain_set != self.codomain_set {
            return false;
        }
        self.pairs
            .iter()
            .all(|(x, y)| x == y || !self.contains(y, x))
    }

    /// 判定**传递性（Transitive）**：$\forall x, y, z \in A, ((x, y) \in R \land (y, z) \in R) \implies (x, z) \in R$
    pub fn is_transitive(&self) -> bool {
        if self.domain_set != self.codomain_set {
            return false;
        }
        for (x, y1) in &self.pairs {
            for (y2, z) in &self.pairs {
                if y1 == y2 && !self.contains(x, z) {
                    return false;
                }
            }
        }
        true
    }

    /// 求**自反闭包（Reflexive Closure）**：$r(R) = R \cup I_A$
    pub fn reflexive_closure(&self) -> Result<Self, DiscreteError> {
        self.ensure_homogeneous()?;
        let id = Self::identity(self.domain_set.clone());
        Ok(Self {
            domain_set: self.domain_set.clone(),
            codomain_set: self.codomain_set.clone(),
            pairs: self.pairs.union(&id.pairs),
        })
    }

    /// 求**对称闭包（Symmetric Closure）**：$s(R) = R \cup R^{-1}$
    pub fn symmetric_closure(&self) -> Result<Self, DiscreteError> {
        self.ensure_homogeneous()?;
        let inv = self.inverse();
        Ok(Self {
            domain_set: self.domain_set.clone(),
            codomain_set: self.codomain_set.clone(),
            pairs: self.pairs.union(&inv.pairs),
        })
    }

    /// 求**传递闭包（Transitive Closure）**：$t(R) = R^+ = \bigcup_{k=1}^{|A|} R^k$
    ///
    /// 采用不动点迭代法（等价于有限集上的 Warshall 传递闭包算法）。
    pub fn transitive_closure(&self) -> Result<Self, DiscreteError> {
        self.ensure_homogeneous()?;
        let mut current_pairs = self.pairs.clone();
        loop {
            let mut next_pairs = current_pairs.clone();
            for (x, y1) in &current_pairs {
                for (y2, z) in &current_pairs {
                    if y1 == y2 {
                        next_pairs.insert((x.clone(), z.clone()));
                    }
                }
            }
            if next_pairs.len() == current_pairs.len() {
                break;
            }
            current_pairs = next_pairs;
        }
        Ok(Self {
            domain_set: self.domain_set.clone(),
            codomain_set: self.codomain_set.clone(),
            pairs: current_pairs,
        })
    }

    /// 判定是否为**等价关系（Equivalence Relation）**：同时满足自反性、对称性与传递性
    pub fn is_equivalence(&self) -> bool {
        self.is_reflexive() && self.is_symmetric() && self.is_transitive()
    }

    /// 求元素 $x \in A$ 关于等价关系 $R$ 的**等价类（Equivalence Class）**：$\[x\]_R = \{y \in A \mid (x, y) \in R\}$
    pub fn equivalence_class(&self, element: &T) -> Result<MathSet<T>, DiscreteError>
    where
        T: fmt::Debug,
    {
        if !self.is_equivalence() {
            return Err(DiscreteError::NotEquivalenceRelation);
        }
        if !self.domain_set.contains(element) {
            return Err(DiscreteError::ElementOutOfBounds(format!(
                "元素 {:?} 不在集合中",
                element
            )));
        }
        let class = self
            .domain_set
            .iter()
            .filter(|y| self.contains(element, y))
            .cloned()
            .collect();
        Ok(class)
    }

    /// 求集合 $A$ 关于等价关系 $R$ 的**商集（Quotient Set）**：$A / R = \{\[x\]_R \mid x \in A\}$
    ///
    /// 离散数学基本定理：等价关系 $R$ 的商集 $A / R$ 必然构成集合 $A$ 的一个合法划分（Partition）。
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete_math::{BinaryRelation, MathSet};
    ///
    /// let a = MathSet::from_slice(&[0, 1, 2, 3, 4, 5]);
    /// // 模 3 同余关系
    /// let r = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| x % 3 == y % 3);
    ///
    /// assert!(r.is_equivalence());
    /// assert_eq!(r.equivalence_class(&0).unwrap(), MathSet::from_slice(&[0, 3]));
    ///
    /// let quotient = r.quotient_set().unwrap();
    /// assert_eq!(quotient.cardinality(), 3);
    /// // 商集必构成原集合的一个合法划分
    /// assert!(a.is_valid_partition(&quotient));
    /// ```
    pub fn quotient_set(&self) -> Result<MathSet<MathSet<T>>, DiscreteError>
    where
        T: fmt::Debug,
    {
        if !self.is_equivalence() {
            return Err(DiscreteError::NotEquivalenceRelation);
        }
        let mut quotient = MathSet::new();
        for x in &self.domain_set {
            quotient.insert(self.equivalence_class(x)?);
        }
        Ok(quotient)
    }

    /// 判定是否为**偏序关系（Partial Order）**：同时满足自反性、反对称性与传递性
    pub fn is_partial_order(&self) -> bool {
        self.is_reflexive() && self.is_antisymmetric() && self.is_transitive()
    }

    /// 判定是否为**全序关系 / 线序关系（Total Order / Linear Order）**：
    /// 既是偏序关系，且集合中任意两个元素均具有可比性（$\forall x, y \in A, xRy \lor yRx$）。
    pub fn is_total_order(&self) -> bool {
        if !self.is_partial_order() {
            return false;
        }
        for x in &self.domain_set {
            for y in &self.domain_set {
                if !self.contains(x, y) && !self.contains(y, x) {
                    return false;
                }
            }
        }
        true
    }

    /// 求偏序集 $(A, \preceq)$ 的所有**极小元（Minimal Elements）**
    ///
    /// 定义：$m \in A$ 为极小元 $\iff \neg \exists x \in A (x \neq m \land x \preceq m)$
    pub fn minimal_elements(&self) -> Result<MathSet<T>, DiscreteError> {
        if !self.is_partial_order() {
            return Err(DiscreteError::NotPartialOrder);
        }
        let minimals = self
            .domain_set
            .iter()
            .filter(|m| {
                !self
                    .domain_set
                    .iter()
                    .any(|x| x != *m && self.contains(x, m))
            })
            .cloned()
            .collect();
        Ok(minimals)
    }

    /// 求偏序集 $(A, \preceq)$ 的所有**极大元（Maximal Elements）**
    ///
    /// 定义：$M \in A$ 为极大元 $\iff \neg \exists x \in A (x \neq M \land M \preceq x)$
    pub fn maximal_elements(&self) -> Result<MathSet<T>, DiscreteError> {
        if !self.is_partial_order() {
            return Err(DiscreteError::NotPartialOrder);
        }
        let maximals = self
            .domain_set
            .iter()
            .filter(|m| {
                !self
                    .domain_set
                    .iter()
                    .any(|x| x != *m && self.contains(m, x))
            })
            .cloned()
            .collect();
        Ok(maximals)
    }

    /// 求偏序集 $(A, \preceq)$ 的**最小元（Least Element）**
    ///
    /// 定义：$l \in A$ 为最小元 $\iff \forall x \in A, l \preceq x$（若存在则唯一）
    pub fn least_element(&self) -> Result<Option<T>, DiscreteError> {
        if !self.is_partial_order() {
            return Err(DiscreteError::NotPartialOrder);
        }
        let least = self
            .domain_set
            .iter()
            .find(|l| self.domain_set.iter().all(|x| self.contains(l, x)))
            .cloned();
        Ok(least)
    }

    /// 求偏序集 $(A, \preceq)$ 的**最大元（Greatest Element）**
    ///
    /// 定义：$g \in A$ 为最大元 $\iff \forall x \in A, x \preceq g$（若存在则唯一）
    pub fn greatest_element(&self) -> Result<Option<T>, DiscreteError> {
        if !self.is_partial_order() {
            return Err(DiscreteError::NotPartialOrder);
        }
        let greatest = self
            .domain_set
            .iter()
            .find(|g| self.domain_set.iter().all(|x| self.contains(x, g)))
            .cloned();
        Ok(greatest)
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryRelation;
    use crate::set::MathSet;

    #[test]
    fn test_relation_closures() {
        let a = MathSet::from_slice(&[1, 2, 3]);
        // R = {(1, 2), (2, 3)}
        let pairs = MathSet::from_slice(&[(1, 2), (2, 3)]);
        let r = BinaryRelation::new(a.clone(), a.clone(), pairs).unwrap();

        assert!(!r.is_reflexive());
        assert!(r.is_irreflexive());
        assert!(!r.is_symmetric());
        assert!(r.is_antisymmetric());
        assert!(!r.is_transitive());

        // 自反闭包 r(R)
        let refl = r.reflexive_closure().unwrap();
        assert!(refl.is_reflexive());
        assert_eq!(refl.pairs().cardinality(), 2 + 3);

        // 对称闭包 s(R)
        let sym = r.symmetric_closure().unwrap();
        assert!(sym.is_symmetric());
        assert!(sym.contains(&2, &1) && sym.contains(&3, &2));

        // 传递闭包 t(R) 应包含 (1, 3)
        let trans = r.transitive_closure().unwrap();
        assert!(trans.is_transitive());
        assert!(trans.contains(&1, &3));
        assert_eq!(trans.pairs().cardinality(), 3);
    }

    #[test]
    fn test_relation_inverse_and_composition() {
        let a = MathSet::from_slice(&[1, 2, 3]);
        let b = MathSet::from_slice(&['x', 'y']);
        let c = MathSet::from_slice(&[10, 20]);

        // R ⊆ A × B, S ⊆ B × C
        let r = BinaryRelation::new(
            a.clone(),
            b.clone(),
            MathSet::from_slice(&[(1, 'x'), (2, 'x'), (3, 'y')]),
        )
        .unwrap();
        let s = BinaryRelation::new(
            b.clone(),
            c.clone(),
            MathSet::from_slice(&[('x', 10), ('y', 20)]),
        )
        .unwrap();

        // 逆关系 R^{-1} ⊆ B × A
        let r_inv = r.inverse();
        assert!(r_inv.contains(&'x', &1));
        assert_eq!(r_inv.domain_set(), &b);
        // (R^{-1})^{-1} = R
        assert_eq!(r_inv.inverse(), r);

        // 复合关系 S ∘ R ⊆ A × C
        let composed = r.compose(&s).unwrap();
        assert!(composed.contains(&1, &10));
        assert!(composed.contains(&3, &20));
        assert_eq!(composed.pairs().cardinality(), 3);

        // 前域与陪域不匹配时拒绝复合：R 的陪域 {x, y} ≠ T 的前域 {x}
        let t = BinaryRelation::new(
            MathSet::from_slice(&['x']),
            c.clone(),
            MathSet::from_slice(&[('x', 10)]),
        )
        .unwrap();
        assert_eq!(
            r.compose(&t),
            Err(crate::DiscreteError::IncompatibleDomains)
        );
    }

    #[test]
    fn test_equivalence_relation_and_quotient_set() {
        // 集合 A = {0, 1, 2, 3, 4, 5} 上的模 3 同余关系：x ≡ y (mod 3)
        let a = MathSet::from_slice(&[0, 1, 2, 3, 4, 5]);
        let mod3_rel = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| x % 3 == y % 3);

        assert!(mod3_rel.is_equivalence());

        // [0]_R = {0, 3}, [1]_R = {1, 4}, [2]_R = {2, 5}
        assert_eq!(
            mod3_rel.equivalence_class(&0).unwrap(),
            MathSet::from_slice(&[0, 3])
        );
        assert_eq!(
            mod3_rel.equivalence_class(&4).unwrap(),
            MathSet::from_slice(&[1, 4])
        );

        // 商集 A / R 必须构成集合 A 的一个合法划分
        let quotient = mod3_rel.quotient_set().unwrap();
        assert_eq!(quotient.cardinality(), 3);
        assert!(a.is_valid_partition(&quotient));
    }

    #[test]
    fn test_partial_order_and_extrema() {
        // 集合 A = {1, 2, 3, 4, 6, 12} 上的整除关系 (x | y)
        let a = MathSet::from_slice(&[1, 2, 3, 4, 6, 12]);
        let divides = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| y % x == 0);

        assert!(divides.is_partial_order());
        // 2 和 3 不可比（2 ∤ 3 且 3 ∤ 2），故不是全序关系
        assert!(!divides.is_total_order());

        // 极小元与最小元均为 1，极大元与最大元均为 12
        assert_eq!(
            divides.minimal_elements().unwrap(),
            MathSet::from_slice(&[1])
        );
        assert_eq!(
            divides.maximal_elements().unwrap(),
            MathSet::from_slice(&[12])
        );
        assert_eq!(divides.least_element().unwrap(), Some(1));
        assert_eq!(divides.greatest_element().unwrap(), Some(12));

        // 若去掉 1 和 12，在 {2, 3, 4, 6} 上考察整除偏序：
        let b = MathSet::from_slice(&[2, 3, 4, 6]);
        let divides_b = BinaryRelation::from_predicate(b.clone(), b.clone(), |x, y| y % x == 0);
        assert!(divides_b.is_partial_order());
        assert_eq!(
            divides_b.minimal_elements().unwrap(),
            MathSet::from_slice(&[2, 3])
        );
        assert_eq!(
            divides_b.maximal_elements().unwrap(),
            MathSet::from_slice(&[4, 6])
        );
        // 此时不存在唯一最小元和最大元
        assert_eq!(divides_b.least_element().unwrap(), None);
        assert_eq!(divides_b.greatest_element().unwrap(), None);

        // 标准小于等于关系 (<=) 是全序关系
        let leq = BinaryRelation::from_predicate(b.clone(), b.clone(), |x, y| x <= y);
        assert!(leq.is_total_order());
    }

    #[test]
    fn test_relation_construction_validation() {
        let a = MathSet::from_slice(&[1, 2]);

        // 有序对中含前域之外的元素 -> 构造失败
        let bad = BinaryRelation::new(a.clone(), a.clone(), MathSet::from_slice(&[(1, 2), (3, 2)]));
        assert!(bad.is_err());

        // 空关系与全域关系
        let empty_rel = BinaryRelation::empty(a.clone(), a.clone());
        assert_eq!(empty_rel.pairs().cardinality(), 0);

        let universal = BinaryRelation::universal(a.clone(), a.clone());
        assert_eq!(universal.pairs().cardinality(), 4);
    }
}
