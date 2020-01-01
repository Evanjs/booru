use std::collections::HashSet;


#[derive(Serialize, Deserialize, Default)]
pub struct Tags {
    pub tags: HashSet<Tag>,
}

// Can't implement Default for enums, thus can't have any enums in a HashSet, until perhaps RFC (PR 2593) is approved
// `tags::TagType` doesn't implement `std::fmt::Debug`
// https://stackoverflow.com/a/36777679
// https://github.com/rust-lang/rfcs/pull/2593
//#[derive(Serialize, Deserialize, Debug, Default)]
//#[serde(untagged)]
//pub enum TagType {
//    Artist,
//    Character,
//    Copyright,
//    General,
//    Meta
//}
//
//impl TagType {
//    pub fn as_str(&self) -> &str {
//        let tag_string = "tag_string_".to_owned();
//        match self {
//            &TagType::Artist =>  tag_string.push_str("artist"),
//            &TagType::Character => tag_string.push_str("character"),
//            &TagType::Copyright => tag_string.push_str("copyright"),
//            &TagType::General => tag_string.push_str("general"),
//            &TagType::Meta => tag_string.push_str("meta"),
//        }
//        &tag_string
//    }
//}
//
//impl FromStr for TagType {
//    type Err = ();
//
//    fn from_str(s: &str) -> Result<TagType, ()> {
//        match s {
//            "artist" => Ok(TagType::Artist),
//            "character" => Ok(TagType::Character),
//            "copyright" => Ok(TagType::Copyright),
//            "general" => Ok(TagType::General),
//            "meta" => Ok(TagType::Meta),
//            _ => Err(()),
//        }
//    }
//}

#[derive(Serialize, Deserialize, Debug, Default, Hash, Eq, PartialEq)]
pub struct Tag {
    pub tag: String,
    pub tag_type: String,
}

impl Tag {
    pub fn new(tag: String, tag_type: String) -> Tag {
        Tag { tag, tag_type }
    }
}
