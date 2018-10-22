#[derive(Serialize, Deserialize, Debug)]
struct KeeperData {
    uid: i64,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct RootInterface {
    id: i64,
    created_at: String,
    uploader_id: i64,
    score: i64,
    source: String,
    #[serde(deserialize_with = "default_if_empty")]
    #[serde(default)]
    md5: String,
    #[serde(deserialize_with = "default_if_empty")]
    pub last_comment_bumped_at: String,
    rating: String,
    image_width: i64,
    image_height: i64,
    tag_string: String,
    is_note_locked: bool,
    fav_count: i64,
    #[serde(deserialize_with = "default_if_empty")]
    #[serde(default)]
    file_ext: String,
    #[serde(deserialize_with = "default_if_empty")]
    last_noted_at: String,
    is_rating_locked: bool,
    #[serde(deserialize_with = "default_if_empty")]
    parent_id: i64,
    has_children: bool,
    #[serde(deserialize_with = "default_if_empty")]
    approver_id: i64,
    tag_count_general: i64,
    tag_count_artist: i64,
    tag_count_character: i64,
    tag_count_copyright: i64,
    file_size: i64,
    is_status_locked: bool,
    pool_string: String,
    up_score: i64,
    down_score: i64,
    is_pending: bool,
    is_flagged: bool,
    is_deleted: bool,
    tag_count: i64,
    updated_at: String,
    is_banned: bool,
    #[serde(deserialize_with = "default_if_empty")]
    pixiv_id: i64,
    #[serde(deserialize_with = "default_if_empty")]
    last_commented_at: String,
    has_active_children: bool,
    bit_flags: i64,
    tag_count_meta: i64,
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

fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
    where D: serde::Deserializer<'de>, T: serde::Deserialize<'de> + Default,
{
    use serde::Deserialize;
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}

