#!/usr/bin/env bash
# Switch the rust-analyzer build target in Zed between native and wasm.
#
#   ./ra-target.sh          toggle between native <-> wasm
#   ./ra-target.sh wasm     force wasm  (wasm32-unknown-unknown)
#   ./ra-target.sh native   force native (host default)
#   ./ra-target.sh status   print current target, change nothing
#
# Zed hot-reloads .zed/settings.json, so rust-analyzer restarts with the
# new target on its own. Assumes .zed/settings.json is plain JSON (no comments).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SETTINGS="$ROOT/.zed/settings.json"
WASM_TARGET="wasm32-unknown-unknown"

python3 - "$SETTINGS" "$WASM_TARGET" "${1:-toggle}" <<'PY'
import json, os, sys

settings_path, wasm_target, mode = sys.argv[1], sys.argv[2], sys.argv[3]

# Load existing settings (or start fresh).
data = {}
if os.path.exists(settings_path):
    with open(settings_path) as f:
        text = f.read().strip()
    if text:
        data = json.loads(text)

def current_target():
    return (data.get("lsp", {})
                .get("rust-analyzer", {})
                .get("initialization_options", {})
                .get("cargo", {})
                .get("target"))

cur = current_target()

if mode == "status":
    print(f"native (host)" if cur is None else cur)
    sys.exit(0)

if mode == "toggle":
    mode = "native" if cur == wasm_target else "wasm"
elif mode not in ("wasm", "native"):
    sys.exit(f"unknown mode: {mode!r} (use wasm | native | status | toggle)")

# Navigate/create the nested path.
cargo = (data.setdefault("lsp", {})
             .setdefault("rust-analyzer", {})
             .setdefault("initialization_options", {})
             .setdefault("cargo", {}))

if mode == "wasm":
    cargo["target"] = wasm_target
else:  # native: drop the key so rust-analyzer uses the host triple
    cargo.pop("target", None)

os.makedirs(os.path.dirname(settings_path), exist_ok=True)
with open(settings_path, "w") as f:
    json.dump(data, f, indent=2)
    f.write("\n")

print(f"native (host)" if mode == "native" else wasm_target)
PY
