# 发布前检查清单

[English](RELEASE_CHECKLIST.md)

本清单只用于验证发布候选，不代表获准创建 tag、GitHub Release、部署、发布包或发布凭据。

## 发布权限

- [ ] 第二位可信 owner 已审查发布候选和恢复方案。
- [ ] 发布凭据仅由启用必需 reviewer 和最小权限的 protected environment 持有。
- [ ] 仓库规则保护发布 commit，并要求相关检查通过。

自动 release workflow 会在受保护的 `release` environment 中验证候选，并在所有必需检查通过后发布。

## 候选身份

- [ ] 选择 SemVer 版本并确认兼容性影响。
- [ ] 记录要发布的准确且干净的 commit。
- [ ] 确认拟定的不可变 tag 为 `v<version>`，且指向该 commit。
- [ ] 在 `CHANGELOG.md` 中记录发布日期、用户可见变更、迁移说明和安全影响。

## 质量门禁

- [ ] `cargo fmt --all -- --check` 通过。
- [ ] 原生 workspace 的 `cargo test --workspace` 通过。
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` 通过。
- [ ] `cargo check -p diff_viz_web --target wasm32-unknown-unknown --locked` 通过。
- [ ] 真实 WASM 产物构建成功，Playwright 项目在 375、768、1024 与 1440 px 下通过。
- [ ] Markdown lint、`scripts/test_check_docs.rb`、`scripts/check_docs.rb`、`scripts/test_check_workflow_contracts.rb` 和 `scripts/check_workflow_contracts.rb` 通过。
- [ ] `cargo deny check advisories bans licenses sources` 和 `cargo audit` 针对候选 lockfile 通过。
- [ ] 必需的托管 CI 检查在准确候选 commit 上通过；本地运行不能替代受保护的 CI 证据。

## 契约与风险审查

- [ ] 确认每项依赖、随附资源和发布产物均具有可接受的许可证，并包含必要声明。
- [ ] 重新核对文档所述的隐私行为、网络访问、敏感输入处理和资源限制。
- [ ] 将 `schemaVersion`、序列化字段、错误 code 和公开 Rust API 与上个版本比较。
- [ ] 记录任何破坏性 schema 或 API 变更，并按要求提升 SemVer。
- [ ] 确认生成文件可复现，且不含本地路径、凭据、测试输出或开发缓存。

## 产物证据

- [ ] 在 protected environment 中从已审查 commit 构建发布产物。
- [ ] 为每个分发产物记录 SHA-256 checksum。
- [ ] 生成并保留覆盖实际交付依赖图的 SBOM。
- [ ] 附加能够标识源 commit、构建 workflow、toolchain 和产物 digest 的 provenance。
- [ ] 验证干净的消费环境无需未发布的本地状态即可校验 checksum 并使用产物。

## 恢复

- [ ] 发布前写明 rollback 或 fix-forward 方案，包括决策人和用户通知方式。
- [ ] 保留上一份已知可用产物及其 checksum。
- [ ] 明确如何撤回错误产物，同时不移动或静默替换不可变 tag。
- [ ] 获得授权并发布后，对照已审查候选复核已发布 tag、release notes、产物、checksum、SBOM 和 provenance。

证据等级与 badge 规则见[成熟度](MATURITY.zh-CN.md)。
