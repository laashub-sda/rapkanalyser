use std::collections::{HashMap, HashSet};
use dex::Dex;
use memmap::Mmap;
use crate::analyzer::dex::dex_package_node::DexPackageNode;
use crate::analyzer::dex::dex_file::DexFile;
use multi_map::MultiMap;
use dex::string::DexString;
use dex::method::Method;

#[derive(Debug, Clone)]
pub struct ProguardUsagesMap {
    classes: HashSet<String>,
    methods_by_class: HashMap<String, String>,
    fields_by_class: HashMap<String, String>,
}

impl ProguardUsagesMap {
    pub fn new() -> ProguardUsagesMap {
        ProguardUsagesMap {
            classes: Default::default(),
            methods_by_class: Default::default(),
            fields_by_class: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProguardMap {}

impl ProguardMap {
    pub fn new() -> ProguardMap {
        ProguardMap {}
    }
}

#[derive(Debug, Clone)]
pub struct PackageTreeCreator {
    proguard_map: ProguardMap,
    usages_map: ProguardUsagesMap
}

impl PackageTreeCreator {
    pub fn new() -> PackageTreeCreator {
        PackageTreeCreator {
            proguard_map: ProguardMap::new(),
            usages_map: ProguardUsagesMap::new()
        }
    }

    pub fn construct_package_tree(&self, dexes: Vec<Dex<Mmap>>) {
        let mut root = DexPackageNode::new(String::from("root"), None);
        for dex_map in dexes {
            let dex = DexFile::from(dex_map);
            self.package_tree(&mut root, dex)
        }
    }

    pub fn get_all_method(&self, dex: &DexFile) -> MultiMap<String, String, &Method> {
        let mut map = MultiMap::new();
        for clz in &dex.classes {
            for method in clz.methods() {
                let source_file = clz.source_file().unwrap();
                map.insert(source_file.to_string(), String::from(""), method.clone());
            }
        }

        map
    }
    pub fn get_all_field(&self, dex: &DexFile) {}
    pub fn get_all_type(&self, dex: &DexFile) {}

    pub fn package_tree(&self, root: &mut DexPackageNode, dex: DexFile) {
        let method_map = self.get_all_method(&dex);
        // self.get_all_field(&dex);
        // self.get_all_type(&dex);

        for clz in &dex.classes {

        }
    }
}