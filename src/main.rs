use serde_json::Value;
use url::Url;

const SERVICE_URL: &str = "https://kgsearch.googleapis.com/v1/entities:search";
const KG_URL: &str = "http://g.co/kg";

use gng::args::Args;
use gng::tui::draw;
use gng::Item;

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

    let mut items = vec![];
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let body: Value = serde_json::from_str(&body).unwrap();
    if let Value::Array(item_list) = &body["itemListElement"] {
        for item in item_list {
            let result = &item["result"];

            let detail = &result["detailedDescription"];

            let name = result["name"].as_str().unwrap().to_string();
            let desc = result["description"].as_str().map(|x| x.to_string());
            let about = detail["articleBody"]
                .as_str()
                .map(|x| x.to_string().replace("\n", ""));
            let urlid = result["@id"].as_str().unwrap().replace("kg:", KG_URL);
            let item = Item::new(name, desc, about, urlid);
            items.push(item);
        }
    }

    draw(&items);
}
