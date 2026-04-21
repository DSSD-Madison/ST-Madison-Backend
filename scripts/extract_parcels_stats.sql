INSTALL httpfs;
LOAD httpfs;

INSTALL json;
LOAD json;

CREATE OR REPLACE SECRET gcs_credentials (
    TYPE gcs,
    KEY_ID getenv('GCS_KEY_ID'),
    SECRET getenv('GCS_SECRET')
);

COPY (
    SELECT
        site_parcel_id,
        parcel_year,
        property_class,
        property_use,
        zoning_all,
        area_plan_name,
        alder_district_name,
        ward,
        total_living_area,
        lot_size,
        current_land_value,
        current_improvement_value,
        current_total_value,
        net_taxes,
        tax_rate,
        net_taxes_per_sqft_lot,
        land_value_per_sqft_lot,
        land_share_property,
        land_value_alignment_index
    FROM read_parquet('gs://stmsn-silver/fact_parcels.parquet')
    WHERE parcel_year = (
        SELECT max(parcel_year) FROM read_parquet('gs://stmsn-silver/fact_parcels.parquet')
    )
) TO 'parcels.stats.json' (FORMAT JSON, ARRAY true);
