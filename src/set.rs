//! 集合论模块：提供有限集合 `MathSet<T>` 及其代数运算、幂集、笛卡尔积与划分验证。

use std::collections::BTreeSet;
use std::fmt;
use std::iter::FromIterator;

/// 离散数学中的有限集合（基于 `BTreeSet<T>` 实现）。
///
/// 采用 `BTreeSet` 作为底层容器，使得 `MathSet<T>` 自身自动实现 `Ord` 与 `Eq`，
/// 从而天然支持**集合的集合**（如幂集 `MathSet<MathSet<T>>` 与商集 `A / R`）。
///
/// # Examples
///
/// ```
/// use discrete_math::MathSet;
///
/// let a = MathSet::from_slice(&[1, 2, 3]);
/// let b = MathSet::from_slice(&[3, 4]);
///
/// // 集合自动去重并按序存储
/// assert_eq!(MathSet::from_slice(&[1, 1, 2]), MathSet::from_slice(&[1, 2]));
///
/// assert_eq!(a.union(&b), MathSet::from_slice(&[1, 2, 3, 4]));
/// assert_eq!(a.intersection(&b), MathSet::from_slice(&[3]));
/// assert!(MathSet::from_slice(&[1, 2]).is_proper_subset(&a));
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MathSet<T: Ord + Clone> {
    elements: BTreeSet<T>,
}

impl<T: Ord + Clone> MathSet<T> {
    /// 创建一个空集 $\emptyset$
    pub fn new() -> Self {
        Self {
            elements: BTreeSet::new(),
        }
    }

    /// 创建一个空集 $\emptyset$（语义别名）
    pub fn empty() -> Self {
        Self::new()
    }

    /// 从切片构造集合（自动去重并排序）
    pub fn from_slice(slice: &[T]) -> Self {
        Self {
            elements: slice.iter().cloned().collect(),
        }
    }

    /// 向集合中插入元素 $x$，返回是否成功新增
    pub fn insert(&mut self, value: T) -> bool {
        self.elements.insert(value)
    }

    /// 从集合中移除元素 $x$，返回该元素原先是否存在
    pub fn remove(&mut self, value: &T) -> bool {
        self.elements.remove(value)
    }

    /// 判定元素是否属于集合（$x \in A$）
    pub fn contains(&self, value: &T) -> bool {
        self.elements.contains(value)
    }

    /// 返回集合的基数（Cardinality，即元素个数 $|A|$）
    pub fn cardinality(&self) -> usize {
        self.elements.len()
    }

