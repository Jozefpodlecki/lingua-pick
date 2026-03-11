use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
pub struct FeatureNode {
    pub feature: LanguageFeature,
    pub children: Vec<FeatureNode>,
}

pub fn collect_with_ancestors(
    feature: &LanguageFeature,
    all_features: &HashMap<u32, LanguageFeature>,
    collected: &mut HashMap<u32, LanguageFeature>,
) {
    if collected.contains_key(&feature.id) {
        return;
    }
    collected.insert(feature.id, feature.clone());
    if let Some(pid) = feature.parent_id {
        if let Some(parent) = all_features.get(&pid) {
            collect_with_ancestors(parent, all_features, collected);
        }
    }
}

pub fn build_feature_tree(
    features: &[LanguageFeature],
) -> Vec<FeatureNode> {
    let mut feature_map: HashMap<u32, &LanguageFeature> = HashMap::new();
    for f in features {
        feature_map.insert(f.id, f);
    }

    let mut parent_map: HashMap<Option<u32>, Vec<&LanguageFeature>> = HashMap::new();
    for f in features {
        parent_map.entry(f.parent_id).or_default().push(f);
    }


    fn build_node(f: &LanguageFeature, parent_map: &HashMap<Option<u32>, Vec<&LanguageFeature>>) -> FeatureNode {
        let children = parent_map
            .get(&Some(f.id))
            .unwrap_or(&vec![])
            .iter()
            .map(|child| build_node(child, parent_map))
            .collect();

        FeatureNode {
            feature: f.clone(),
            children,
        }
    }

    parent_map.get(&None)
        .unwrap_or(&vec![])
        .iter()
        .map(|root| build_node(root, &parent_map))
        .collect()
}