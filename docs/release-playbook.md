# Release playbook

The release playbook shared by every crafting app lives in craftrules:
[`../craftrules/release/playbook.md`](../../../craftrules/release/playbook.md)
(`EuLucaDev/craftrules`, checked out next to `novafoods-apps/`; the repository is not public). It is
the canonical copy; don't duplicate it here. Cutting a release step by step:
`craftrules/release/cut-a-release.md`.

Nova Fig copies Nova Photo's reference implementation (`.github/workflows/release.yml`,
`packaging/`, the `xtask version` command) with Nova Fig's names. Nova Fig-specific details are in
[releasing.md](releasing.md).