    /// 返回集合元素个数（等同于 `cardinality`）
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// 判定是否为空集（$A = \emptyset$）
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// 返回集合元素的只读迭代器
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    /// 判定当前集合是否为 `other` 的子集（$A \subseteq B$）
    ///
    /// 数学定义：$\forall x (x \in A \implies x \in B)$
    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.is_subset(&other.elements)
    }

    /// 判定当前集合是否为 `other` 的真子集（$A \subsetneq B$）
    ///
    /// 数学定义：$A \subseteq B \land A \neq B$
    pub fn is_proper_subset(&self, other: &Self) -> bool {
        self.is_subset(other) && self != other
    }

    /// 判定当前集合是否为 `other` 的超集（$A \supseteq B$）
    pub fn is_superset(&self, other: &Self) -> bool {
        self.elements.is_superset(&other.elements)
    }

    /// 判定两个集合是否不相交（$A \cap B = \emptyset$）
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.elements.is_disjoint(&other.elements)
    }

    /// 并集运算（$A \cup B = \{x \mid x \in A \lor x \in B\}$）
    pub fn union(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.union(&other.elements).cloned().collect(),
        }
    }

    /// 交集运算（$A \cap B = \{x \mid x \in A \land x \in B\}$）
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            elements: self
                .elements
                .intersection(&other.elements)
                .cloned()
                .collect(),
        }
    }

    /// 差集运算（$A \setminus B = \{x \mid x \in A \land x \notin B\}$）
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.difference(&other.elements).cloned().collect(),
        }
    }

    /// 对称差运算（$A \triangle B = (A \setminus B) \cup (B \setminus A)$）
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        Self {
            elements: self
                .elements
                .symmetric_difference(&other.elements)
                .cloned()
                .collect(),
        }
    }

    /// 相对于全集 $U$ 的补集（$A^c = U \setminus A$）
    pub fn complement(&self, universe: &Self) -> Self {
        universe.difference(self)
    }

    /// 求幂集（Power Set，$\mathcal{P}(A) = \{S \mid S \subseteq A\}$）
    ///
    /// 若 $|A| = n$，则 $|\mathcal{P}(A)| = 2^n$。
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete_math::MathSet;
    ///
    /// let p = MathSet::from_slice(&[1, 2]).power_set();
    /// assert_eq!(p.cardinality(), 4);
    /// assert!(p.contains(&MathSet::empty()));
    /// assert!(p.contains(&MathSet::from_slice(&[1, 2])));
    /// ```
    pub fn power_set(&self) -> MathSet<MathSet<T>> {
        let mut subsets: Vec<MathSet<T>> = vec![MathSet::new()];
        for elem in &self.elements {
            let current_len = subsets.len();
            for i in 0..current_len {
                let mut next_subset = subsets[i].clone();
                next_subset.insert(elem.clone());
                subsets.push(next_subset);
            }
        }
        subsets.into_iter().collect()
    }

    /// 笛卡尔积（Cartesian Product，$A \times B = \{(a, b) \mid a \in A \land b \in B\}$）
    ///
    /// 笛卡尔积是连接**集合**与**二元关系**的桥梁：任何从 $A$ 到 $B$ 的二元关系都是 $A \times B$ 的子集。
    pub fn cartesian_product<U: Ord + Clone>(&self, other: &MathSet<U>) -> MathSet<(T, U)> {
        let mut product = MathSet::new();
        for a in &self.elements {
            for b in &other.elements {
                product.insert((a.clone(), b.clone()));
            }
        }
        product
    }

    /// 验证给定的集合族 `partition` 是否构成当前集合 $A$ 的一个合法**划分（Partition）**。
    ///
    /// 划分的三大公理：
    /// 1. 每个块非空：$\forall S \in P, S \neq \emptyset$
    /// 2. 块之间两两不相交：$\forall S_1, S_2 \in P, S_1 \neq S_2 \implies S_1 \cap S_2 = \emptyset$
    /// 3. 所有块的并集等于原集合：$\bigcup_{S \in P} S = A$
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete_math::MathSet;
    ///
    /// let a = MathSet::from_slice(&[1, 2, 3]);
    /// let valid = MathSet::from_slice(&[
    ///     MathSet::from_slice(&[1, 2]),
    ///     MathSet::from_slice(&[3]),
    /// ]);
    /// assert!(a.is_valid_partition(&valid));
    ///
    /// // 块之间存在交集，不是合法划分
    /// let invalid = MathSet::from_slice(&[
    ///     MathSet::from_slice(&[1, 2]),
    ///     MathSet::from_slice(&[2, 3]),
    /// ]);
    /// assert!(!a.is_valid_partition(&invalid));
    /// ```
    pub fn is_valid_partition(&self, partition: &MathSet<MathSet<T>>) -> bool {
        let mut union_all = MathSet::new();
        let blocks: Vec<&MathSet<T>> = partition.iter().collect();

        for (i, block) in blocks.iter().enumerate() {
            if block.is_empty() {
                return false;
            }
            for other_block in blocks.iter().skip(i + 1) {
                if !block.is_disjoint(other_block) {
                    return false;
                }
            }
            union_all = union_all.union(block);
        }

        &union_all == self
    }
}

impl<T: Ord + Clone> FromIterator<T> for MathSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            elements: iter.into_iter().collect(),
        }
    }
}

