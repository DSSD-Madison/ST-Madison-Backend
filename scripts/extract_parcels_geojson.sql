INSTALL httpfs;
LOAD httpfs;

INSTALL json;
LOAD json;

INSTALL spatial;
LOAD spatial;

CREATE OR REPLACE SECRET gcs_credentials (
    TYPE gcs,
    KEY_ID getenv('GCS_KEY_ID'),
    SECRET getenv('GCS_SECRET')
);

COPY (
    WITH source_parcels AS (
        SELECT
            site_parcel_id,
            parcel_id,
            parcel_year,
            parcel_address,
            full_address,
            property_class,
            property_use,
            zoning_all,
            area_name,
            alder_district_name,
            area_plan_name,
            ward,
            bedrooms,
            full_baths,
            half_baths,
            total_living_area,
            lot_size,
            total_dwelling_units,
            current_land_value,
            current_improvement_value,
            current_total_value,
            net_taxes,
            total_taxes,
            tax_rate,
            net_taxes_per_sqft_lot,
            total_taxes_per_sqft_lot,
            land_value_per_sqft_lot,
            total_net_taxes_city,
            current_total_land_value_city,
            current_total_value_city,
            land_share_property,
            land_share_city,
            total_share_city,
            land_total_ratio_city,
            land_value_alignment_index,
            land_value_shift_taxes,
            ST_CollectionExtract(
                ST_Force2D(
                    CASE
                        WHEN ST_IsValid(geom_4326) THEN geom_4326
                        ELSE ST_MakeValid(geom_4326)
                    END
                ),
                3
            ) AS parcel_geometry
        FROM read_parquet('gs://stmsn-silver/fact_parcels.parquet')
        WHERE geom_4326 IS NOT NULL
          AND NOT ST_IsEmpty(geom_4326)
    )
    SELECT
        'Feature' AS type,
        ST_AsGeoJSON(ST_FlipCoordinates(parcel_geometry)) AS geometry,
        json_object(
            'site_parcel_id', site_parcel_id,
            'parcel_id', parcel_id,
            'parcel_year', parcel_year,
            'parcel_address', parcel_address,
            'full_address', full_address,
            'property_class', property_class,
            'property_use', property_use,
            'zoning_all', zoning_all,
            'area_name', area_name,
            'alder_district_name', alder_district_name,
            'area_plan_name', area_plan_name,
            'ward', ward,
            'bedrooms', bedrooms,
            'full_baths', full_baths,
            'half_baths', half_baths,
            'total_living_area', total_living_area,
            'lot_size', lot_size,
            'total_dwelling_units', total_dwelling_units,
            'current_land_value', current_land_value,
            'current_improvement_value', current_improvement_value,
            'current_total_value', current_total_value,
            'net_taxes', net_taxes,
            'total_taxes', total_taxes,
            'tax_rate', tax_rate,
            'net_taxes_per_sqft_lot', net_taxes_per_sqft_lot,
            'total_taxes_per_sqft_lot', total_taxes_per_sqft_lot,
            'land_value_per_sqft_lot', land_value_per_sqft_lot,
            'total_net_taxes_city', total_net_taxes_city,
            'current_total_land_value_city', current_total_land_value_city,
            'current_total_value_city', current_total_value_city,
            'land_share_property', land_share_property,
            'land_share_city', land_share_city,
            'total_share_city', total_share_city,
            'land_total_ratio_city', land_total_ratio_city,
            'land_value_alignment_index', land_value_alignment_index,
            'land_value_shift_taxes', land_value_shift_taxes
        ) AS properties
    FROM source_parcels
    WHERE parcel_geometry IS NOT NULL
      AND NOT ST_IsEmpty(parcel_geometry)
) TO 'parcels.geojson.ndjson' (FORMAT json, ARRAY false);