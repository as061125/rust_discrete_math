# 更新日志

本项目遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 的组织方式，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [0.2.0] - 2026-10-22

首个公开发布版本，包含集合、二元关系、函数三个模块的完整实现。

### 新增

- **集合论**：`MathSet<T>` 支持并、交、差、对称差、补集、子集判定、幂集、笛卡尔积与划分验证。
- **二元关系**：`BinaryRelation<A, B>` 支持定义域/值域/像集、逆关系、复合；同质关系支持五大性质判定、
  三大闭包、等价关系与商集、偏序/全序关系及极小/极大/最小/最大元。
- **函数**：`DiscreteFunction<A, B>` 严格按左全性与单值性构造，支持求值、像与原像、单射/满射/双射判定、
  函数复合与逆函数。
- `DiscreteError` 统一错误类型，实现 `Display` 与 `std::error::Error`。
- `src/main.rs` 演示程序，`cargo run` 可查看各模块实际输出。

### 变更

- 将原本的单文件 `src/lib.rs` 拆分为 `error`、`set`、`relation`、`function` 四个模块，
  `lib.rs` 只负责模块声明与根路径重导出，原有 `discrete_math::set::MathSet` 等路径保持兼容。
- 单元测试随模块就近存放；测试数量由 8 个增至 12 个，并新增 7 个文档测试。

### 工程

- 采用 `MIT OR Apache-2.0` 双许可。
- 补充 crates.io 元数据：`license`、`repository`、`documentation`、`rust-version`。
