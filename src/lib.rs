pub mod posts;
pub mod errors;

extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate more_asserts;
extern crate core;

extern crate image;
extern crate serde_json;


use image::*;

fn get_posts() -> Vec<posts::RootInterface> {
const API_KEY: &'static str = "";
const BASE_URI: &'static str = "https://danbooru.donmai.us/";

fn get_posts() -> Result<Vec<posts::Post>, String> {
    let client = reqwest::Client::new();
    let base_url = reqwest::Url::parse(BASE_URI).ok().expect("failed to parse base url");
    let url = base_url.join("posts.json?").ok().expect("failed to parse url");
    let builder = client.get(url);
    let mut request = builder.send().ok().expect("failed to send request");

    if request.status().is_success() {
        let posts: Vec<posts::Post> = request.json().expect("failed to get posts");
        Ok(posts)
    } else if request.status().is_server_error() {
        let message: errors::ServerError = request.json().ok()
            .expect("failed to parse server error");
        Err(format!("Something else happened. Message: {:?}", message))
    } else {
        Err(format!("Something else happened. Status: {:?}", request.status()))
    }
}

// todo: add error handling for api limits.  example: "You cannot search more than 2 tags"  This is currently panicking from such error
pub fn search_tag(query: String) -> Result<Vec<posts::Post>, String> {
    let client = reqwest::Client::new();
    let base_url: reqwest::Url = reqwest::Url::parse(BASE_URI).ok().unwrap();
    let url = base_url.join("posts.json?").ok().unwrap();
    let builder = client.get(url)
        .query(&[
            ("tags", query),
            ("api_key", API_KEY.to_string()),
            ("limit", "25".to_string())
        ]);
    trace!("{:#?}", builder);
    let mut request = builder.send().ok()
        .expect("failed to send request");

    if request.status().is_success() {
            let posts: Vec<posts::Post> = request.json()
                .expect("failed to get posts");
            Ok(posts)
    } else if request.status().is_server_error() {
        // todo: fix this
        let message: errors::ServerError = request.json()
            .expect("failed to get server error");
        Err(format!("Something else happened. Message: {:?}", message))
    } else {
        Err(format!("Something else happened. Status: {:?}", request.status()))
    }
}

// todo: most of these functions accomplish half of the same things, ie basic get request -> Post list.  try to move url parsing and response handling out
pub fn get_post_by_id(id: u64) -> Result<posts::Post, String> {
    let client = reqwest::Client::new();
    let url = reqwest::Url::parse(format!("{}posts/{}.json", BASE_URI, id).as_str()).ok()
        .expect("failed to parse url");
    trace!("get post by id - url: {:?}", url);

    let builder = client.get(url);
    let mut request = builder.send()
        .expect("failed to send request");

    if request.status().is_success() {
        let post: posts::Post = request.json().unwrap();
        Ok(post)
    } else if request.status().is_server_error() {
        let message: errors::ServerError = request.json().ok().expect("failed to retrieve server error");
        Err(format!("Something else happened. Message: {:?}", message))
    } else {
        Err(format!("Something else happened. Status: {:?}", request.status()))
    }
}

fn get_image(post: posts::Post) -> Result<DynamicImage, ImageError> {
    let url = reqwest::Url::parse(&post.get_file_url()).ok().expect("failed to get thing");
    let mut buf: Vec<u8> = vec![];
    let _ = reqwest::get(url).unwrap().copy_to(&mut buf);
    let img = image::load_from_memory(&buf);
    img
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn null_fields() {
        let data = get_posts().unwrap();
        assert_gt!(data.len(), 0);
        data.iter().filter(|p| p.get_last_comment_bumped_at() == "").for_each(|x| println!("{:?}", x));
    }

    #[test]
    fn get_image() {
        // let u = "https://danbooru.donmai.us/data/sample/sample-e976cde6cc68aca3108527561798b980.jpg";
        let post = get_post_by_id(3293386);
        let img = super::get_image(post.unwrap()).expect("failed to get image");
        assert_eq!(img.width(), 1032);
        assert_eq!(img.height(), 838);
    }

    #[test]
    // #[should_panic] // this will fail unless an api key and login (supporting more than 2 tags) is provided
    // todo: check if this is fixed
    fn multiple_tag_query() {
        let tags = "goblin_slayer rating:s sword_maiden 1girl";
        search_tag(tags.to_string()).ok();
    }

    #[test]
    fn normal_get_posts() {
        let tags = "goblin_slayer! sword_maiden";
        let results = search_tag(tags.to_string()).ok();
    }

    #[test]
    fn get_post() {
        let id = 3293386;
        let post = get_post_by_id(id);
//        assert_eq!(post.get_id(), id)
    }
}
