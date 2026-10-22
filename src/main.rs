//! `discrete_math` 命令行演示程序：一次运行即可看完全部三大模块的典型用法。
//!
//! 运行方式：`cargo run`

use discrete_math::{BinaryRelation, DiscreteFunction, MathSet};

fn main() {
    header("一、集合代数 MathSet<T>");
    demo_set_algebra();

    header("二、等价关系与商集 BinaryRelation<A, B>");
    demo_equivalence_relation();

    header("三、偏序关系与极值元");
    demo_partial_order();

    header("四、函数、双射与逆函数 DiscreteFunction<A, B>");
    demo_functions();
}

fn header(title: &str) {
    println!("\n=== {title} ===");
}

fn demo_set_algebra() {
    let universe = MathSet::from_slice(&[1, 2, 3, 4, 5, 6]);
    let a = MathSet::from_slice(&[1, 2, 3, 3]); // 自动去重
    let b = MathSet::from_slice(&[3, 4, 5]);

    println!("全集 U = {universe:?}");
    println!("A = {a:?}, |A| = {}", a.cardinality());
    println!("B = {b:?}, |B| = {}", b.cardinality());
    println!("A ∪ B = {:?}", a.union(&b));
    println!("A ∩ B = {:?}", a.intersection(&b));
    println!("A \\ B = {:?}", a.difference(&b));
    println!("A △ B = {:?}", a.symmetric_difference(&b));
    println!("A 关于 U 的补集 = {:?}", a.complement(&universe));
    println!(
        "|A × B| = {} × {} = {}",
        a.cardinality(),
        b.cardinality(),
        a.cartesian_product(&b).cardinality()
    );
    println!(
        "|P(A)| = 2^{} = {}",
        a.cardinality(),
        a.power_set().cardinality()
    );

    // 划分验证：{{1, 2}, {3}} 是 A 的合法划分
    let partition = MathSet::from_slice(&[MathSet::from_slice(&[1, 2]), MathSet::from_slice(&[3])]);
    println!(
        "{{{{1, 2}}, {{3}}}} 是 A 的合法划分: {}",
        a.is_valid_partition(&partition)
    );
}

fn demo_equivalence_relation() {
    let a = MathSet::from_slice(&[0, 1, 2, 3, 4, 5]);
    // 模 3 同余关系：x ≡ y (mod 3)
    let mod3 = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| x % 3 == y % 3);

    println!(
        "A = {a:?}, R 为模 3 同余关系, |R| = {}",
        mod3.pairs().cardinality()
    );
    println!(
        "自反: {}, 对称: {}, 传递: {}",
        mod3.is_reflexive(),
        mod3.is_symmetric(),
        mod3.is_transitive()
    );
    println!("是否为等价关系: {}", mod3.is_equivalence());
    println!(
        "[0]_R = {:?}, [1]_R = {:?}",
        mod3.equivalence_class(&0).unwrap(),
        mod3.equivalence_class(&1).unwrap()
    );

    let quotient = mod3.quotient_set().unwrap();
    println!("商集 A/R = {quotient:?}");
    println!("商集构成 A 的合法划分: {}", a.is_valid_partition(&quotient));
}

fn demo_partial_order() {
    let a = MathSet::from_slice(&[1, 2, 3, 4, 6, 12]);
    // 整除关系：x | y
    let divides = BinaryRelation::from_predicate(a.clone(), a.clone(), |x, y| y % x == 0);

    println!("A = {a:?}, R 为整除关系 (x | y)");
    println!("是否为偏序关系: {}", divides.is_partial_order());
    println!(
        "是否为全序关系: {}（2 与 3 不可比）",
        divides.is_total_order()
    );
    println!("极小元: {:?}", divides.minimal_elements().unwrap());
    println!("极大元: {:?}", divides.maximal_elements().unwrap());
    println!("最小元: {:?}", divides.least_element().unwrap());
    println!("最大元: {:?}", divides.greatest_element().unwrap());

    // 去掉 1 与 12 后，不再存在最小元与最大元
    let b = MathSet::from_slice(&[2, 3, 4, 6]);
    let divides_b = BinaryRelation::from_predicate(b.clone(), b.clone(), |x, y| y % x == 0);
    println!(
        "在 {b:?} 上: 极小元 = {:?}",
        divides_b.minimal_elements().unwrap()
    );
    println!(
        "在 {b:?} 上: 最小元 = {:?}",
        divides_b.least_element().unwrap()
    );
}

fn demo_functions() {
    let a = MathSet::from_slice(&[1, 2, 3]);
    let b = MathSet::from_slice(&[2, 4, 6, 8]);

    // f(x) = 2x：单射但非满射
    let f = DiscreteFunction::from_fn(a.clone(), b.clone(), |x| x * 2).unwrap();
    println!("f: {a:?} -> {b:?}, f(x) = 2x");
    println!(
        "  单射: {}, 满射: {}, 双射: {}",
        f.is_injective(),
        f.is_surjective(),
        f.is_bijective()
    );
    println!("  求逆函数失败: {}", f.inverse().unwrap_err());

    // g(x) = 10x：双射，可求逆函数
    let c = MathSet::from_slice(&[10, 20, 30]);
    let g = DiscreteFunction::from_fn(a.clone(), c.clone(), |x| x * 10).unwrap();
    println!("g: {a:?} -> {c:?}, g(x) = 10x");
    println!("  双射: {}, g(2) = {:?}", g.is_bijective(), g.eval(&2));

    let g_inv = g.inverse().unwrap();
    println!("  g⁻¹(20) = {:?}", g_inv.eval(&20));
    println!(
        "  g⁻¹ ∘ g = id_A: {}",
        g.compose(&g_inv).unwrap() == DiscreteFunction::identity(a.clone())
    );
    println!(
        "  g ∘ g⁻¹ = id_C: {}",
        g_inv.compose(&g).unwrap() == DiscreteFunction::identity(c)
    );

    // h(x) = x mod 2：满射但非单射，可求原像
    let d = MathSet::from_slice(&[1, 2, 3, 4]);
    let e = MathSet::from_slice(&[0, 1]);
    let h = DiscreteFunction::from_fn(d.clone(), e, |x| x % 2).unwrap();
    println!("h: {d:?} -> {{0, 1}}, h(x) = x mod 2");
    println!("  单射: {}, 满射: {}", h.is_injective(), h.is_surjective());
    println!(
        "  原像 h⁻¹({{0}}) = {:?}",
        h.preimage_of(&MathSet::from_slice(&[0]))
    );
}
