#[derive(Serialize, Deserialize, Debug, Default)]
struct KeeperData {
    uid: u64,
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RootInterface {
    id: u64,
    created_at: String,
    uploader_id: u64,
    score: i64,
    source: String,
    #[serde(deserialize_with = "default_if_empty")]
    #[serde(default)]
    md5: String,
    #[serde(deserialize_with = "default_if_empty")]
    last_comment_bumped_at: String,
    rating: String,
    image_width: u64,
    image_height: u64,
    tag_string: String,
    is_note_locked: bool,
    fav_count: u64,
    #[serde(deserialize_with = "default_if_empty")]
    #[serde(default)]
    file_ext: String,
    #[serde(deserialize_with = "default_if_empty")]
    last_noted_at: String,
    is_rating_locked: bool,
    #[serde(deserialize_with = "default_if_empty")]
    parent_id: u64,
    has_children: bool,
    #[serde(deserialize_with = "default_if_empty")]
    approver_id: u64,
    tag_count_general: u64,
    tag_count_artist: u64,
    tag_count_character: u64,
    tag_count_copyright: u64,
    file_size: u64,
    is_status_locked: bool,
    pool_string: String,
    up_score: i64,
    down_score: i64,
    is_pending: bool,
    is_flagged: bool,
    is_deleted: bool,
    tag_count: u64,
    updated_at: String,
    is_banned: bool,
    #[serde(deserialize_with = "default_if_empty")]
    pixiv_id: u64,
    #[serde(deserialize_with = "default_if_empty")]
    last_commented_at: String,
    has_active_children: bool,
    bit_flags: u64,
    tag_count_meta: u64,
    #[serde(deserialize_with = "default_if_empty")]
    #[serde(default)]
    keeper_data: KeeperData,
    uploader_name: String,
    has_large: bool,
    has_visible_children: bool,
    #[serde(deserialize_with = "default_if_empty")]
    children_ids: String,
    is_favorited: bool,
    tag_string_general: String,
    tag_string_character: String,
    tag_string_copyright: String,
    tag_string_artist: String,
    tag_string_meta: String,
    #[serde(default)]
    #[serde(deserialize_with = "default_if_empty")]
    file_url: String,
    #[serde(default)]
    #[serde(deserialize_with = "default_if_empty")]
    large_file_url: String,
    #[serde(default)]
    #[serde(deserialize_with = "default_if_empty")]
    preview_file_url: String,
}

impl RootInterface {

    pub fn get_large_file_url(&self) -> String {
        return self.large_file_url.to_string();
    }

    pub fn get_preview_file_url(&self) -> String {
        return self.preview_file_url.to_string();
    }

    pub fn get_image_height(&self) -> u64 {
        return self.image_height;
    }

    pub fn get_id(&self) -> u64 {
        return self.id;
    }

    pub fn get_file_ext(&self) -> String {
        return self.file_ext.to_string();
    }

    pub fn get_image_width(&self) -> u64 {
        return self.image_width;
    }

    pub fn get_file_url(&self) -> String {
        return self.file_url.to_string();
    }

    pub fn get_last_comment_bumped_at(&self) -> String {
        return self.last_comment_bumped_at.to_string();
    }

    pub fn get_tag_string(&self) -> String {
        return self.tag_string.to_string();
    }

    pub fn get_tag_string_copyright(&self) -> String {
        return self.tag_string_copyright.to_string();
    }

    pub fn get_tag_string_character(&self) -> String {
        return self.tag_string_character.to_string();
    }

    pub fn get_tag_string_general(&self) -> String {
        return self.tag_string_general.to_string();
    }

    pub fn get_tag_string_artist(&self) -> String {
        return self.tag_string_artist.to_string();
    }

    pub fn get_tag_string_meta(&self) -> String {
        return self.tag_string_meta.to_string();
    }
}

fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
    where D: serde::Deserializer<'de>, T: serde::Deserialize<'de> + Default,
{
    use serde::Deserialize;
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}

