use crate::gen3::Profile3;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static PATH: Mutex<String> = Mutex::new(String::new());

fn read_json<T: DeserializeOwned + Default>(file: &str) -> T {
    let mut path = PathBuf::from(PATH.lock().unwrap().clone());
    path.push(file);
    let Ok(file) = File::open(path) else {
        return T::default();
    };
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to deserialize json")
}

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

pub fn init_profile_loader(location: String) -> bool {
    *PATH.lock().unwrap() = location.clone();
    let exists = Path::new(&location).exists();
    if !exists {
        write_json("gen3.json", &Vec::<Profile3>::new());
        write_json("gen4.json", &Vec::<Profile3>::new());
        write_json("gen5.json", &Vec::<Profile3>::new());
        write_json("gen8.json", &Vec::<Profile3>::new());
    }
    exists
}

pub mod profile_loader_3 {
    use crate::gen3::Profile3;
    use crate::parents::profile_loader::{read_json, write_json};

    pub fn get_json(profile: &Profile3) -> String {
        serde_json::to_string(profile).expect("Failed to serialize Profile3 to json")
    }

    pub fn get_profile(json: &str) -> Profile3 {
        serde_json::from_str(json).expect("Failed to deserialize json to Profile3")
    }

    pub fn add_profile(profile: Profile3) {
        let mut profiles = read_json::<Vec<Profile3>>("gen3.json");
        profiles.push(profile);
        write_json("gen3.json", &profiles);
    }

    pub fn get_profiles() -> Vec<Profile3> {
        read_json("gen3.json")
    }

    pub fn remove_profile(profile: &Profile3) {
        let profiles = read_json::<Vec<Profile3>>("gen3.json")
            .into_iter()
            .filter(|p| p.ne(profile))
            .collect::<Vec<_>>();
        write_json("gen3.json", &profiles);
    }

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

pub mod profile_loader_4 {
    use crate::gen4::Profile4;
    use crate::parents::profile_loader::{read_json, write_json};

    pub fn get_json(profile: &Profile4) -> String {
        serde_json::to_string(profile).expect("Failed to serialize Profile3 to json")
    }

    pub fn get_profile(json: &str) -> Profile4 {
        serde_json::from_str(json).expect("Failed to deserialize json to Profile3")
    }

    pub fn add_profile(profile: Profile4) {
        let mut profiles = read_json::<Vec<Profile4>>("gen4.json");
        profiles.push(profile);
        write_json("gen4.json", &profiles);
    }

    pub fn get_profiles() -> Vec<Profile4> {
        read_json("gen4.json")
    }

    pub fn remove_profile(profile: &Profile4) {
        let profiles = read_json::<Vec<Profile4>>("gen4.json")
            .into_iter()
            .filter(|p| p.ne(profile))
            .collect::<Vec<_>>();
        write_json("gen4.json", &profiles);
    }

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

pub mod profile_loader_5 {
    use crate::gen5::Profile5;
    use crate::parents::profile_loader::{read_json, write_json};

    pub fn get_json(profile: &Profile5) -> String {
        serde_json::to_string(profile).expect("Failed to serialize Profile3 to json")
    }

    pub fn get_profile(json: &str) -> Profile5 {
        serde_json::from_str(json).expect("Failed to deserialize json to Profile3")
    }

    pub fn add_profile(profile: Profile5) {
        let mut profiles = read_json::<Vec<Profile5>>("gen5.json");
        profiles.push(profile);
        write_json("gen5.json", &profiles);
    }

    pub fn get_profiles() -> Vec<Profile5> {
        read_json("gen5.json")
    }

    pub fn remove_profile(profile: &Profile5) {
        let profiles = read_json::<Vec<Profile5>>("gen5.json")
            .into_iter()
            .filter(|p| p.ne(profile))
            .collect::<Vec<_>>();
        write_json("gen5.json", &profiles);
    }

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

pub mod profile_loader_8 {
    use crate::gen8::Profile8;
    use crate::parents::profile_loader::{read_json, write_json};

    pub fn get_json(profile: &Profile8) -> String {
        serde_json::to_string(profile).expect("Failed to serialize Profile3 to json")
    }

    pub fn get_profile(json: &str) -> Profile8 {
        serde_json::from_str(json).expect("Failed to deserialize json to Profile3")
    }

    pub fn add_profile(profile: Profile8) {
        let mut profiles = read_json::<Vec<Profile8>>("gen8.json");
        profiles.push(profile);
        write_json("gen8.json", &profiles);
    }

    pub fn get_profiles() -> Vec<Profile8> {
        read_json("gen8.json")
    }

    pub fn remove_profile(profile: &Profile8) {
        let profiles = read_json::<Vec<Profile8>>("gen8.json")
            .into_iter()
            .filter(|p| p.ne(profile))
            .collect::<Vec<_>>();
        write_json("gen8.json", &profiles);
    }

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
