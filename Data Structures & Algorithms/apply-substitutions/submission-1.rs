impl Solution {
    pub fn apply_substitutions(replacements: Vec<Vec<String>>, text: String) -> String {
        let mut initial = HashMap::<String, String>::new();
        for replacement in &replacements {
            let key = replacement[0].clone();
            let val = replacement[1].clone();
            initial.insert(key, val);
        }

        let mut resolved = HashMap::new();

        initial.keys().for_each(|key| {
            resolve_key(key, &initial, &mut resolved);
        });

        let mut result = String::new();
        for (idx, split) in text.split('%').enumerate() {
            if idx % 2 == 0 {
                result.push_str(split);
            } else {
                result.push_str(resolved.get(split).unwrap());
            }
        }
        result
    }
}

fn resolve_key(
    key: &str,
    map: &HashMap<String, String>,
    resolved: &mut HashMap<String, String>,
) -> String {
    if let Some(value) = resolved.get(key) {
        return value.clone();
    }

    let val = map.get(key).unwrap();
    if !val.contains('%') {
        resolved.insert(key.into(), val.into());
        return val.into();
    }

    let mut result = String::new();
    for (idx, split) in val.split('%').enumerate() {
        if idx % 2 == 0 {
            result.push_str(split);
        } else {
            result.push_str(&resolve_key(split, map, resolved));
        }
    }
    resolved.insert(key.into(), result.clone());
    result
}
