// #![allow(clippy::assigning_clones)]
// use futures_util::StreamExt;

use std::collections::HashMap;

use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, prelude::OAuthClient, scopes};
use yew::prelude::*;
use yew_router::{AnyRoute, prelude::*};
// use playlist::fetch_playlist;
// use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, prelude::*, scopes};
// use score::search;
// use tracing::warn;
use yew::{Html, function_component, html, use_state};
// mod playlist;
// mod score;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/callback")]
    Callback,
}
// #[tokio::main]
// async fn main() {
//     // You can use any logger for debugging.
//     env_logger::init();

//     // Set RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file (after
//     // enabling the `env-file` feature) or export them manually:
//     //
//     // export RSPOTIFY_CLIENT_ID="your client_id"
//     //
//     // It will then be read with `from_env`.
//     //
//     // Otherwise, set client_id explictly:
//     //
//     // ```
//     // let creds = Credentials::new_pkce("my-client-id");
//     // ```
//     let creds = Credentials::from_env().unwrap();

//     // Same for RSPOTIFY_REDIRECT_URI. You can also set it explictly:
//     //
//     // ```
//     // let oauth = OAuth {
//     //     redirect_uri: "http://localhost:8888/callback".to_string(),
//     //     scopes: scopes!("user-read-recently-played"),
//     //     ..Default::default(),
//     // };
//     // ```
//     let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();

//     let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

//     // Obtaining the access token
//     let url = spotify.get_authorize_url(None).unwrap();
//     // This function requires the `cli` feature enabled.
//     spotify.prompt_for_token(&url).await.unwrap();

//     // Running the requests
//     // https://open.spotify.com/playlist/4OUW72tbxXVLgi4zkY14Kh?si=42b340478591412a

//     let mut tracks = fetch_playlist(&spotify, "4OUW72tbxXVLgi4zkY14Kh".into())
//         .await
//         .unwrap();
//     while let Some(full_track) = tracks.next().await {
//         let full_track = full_track.unwrap();
//         let resp = search(&full_track.name).await.unwrap();
//         if let Some(song) = resp.first() {
//             println!("{} -> {}", full_track.name, song.link())
//         } else {
//             warn!("No song found for track: {}", full_track.name);
//         }
//     }
// }

#[function_component(Home)]
fn home() -> Html {
    // let counter = use_state(|| 0);
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter + 1;
    //         counter.set(value);
    //     }
    // };

    // let url =

    // html! {
    //     <div>
    //         <button {onclick}>{ "+1" }</button>
    //         <p>{ *counter }</p>
    //     </div>
    // }

    let creds = Credentials::new("0613391cb83444989583bf6009fecef6", "");
    let oauth = OAuth {
        redirect_uri: "http://127.0.0.1:8888/callback".into(),
        scopes: scopes!("user-read-playback-state"),
        ..Default::default()
    };
    // let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();
    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());
    let url = spotify.get_authorize_url(None).unwrap();
    println!("{:?}", url);
    html! {
        <div>
            <p>{ "+1" }</p>
            <a href={url}>{ "spotify auth" }</a>
        </div>
    }
}

#[function_component(Callback)]
fn callback() -> Html {
    // let context = use_route::<Route>().unwrap();
    let location = use_location().unwrap();
    let state = use_state(|| 0);
    let query = location.query_str();
    let query = query.strip_prefix("?code=").unwrap();

    html! {
        <div>
            <p>{ "callback page :)" }</p>
            <p>{ query }</p>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::Callback => html! {
            <Callback />
        },
        // Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
