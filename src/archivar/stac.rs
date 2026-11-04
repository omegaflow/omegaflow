use crate::archivar::extract::flatten_geojson_coords;
use crate::archivar::hapi_csv::parse_iso_seconds;
use crate::archivar::json::{JsonVal, parse_json};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct StacAsset {
    pub key: String,
    pub href: String,
    pub media_type: Option<String>,
    pub title: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StacItem {
    pub id: String,
    pub collection: Option<String>,
    pub start_epoch: Option<f64>,
    pub end_epoch: Option<f64>,
    pub datetime_epoch: Option<f64>,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub assets: Vec<StacAsset>,
}

fn obj(v: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match v {
        JsonVal::Obj(m) => Some(m),
        _ => None,
    }
}

fn arr(v: &JsonVal) -> Option<&Vec<JsonVal>> {
    match v {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn str_of(v: Option<&JsonVal>) -> Option<String> {
    match v {
        Some(JsonVal::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

fn representative_point(geometry: &JsonVal) -> Option<(f64, f64)> {
    let g = obj(geometry)?;
    let geom_type = str_of(g.get("type"))?;
    let coords = arr(g.get("coordinates")?)?;
    let first = coords.first()?;
    let ring: &Vec<JsonVal> = match geom_type.as_str() {
        "Polygon" => arr(first)?,
        "MultiPolygon" => arr(arr(first)?.first()?)?,
        _ => return None,
    };
    let vertices = flatten_geojson_coords(ring);
    if vertices.is_empty() {
        return None;
    }
    let n = vertices.len() as f64;
    let lon = vertices.iter().map(|v| v.0).sum::<f64>() / n;
    let lat = vertices.iter().map(|v| v.1).sum::<f64>() / n;
    if lon.is_finite() && lat.is_finite() {
        Some((lon, lat))
    } else {
        None
    }
}

pub fn parse_collection_ids(bytes: &[u8]) -> Option<Vec<String>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let json = parse_json(text)?;
    let collections = arr(obj(&json)?.get("collections")?)?;
    let mut out = Vec::new();
    for c in collections {
        if let Some(id) = obj(c).and_then(|m| str_of(m.get("id"))) {
            out.push(id);
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn parse_items(bytes: &[u8]) -> Option<Vec<StacItem>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let json = parse_json(text)?;
    let features: Vec<JsonVal> = match obj(&json).and_then(|m| m.get("features")) {
        Some(v) => arr(v)?.clone(),
        None => vec![json],
    };
    let mut out = Vec::new();
    for f in &features {
        let Some(m) = obj(f) else { continue };
        let Some(id) = str_of(m.get("id")) else {
            continue;
        };
        let collection = str_of(m.get("collection"));
        let (start_epoch, end_epoch, datetime_epoch) = match m.get("properties").and_then(obj) {
            Some(p) => (
                str_of(p.get("start_datetime")).and_then(|s| parse_iso_seconds(&s)),
                str_of(p.get("end_datetime")).and_then(|s| parse_iso_seconds(&s)),
                str_of(p.get("datetime")).and_then(|s| parse_iso_seconds(&s)),
            ),
            None => (None, None, None),
        };
        let (lon, lat) = match m.get("geometry").and_then(representative_point) {
            Some((lon, lat)) => (Some(lon), Some(lat)),
            None => (None, None),
        };
        let mut assets = Vec::new();
        if let Some(am) = m.get("assets").and_then(obj) {
            for (key, a) in am {
                let Some(av) = obj(a) else { continue };
                let Some(href) = str_of(av.get("href")) else {
                    continue;
                };
                let roles = match av.get("roles").and_then(arr) {
                    Some(rs) => rs.iter().filter_map(|r| str_of(Some(r))).collect(),
                    None => Vec::new(),
                };
                assets.push(StacAsset {
                    key: key.clone(),
                    href,
                    media_type: str_of(av.get("type")),
                    title: str_of(av.get("title")),
                    roles,
                });
            }
        }
        out.push(StacItem {
            id,
            collection,
            start_epoch,
            end_epoch,
            datetime_epoch,
            lon,
            lat,
            assets,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn select_asset<'a>(item: &'a StacItem, prefer_media: &[&str]) -> Option<&'a StacAsset> {
    for p in prefer_media {
        if let Some(a) = item
            .assets
            .iter()
            .find(|a| a.media_type.as_deref().is_some_and(|m| m.contains(p)))
        {
            return Some(a);
        }
    }
    item.assets
        .iter()
        .find(|a| !a.roles.iter().any(|r| r == "thumbnail" || r == "metadata"))
        .or_else(|| item.assets.first())
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLLECTIONS: &str = r#"{"collections":[{"id":"CCM_A"},{"id":"CCM_B"}],"links":[]}"#;

    const ITEMS: &str = r#"{"type":"FeatureCollection","features":[
      {"id":"item-1","collection":"CCM_A","assets":{
        "thumbnail":{"href":"https://example.org/t.png","type":"image/png","roles":["thumbnail"]},
        "data":{"href":"https://example.org/d.tif","type":"image/tiff; application=geotiff","roles":["data"]}
      }},
      {"id":"item-2","collection":"CCM_A","assets":{
        "data":{"href":"https://example.org/e.nc","type":"application/x-netcdf","roles":["data"]}
      }}
    ]}"#;

    const ITEMS_GEOM: &str = r#"{"type":"FeatureCollection","features":[
      {"id":"ccm-1","collection":"CCM_A",
       "properties":{
         "start_datetime":"2023-06-15T12:30:00Z",
         "end_datetime":"2023-06-15T12:40:00Z",
         "datetime":"2023-06-16T00:00:00Z"
       },
       "geometry":{"type":"MultiPolygon","coordinates":[[[[10.0,20.0],[12.0,20.0],[12.0,22.0],[10.0,22.0],[10.0,20.0]]]]},
       "assets":{"data":{"href":"https://example.org/d.tif","type":"image/tiff","roles":["data"]}}}
    ]}"#;

    #[test]
    fn collection_ids_parse() {
        let ids = parse_collection_ids(COLLECTIONS.as_bytes()).expect("collections");
        assert_eq!(ids, vec!["CCM_A".to_string(), "CCM_B".to_string()]);
    }

    #[test]
    fn items_carry_assets_and_select_prefers_media() {
        let items = parse_items(ITEMS.as_bytes()).expect("items");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, "item-1");
        assert_eq!(items[0].collection.as_deref(), Some("CCM_A"));
        assert_eq!(items[0].assets.len(), 2);
        let pick = select_asset(&items[0], &["geotiff"]).expect("asset");
        assert_eq!(pick.key, "data");
        assert!(pick.href.ends_with("d.tif"));
        let none = select_asset(&items[1], &["geotiff"]).expect("fallback");
        assert_eq!(none.key, "data");
    }

    #[test]
    fn item_carries_epoch_and_representative_point() {
        let items = parse_items(ITEMS_GEOM.as_bytes()).expect("items");
        assert_eq!(items.len(), 1);
        let it = &items[0];
        assert_eq!(it.start_epoch, Some(1_686_832_200.0));
        assert_eq!(it.end_epoch, Some(1_686_832_800.0));
        assert_eq!(it.datetime_epoch, Some(1_686_873_600.0));
        let lon = it.lon.expect("lon");
        let lat = it.lat.expect("lat");
        assert!((lon - 10.8).abs() < 1e-12, "lon {lon}");
        assert!((lat - 20.8).abs() < 1e-12, "lat {lat}");
    }

    #[test]
    fn absent_properties_are_none_not_fabricated_zero() {
        let items = parse_items(ITEMS.as_bytes()).expect("items");
        assert_eq!(items[0].start_epoch, None);
        assert_eq!(items[0].end_epoch, None);
        assert_eq!(items[0].datetime_epoch, None);
        assert_eq!(items[0].lon, None);
        assert_eq!(items[0].lat, None);
    }

    #[test]
    fn empty_payload_is_absent_not_fabricated() {
        assert_eq!(parse_collection_ids(b"{}"), None);
        assert_eq!(parse_items(b"{}"), None);
    }
}
