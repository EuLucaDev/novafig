#!/usr/bin/env bash
# App-specific checks retained alongside shared packaging lint.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

bash packaging/server/install-test.sh
if grep -rniE 'photoshop|psd|novaphoto|novavector|illustrator|adobe|figma' packaging/ .github/ \
    --exclude=packaging-lint.yml --exclude=check-app-specific.sh; then
  echo "::error::package metadata mentions another product (see above)"
  exit 1
fi

python3 - <<'PYEOF'
import yaml
manifests = []
for name in ("br.com.novafoods.novafig.yml", "br.com.novafoods.novafig.bundle.yml"):
    with open("packaging/linux/flatpak/" + name) as source:
        manifest = yaml.safe_load(source)
    assert manifest["id"] == "br.com.novafoods.novafig" and manifest["command"] == "novafig", name
    manifests.append(manifest)
for key in ("runtime", "runtime-version", "sdk", "finish-args"):
    assert manifests[0][key] == manifests[1][key], f"Flatpak manifests disagree on {key}"
print("NovaFig package identity ok")
PYEOF
