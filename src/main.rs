use url::Url;

const SERVICE_URL: &str = "https://kgsearch.googleapis.com/v1/entities:search";

use gng::args::Args;

fn main() {
    let args = Args::parse();

    let apikey = include_str!("../.api_key");

    let lang: &str = &args.lang.to_string();
    let limit: &str = &args.limit.to_string();

    let url = Url::parse_with_params(
        SERVICE_URL,
        &[
            ("languages", lang),
            ("limit", limit),
            ("key", apikey),
            ("query", &args.query),
        ],
    )
    .unwrap();
    // ToDo クエリエラーの処理

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("{}", body);
}