impl<T: Ord + Clone> IntoIterator for MathSet<T> {
    type Item = T;
    type IntoIter = std::collections::btree_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T: Ord + Clone> IntoIterator for &'a MathSet<T> {
    type Item = &'a T;
    type IntoIter = std::collections::btree_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<T: Ord + Clone + fmt::Debug> fmt::Debug for MathSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, elem) in self.elements.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", elem)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::MathSet;

    #[test]
    fn test_set_basic_operations_and_de_morgan() {
        let u = MathSet::from_slice(&[1, 2, 3, 4, 5, 6]);
        let a = MathSet::from_slice(&[1, 2, 3, 3]); // 自动去重 -> {1, 2, 3}
        let b = MathSet::from_slice(&[3, 4, 5]);

        assert_eq!(a.cardinality(), 3);
        assert!(a.contains(&2));
        assert!(!a.contains(&4));

        // 子集与真子集
        let sub = MathSet::from_slice(&[1, 2]);
        assert!(sub.is_subset(&a));
        assert!(sub.is_proper_subset(&a));
        assert!(a.is_subset(&a));
        assert!(!a.is_proper_subset(&a));

        // 并、交、差、对称差
        assert_eq!(a.union(&b), MathSet::from_slice(&[1, 2, 3, 4, 5]));
        assert_eq!(a.intersection(&b), MathSet::from_slice(&[3]));
        assert_eq!(a.difference(&b), MathSet::from_slice(&[1, 2]));
        assert_eq!(
            a.symmetric_difference(&b),
            MathSet::from_slice(&[1, 2, 4, 5])
        );

        // 验证德摩根定律 (De Morgan's Laws):
        // (A ∪ B)^c == A^c ∩ B^c
        let lhs1 = a.union(&b).complement(&u);
        let rhs1 = a.complement(&u).intersection(&b.complement(&u));
        assert_eq!(lhs1, rhs1);

        // (A ∩ B)^c == A^c ∪ B^c
        let lhs2 = a.intersection(&b).complement(&u);
        let rhs2 = a.complement(&u).union(&b.complement(&u));
        assert_eq!(lhs2, rhs2);
    }

    #[test]
    fn test_power_set_and_cartesian_product() {
        let a = MathSet::from_slice(&[1, 2, 3]);
        let p_a = a.power_set();

        // |P(A)| = 2^|A| = 2^3 = 8
        assert_eq!(p_a.cardinality(), 8);
        assert!(p_a.contains(&MathSet::empty()));
        assert!(p_a.contains(&a));
        assert!(p_a.contains(&MathSet::from_slice(&[1, 3])));

        // 笛卡尔积 |A × B| = |A| * |B|
        let b = MathSet::from_slice(&['x', 'y']);
        let prod = a.cartesian_product(&b);
        assert_eq!(prod.cardinality(), 3 * 2);
        assert!(prod.contains(&(2, 'y')));
    }

    #[test]
    fn test_set_partition_validation() {
        let a = MathSet::from_slice(&[1, 2, 3, 4]);

        // 合法划分：{{1, 2}, {3}, {4}}
        let valid_part = MathSet::from_slice(&[
            MathSet::from_slice(&[1, 2]),
            MathSet::from_slice(&[3]),
            MathSet::from_slice(&[4]),
        ]);
        assert!(a.is_valid_partition(&valid_part));

        // 非法划分（有交集）：{{1, 2}, {2, 3}, {4}}
        let overlapping_part = MathSet::from_slice(&[
            MathSet::from_slice(&[1, 2]),
            MathSet::from_slice(&[2, 3]),
            MathSet::from_slice(&[4]),
        ]);
        assert!(!a.is_valid_partition(&overlapping_part));

        // 非法划分（包含空集）：{{1, 2, 3, 4}, ∅}
        let empty_block_part =
            MathSet::from_slice(&[MathSet::from_slice(&[1, 2, 3, 4]), MathSet::empty()]);
        assert!(!a.is_valid_partition(&empty_block_part));
    }

    #[test]
    fn test_set_iterator_round_trip() {
        let a = MathSet::from_slice(&[3, 1, 2]);
        // 迭代顺序为升序，且可经 FromIterator 原样重建
        assert_eq!(a.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(a.iter().cloned().collect::<MathSet<_>>(), a);
    }
}
