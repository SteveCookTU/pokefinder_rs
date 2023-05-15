#[cfg(not(target_arch = "wasm32"))]
use crate::gen3::Profile3;
#[cfg(not(target_arch = "wasm32"))]
use crate::gen4::Profile4;
#[cfg(not(target_arch = "wasm32"))]
use crate::gen5::Profile5;
#[cfg(not(target_arch = "wasm32"))]
use crate::gen8::Profile8;
use serde::de::DeserializeOwned;
use serde::Serialize;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::{File, OpenOptions};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{BufReader, BufWriter};
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;

#[cfg(not(target_arch = "wasm32"))]
static PATH: Mutex<String> = Mutex::new(String::new());

#[cfg(not(target_arch = "wasm32"))]
fn read_json<T: DeserializeOwned + Default>(file: &str) -> T {
    let mut path = PathBuf::from(PATH.lock().unwrap().clone());
    path.push(file);
    let Ok(file) = File::open(path) else {
        return T::default();
    };
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to deserialize json")
}

#[cfg(not(target_arch = "wasm32"))]
fn write_json<T: Serialize>(file: &str, obj: &T) {
    let mut path = PathBuf::from(PATH.lock().unwrap().clone());
    path.push(file);
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .expect("Failed to open profile path");
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, obj).expect("Failed to serialize struct to json")
}

