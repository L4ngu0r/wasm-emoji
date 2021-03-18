use std::ffi::CString;
use std::os::raw::c_char;
use std::collections::BTreeMap;

const GRINNING_FACE: &'static str = "1F600";
const GRINNING_FACE_WITH_BIG_EYES: &'static str = "1F603";
const GRINNING_FACE_WITH_SMILING_EYES: &'static str = "1F604";
const BEAMING_FACE_WITH_SMILING_EYES: &'static str = "1F601";
const GRINNING_SQUINTING_FACE: &'static str = "1F606";
const GRINNING_FACE_WITH_SWEAT: &'static str = "1F605";

#[no_mangle]
pub fn get_emoji(emoji_name: &str) -> *mut c_char {

    let mut map: BTreeMap<&'static str, &'static str> = BTreeMap::new();

    map.insert("GRINNING_FACE", "1F600");
    map.insert("GRINNING_FACE_WITH_BIG_EYES", "1F603");
    map.insert("GRINNING_FACE_WITH_SMILING_EYES", "1F604");

    let mut results: Vec<&'static str> = Vec::new();
    for (_k, v) in map.range(emoji_name..) {
        results.push(v);
    }

    let s = CString::new(results.join("-")).unwrap();
    s.into_raw()
}
