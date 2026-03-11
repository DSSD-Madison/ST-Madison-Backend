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

readonly output_dir="${OUTPUT_DIR:-${repo_root}/build/tiles}"
readonly pmtiles_name="${PMTILES_NAME:-madison-parcels.pmtiles}"
readonly min_zoom="${MIN_ZOOM:-10}"
readonly max_zoom="${MAX_ZOOM:-14}"

require_command duckdb
require_command tippecanoe
require_env GCS_KEY_ID
require_env GCS_SECRET

mkdir -p "${output_dir}"
rm -f "${output_dir}/parcels.geojson.ndjson" "${output_dir}/${pmtiles_name}"

pushd "${output_dir}" >/dev/null
duckdb -init /dev/null < "${script_dir}/extract_parcels_geojson.sql"

tippecanoe \
    --force \
    --projection=EPSG:4326 \
    --read-parallel \
    --layer=parcels \
    --name="Madison Parcels" \
    --description="Madison, WI parcel polygons from silver.parcels" \
    --minimum-zoom="${min_zoom}" \
    --maximum-zoom="${max_zoom}" \
    --maximum-tile-bytes=4000000 \
    --drop-densest-as-needed \
    --extend-zooms-if-still-dropping \
    --simplification=6 \
    --simplify-only-low-zooms \
    --no-simplification-of-shared-nodes \
    --no-tiny-polygon-reduction-at-maximum-zoom \
    --single-precision \
    --output="${pmtiles_name}" \
    parcels.geojson.ndjson
popd >/dev/null

du -h "${output_dir}/${pmtiles_name}"