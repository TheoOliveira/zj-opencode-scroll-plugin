#!/usr/bin/env bash
# build.sh — compila o plugin para wasm32-wasip1 e copia para ~/.config/zellij/plugins/

set -euo pipefail

PLUGIN_NAME="opencode-scroll-plugin"
TARGET="wasm32-wasip1"
ZELLIJ_PLUGINS_DIR="${HOME}/.config/zellij/plugins"

echo "🔧 Compilando ${PLUGIN_NAME}..."
cargo build --release --target ${TARGET}

WASM_SRC="target/${TARGET}/release/${PLUGIN_NAME}.wasm"

echo "📦 Copiando para ${ZELLIJ_PLUGINS_DIR}/"
mkdir -p "${ZELLIJ_PLUGINS_DIR}"
cp "${WASM_SRC}" "${ZELLIJ_PLUGINS_DIR}/${PLUGIN_NAME}.wasm"

echo "✅ Pronto! Plugin em: ${ZELLIJ_PLUGINS_DIR}/${PLUGIN_NAME}.wasm"
echo ""
echo "Para carregar manualmente:"
echo "  zellij run --plugin file:${ZELLIJ_PLUGINS_DIR}/${PLUGIN_NAME}.wasm"
echo ""
echo "Para usar via layout, veja: layout/opencode.kdl"