#[cfg(target_arch = "wasm32")]
fn write_json<T: Serialize>(key: &str, obj: &T) {
    if let Some(window) = web_sys::window() {
        let json = serde_json::to_string(obj).unwrap_or_default();
        if let Ok(Some(local_storage)) = window.local_storage() {
            let _ = local_storage.set_item(key, &json);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn read_json<T: DeserializeOwned + Default>(key: &str) -> T {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            if let Ok(Some(json)) = local_storage.get(key) {
                return serde_json::from_str(&json).unwrap_or_default();
            }
        }
    }
    T::default()
}

/// Initializes the path to the directory which will hold profile data on non-wasm builds.
///
/// WASM targets use web-sys to save profile data to local storage of the browser
#[cfg(not(target_arch = "wasm32"))]
pub fn init_profile_loader(location: String) -> bool {
    *PATH.lock().unwrap() = location.clone();
    let exists = Path::new(&location).exists();
    if !exists {
        write_json("gen3.json", &Vec::<Profile3>::new());
        write_json("gen4.json", &Vec::<Profile4>::new());
        write_json("gen5.json", &Vec::<Profile5>::new());
        write_json("gen8.json", &Vec::<Profile8>::new());
    }
    exists
}

/// Functions for handling Gen 3 profiles ([`Profile3`])
pub mod profile_loader_3 {
    use crate::gen3::Profile3;
    use crate::parents::profile_loader::{read_json, write_json};

    /// Adds a profile to the stored json
    pub fn add_profile(profile: Profile3) {
        let mut profiles = read_json::<Vec<Profile3>>("gen3.json");
        profiles.push(profile);
        write_json("gen3.json", &profiles);
    }

    /// Reads profiles from the stored json
    pub fn get_profiles() -> Vec<Profile3> {
        read_json("gen3.json")
    }

    /// Deletes a profile from the stored json
    pub fn remove_profile(profile: &Profile3) {
        let profiles = read_json::<Vec<Profile3>>("gen3.json")
            .into_iter()
            .filter(|p| p.ne(profile))
            .collect::<Vec<_>>();
        write_json("gen3.json", &profiles);
    }

    /// Updates a profile from the stored json
    pub fn update_profile(update: &Profile3, original: &Profile3) {
        if update != original {
            let mut profiles = read_json::<Vec<Profile3>>("gen3.json");
            if let Some(prof) = profiles.iter_mut().find(|p| p.eq(&original)) {
                *prof = update.clone();
            }
            write_json("gen3.json", &profiles);
        }
    }
}

/// Functions for handling Gen 4 profiles ([`Profile4`])
pub mod profile_loader_4 {
    use crate::gen4::Profile4;
    use crate::parents::profile_loader::{read_json, write_json};

    /// Adds a profile to the stored json
    pub fn add_profile(profile: Profile4) {
        let mut profiles = read_json::<Vec<Profile4>>("gen4.json");
        profiles.push(profile);
        write_json("gen4.json", &profiles);
    }

    /// Reads profiles from the stored json
    pub fn get_profiles() -> Vec<Profile4> {
        read_json("gen4.json")
    }

    /// Deletes a profile from the stored json
    pub fn remove_profile(profile: &Profile4) {
        let profiles = read_json::<Vec<Profile4>>("gen4.json")
            .into_iter()
            .filter(|p| p.ne(profile))
            .collect::<Vec<_>>();
        write_json("gen4.json", &profiles);
    }

    /// Updates a profile from the stored json
    pub fn update_profile(update: &Profile4, original: &Profile4) {
        if update != original {
            let mut profiles = read_json::<Vec<Profile4>>("gen4.json");
            if let Some(prof) = profiles.iter_mut().find(|p| p.eq(&original)) {
                *prof = update.clone();
            }
            write_json("gen4.json", &profiles);
        }
    }
}

/// Functions for handling Gen 5 profiles ([`Profile5`])
pub mod profile_loader_5 {
    use crate::gen5::Profile5;
    use crate::parents::profile_loader::{read_json, write_json};

    /// Adds a profile to the stored json
    pub fn add_profile(profile: Profile5) {
        let mut profiles = read_json::<Vec<Profile5>>("gen5.json");
        profiles.push(profile);
        write_json("gen5.json", &profiles);
    }

    /// Reads profiles from the stored json
    pub fn get_profiles() -> Vec<Profile5> {
        read_json("gen5.json")
    }

    /// Deletes a profile from the stored json
    pub fn remove_profile(profile: &Profile5) {
        let profiles = read_json::<Vec<Profile5>>("gen5.json")
            .into_iter()
            .filter(|p| p.ne(profile))
            .collect::<Vec<_>>();
        write_json("gen5.json", &profiles);
    }

    /// Updates a profile from the stored json
    pub fn update_profile(update: &Profile5, original: &Profile5) {
        if update != original {
            let mut profiles = read_json::<Vec<Profile5>>("gen5.json");
            if let Some(prof) = profiles.iter_mut().find(|p| p.eq(&original)) {
                *prof = update.clone();
            }
            write_json("gen5.json", &profiles);
        }
    }
}

/// Functions for handling Gen 8 profiles ([`Profile8`])
pub mod profile_loader_8 {
    use crate::gen8::Profile8;
    use crate::parents::profile_loader::{read_json, write_json};

    /// Adds a profile to the stored json
    pub fn add_profile(profile: Profile8) {
        let mut profiles = read_json::<Vec<Profile8>>("gen8.json");
        profiles.push(profile);
        write_json("gen8.json", &profiles);
    }

    /// Reads profiles from the stored json
    pub fn get_profiles() -> Vec<Profile8> {
        read_json("gen8.json")
    }

    /// Deletes a profile from the stored json
    pub fn remove_profile(profile: &Profile8) {
        let profiles = read_json::<Vec<Profile8>>("gen8.json")
            .into_iter()
            .filter(|p| p.ne(profile))
            .collect::<Vec<_>>();
        write_json("gen8.json", &profiles);
    }

    /// Updates a profile from the stored json
    pub fn update_profile(update: &Profile8, original: &Profile8) {
        if update != original {
            let mut profiles = read_json::<Vec<Profile8>>("gen8.json");
            if let Some(prof) = profiles.iter_mut().find(|p| p.eq(&original)) {
                *prof = update.clone();
            }
            write_json("gen8.json", &profiles);
        }
    }
}
