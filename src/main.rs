use random_word::Lang;
use serde_derive::Deserialize;
use warp::Filter;

#[derive(Deserialize, Debug)]
struct QueryParams {
    starts_with: Option<char>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let route = warp::get()
        .and(warp::query::<QueryParams>())
        .map(|q: QueryParams| {
            let word = if let Some(starts_with) = q.starts_with {
                random_word::gen_starts_with(starts_with, Lang::En).unwrap_or_default()
            } else {
                random_word::gen(Lang::En)
            };
            Ok(word.to_owned())
        });
    println!("Listening on http://0.0.0.0:8090");
    warp::serve(route).run(([0, 0, 0, 0], 8090)).await;
}
