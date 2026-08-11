# Security Policy

## Supported Versions

Security fixes are applied to the current `main` branch and supported release tags.

| Version | Supported |
| --- | --- |
| `main` | Yes |
| `v0.1.x` | Yes |

## Reporting

Use [GitHub private vulnerability reporting](https://github.com/Tinkora/diff_viz/security/advisories/new)
for security issues. Do not disclose a vulnerability or secret in a public issue.

Include the affected revision, impact, reproduction steps, and suggested
mitigation. Do not include real credentials or private user data.

## Boundary

The tool is local-only and has no authentication, persistence, account system,
or server transport. Input is bounded to 100 KiB per side and is processed in
WASM in the browser. Changes to this boundary require new threat-model and
regression tests.
