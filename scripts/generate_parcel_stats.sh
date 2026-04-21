#!/usr/bin/env bash

set -euo pipefail

require_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "Missing required command: $1" >&2
        exit 1
    fi
}

require_env() {
    if [[ -z "${!1:-}" ]]; then
        echo "Missing required environment variable: $1" >&2
        exit 1
    fi
}

readonly script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly repo_root="$(cd -- "${script_dir}/.." && pwd)"

readonly output_dir="${OUTPUT_DIR:-${repo_root}/build/stats}"
readonly stats_name="${STATS_NAME:-parcels.stats.json}"

require_command duckdb
require_env GCS_KEY_ID
require_env GCS_SECRET

mkdir -p "${output_dir}"
rm -f "${output_dir}/${stats_name}"

pushd "${output_dir}" >/dev/null
duckdb -init /dev/null < "${script_dir}/extract_parcels_stats.sql"
popd >/dev/null

du -h "${output_dir}/${stats_name}"
