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
    fn empty_payload_is_absent_not_fabricated() {
        assert_eq!(parse_collection_ids(b"{}"), None);
        assert_eq!(parse_items(b"{}"), None);
    }
}
