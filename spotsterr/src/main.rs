#![allow(clippy::assigning_clones)]
use futures_util::StreamExt;

use playlist::fetch_playlist;
use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, prelude::*, scopes};
mod playlist;
mod score;

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Set RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file (after
    // enabling the `env-file` feature) or export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    //
    // It will then be read with `from_env`.
    //
    // Otherwise, set client_id explictly:
    //
    // ```
    // let creds = Credentials::new_pkce("my-client-id");
    // ```
    let creds = Credentials::from_env().unwrap();

    // Same for RSPOTIFY_REDIRECT_URI. You can also set it explictly:
    //
    // ```
    // let oauth = OAuth {
    //     redirect_uri: "http://localhost:8888/callback".to_string(),
    //     scopes: scopes!("user-read-recently-played"),
    //     ..Default::default(),
    // };
    // ```
    let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();

    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    // Obtaining the access token
    let url = spotify.get_authorize_url(None).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).await.unwrap();

    // Running the requests
    // https://open.spotify.com/playlist/4OUW72tbxXVLgi4zkY14Kh?si=42b340478591412a

    let mut tracks = fetch_playlist(&spotify, "4OUW72tbxXVLgi4zkY14Kh".into())
        .await
        .unwrap();
    while let Some(full_track) = tracks.next().await {
        println!("{:?}", full_track.unwrap().name);
    }
}
