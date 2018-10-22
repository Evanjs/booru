pub mod posts;

extern crate reqwest;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

extern crate more_asserts;
extern crate core;

use image::*;
//fn get_posts(limit: u32, page: u32, random: u32, raw: u32) -> posts::RootInterface {
//    let mut map = HashMap::new();
//    map.insert("limit", limit);
//    map.insert("page", page);
//    map.insert("random", random);
//    map.insert("raw", raw);
//
//    let url = reqwest::Url::parse_with_params("https://danbooru.donmai.us/posts.json", &map);
//
//    let response = reqwest::get(url);
//    let text = response.unwrap().text().unwrap();
//    let posts: posts::RootInterface = serde_json::from_str(&text).unwrap();
//    posts
//}

fn get_posts() -> Vec<posts::RootInterface> {
    let url = reqwest::Url::parse("https://danbooru.donmai.us/posts.json").ok().unwrap();

    let response = reqwest::get(url).unwrap().text().unwrap().to_string();
    let posts: Vec<posts::RootInterface> = serde_json::from_str(&response).unwrap();
    posts
}

pub fn search_tag(query: String) -> Vec<posts::RootInterface> {
    let url = reqwest::Url::parse(format!("https://danbooru.donmai.us/posts.json?limit=100&tags={}", query).as_str()).ok().unwrap();
    let response = reqwest::get(url).unwrap().text().unwrap().to_string();
    let posts: Vec<posts::RootInterface> = serde_json::from_str(&response).unwrap();
    posts
}

// todo: most of these functions accomplish half of the same things, ie basic get request -> Post list.  try to move url parsing and response handling out
pub fn get_post_by_id(id: u64) -> posts::RootInterface {
    let url = reqwest::Url::parse(format!("https://danbooru.donmai.us/posts/{}.json", id).as_str()).ok().unwrap();
//    println!("Going to try to get response from url {}", url.to_string());
    let response = reqwest::get(url).unwrap().text().unwrap().to_string();
    let post: posts::RootInterface = serde_json::from_str(&response).unwrap();
    post
}

fn get_image(post: posts::RootInterface) -> DynamicImage {
    let url = reqwest::Url::parse(&post.get_file_url()).ok().unwrap();
    let mut buf: Vec<u8> = vec![];
    let h_img = reqwest::get(url).unwrap().copy_to(&mut buf);
    let img = image::load_from_memory(&buf);
    img.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn null_fields() {
        let data = get_posts();
        assert_gt!(data.len(), 0);
        data.iter().filter(|p| p.get_last_comment_bumped_at() == "").for_each(|x| println!("{:?}", x));
    }

    #[test]
    fn get_image() {
        let u = "https://danbooru.donmai.us/data/sample/sample-e976cde6cc68aca3108527561798b980.jpg";
        let post = get_post_by_id("3293386".to_string());
        let img = super::get_image(post);
        assert_eq!(img.width(), 1032);
        assert_eq!(img.height(), 838);
    }

    #[test]
    fn get_post() {
        let post = get_post_by_id("3293386".to_string());
    }


}
