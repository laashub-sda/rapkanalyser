use std::path::PathBuf;
use crate::analyzer::archives::Archives;
use crate::sdk_constants::ANDROID_MANIFEST_XML;
use crate::manifest::android_manifest_parser::AndroidManifestParser;
use crate::manifest::manifest_data::ManifestData;
use crate::binary_xml::binary_xml_parser::BinaryXmlParser;
use crate::analyzer::apk_size_calculator::{GzipSizeCalculator, ApkSizeCalculator};
use failure::Error;
use crate::analyzer::archive_tree_structure::{ArchiveTreeStructure, ArchiveEntry};
use std::borrow::Cow;
use dex::{DexReader, Dex};
use crate::analyzer::dex::dex_file_stats::DexFileStats;
use std::io::{Read, Write};
use std::fs::File;
use tempfile::tempdir;
use memmap::Mmap;
use crate::analyzer::dex::package_tree_creator::PackageTreeCreator;
use crate::analyzer::dex::dex_package_node::DexPackageNode;

pub struct ApkAnalyzer {}

impl ApkAnalyzer {
    pub fn new() -> ApkAnalyzer {
        ApkAnalyzer {}
    }
    pub fn apk_file_size(&self, apk: PathBuf) -> u64 {
        let calculator = GzipSizeCalculator::new();
        calculator.get_full_apk_raw_size(apk)
    }

    pub fn apk_download_size(&self, apk: PathBuf) -> u64 {
        let calculator = GzipSizeCalculator::new();
        calculator.get_full_apk_download_size(apk)
    }

    pub fn apk_summary(&self, apk: PathBuf) -> ManifestData {
        let result = self.manifest_print(apk);

        let manifest = AndroidManifestParser::parse(Vec::from(result.as_bytes()));
        *manifest
    }

    pub fn files_list(&self, apk: PathBuf) -> Vec<ArchiveEntry> {
        let mut manager = Archives::open(apk);
        let mut vec = ArchiveTreeStructure::create(manager.files);
        vec.sort_by(|a, b| a.path.cmp(&b.path));
        vec
    }

    pub fn file_cat(&self, apk: PathBuf, name: String) -> String {
        let mut manager = Archives::open(apk);
        let data = manager.get(String::from(name));
        String::from(String::from_utf8_lossy(&*data))
    }

    pub fn manifest_print(&self, apk: PathBuf) -> String {
        let mut manager = Archives::open(apk);
        let data = manager.get(String::from(ANDROID_MANIFEST_XML));

        let result = BinaryXmlParser::decode_xml(data).unwrap();
        result
    }

    pub fn dex_list(&self, apk: PathBuf) -> Vec<ArchiveEntry> {
        let mut archive = Archives::open(apk).files;
        let mut results = vec![];
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            if file.name().ends_with(".dex") {
                let entry = ArchiveEntry::from_zip_file(file);
                results.push(entry);
            }
        }
        results
    }

    pub fn dex_references(&self, apk: PathBuf) -> Vec<DexFileStats> {
        let dexes = ApkAnalyzer::get_all_dex_from_apk(apk);
        let mut files_stats: Vec<DexFileStats> = vec![];
        for x in dexes {
            files_stats.push(DexFileStats::create(x))
        }

        files_stats
    }

    pub fn dex_packages(&self, apk: PathBuf) {
        let dexes = ApkAnalyzer::get_all_dex_from_apk(apk);
        let creator = PackageTreeCreator::new();
        creator.construct_package_tree(dexes)
    }

    fn get_all_dex_from_apk(apk: PathBuf) -> Vec<Dex<Mmap>> {
        let mut archive = Archives::open(apk).files;
        let mut dex_results = vec![];

        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            let file_name = zip_file.name();
            if file_name.ends_with(".dex") {
                let mut buffer = Vec::new();
                let dir = tempdir().unwrap();
                let file_path = dir.path().join(file_name);
                let mut file = File::create(file_path.clone()).unwrap();

                zip_file.read_to_end(&mut buffer);
                file.write(&*buffer);

                let result = DexReader::from_file(file_path);
                match result {
                    Ok(data) => {
                        dex_results.push(data);
                    }
                    Err(_) => {}
                }
            }
        };

        dex_results
    }
}

#[cfg(test)]
mod tests {
    use crate::apk_analyzer::ApkAnalyzer;
    use std::path::PathBuf;

    #[test]
    fn should_identify_application_name_from_apk() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let vec = analyzer.apk_summary(path);
        assert_eq!("com.example.android.multiproject", vec.m_package)
    }

    #[test]
    fn should_support_manifest_print() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/test.apk");

        let string = analyzer.manifest_print(path);
        assert_eq!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
<manifest xmlns:android=\"http://schemas.android.com/apk/res/android\" android:versionName=\"1.0\" platformBuildVersionCode=\"23\" package=\"com.example.helloworld\" platformBuildVersionName=\"6.0-2438415\" android:versionCode=\"1\">
  <uses-sdk android:minSdkVersion=\"3\" />
  <intent-filter>
    <action android:name=\"android.intent.action.MAIN\" />
    <category android:name=\"android.intent.category.LAUNCHER\" />
  </intent-filter>
</manifest>".len(), string.len())
    }

    #[test]
    fn should_get_apk_size() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let size = analyzer.apk_file_size(path);
        assert_eq!(48196, size)
    }

    #[test]
    fn should_get_download_size() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let size = analyzer.apk_download_size(path);
        assert_eq!(39591, size)
    }

    #[test]
    fn should_cat_file() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.file_cat(path, String::from("META-INF/MANIFEST.MF"));
        assert_eq!(1233, files.len());
    }

    #[test]
    fn should_list_files() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.files_list(path);
        assert_eq!(18, files.len());
        // for x in files {
        //     println!("{:?}， size: {:?}, download_size: {:?}", x.path, x.raw_size, x.download_size);
        // }
    }

    #[test]
    fn should_list_dex() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.dex_list(path);
        assert_eq!(1, files.len());
        assert_eq!("classes.dex", files[0].path);
    }

    #[test]
    fn should_list_dex_references() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.dex_references(path);
        assert_eq!(40, files[0].referenced_method_count);
    }

    #[test]
    fn should_list_dex_package() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.dex_packages(path);
        // assert_eq!(40, files[0].referenced_method_count);
    }
}