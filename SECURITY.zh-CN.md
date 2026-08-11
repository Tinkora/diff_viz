# 安全策略

## 支持版本

安全修复应用于当前 `main` 分支和仍受支持的发布 tag。

| 版本 | 支持状态 |
| --- | --- |
| `main` | 支持 |
| `v0.1.x` | 支持 |

## 报告方式

安全问题请使用 [GitHub 私密漏洞报告](https://github.com/Tinkora/diff_viz/security/advisories/new)。
不要在公开 issue 中披露漏洞或 secret。

报告应包含受影响 revision、影响、复现步骤和可选的缓解建议。不要包含真实凭据或私有用户数据。

## 边界

工具仅供本地使用，没有身份验证、持久化、账户系统或服务器 transport。每侧输入上限为 100 KiB，
由浏览器中的 WASM 处理。改变这一边界前必须补充威胁模型和回归测试。
