// #![allow(clippy::assigning_clones)]
// use futures_util::StreamExt;

use std::{collections::HashMap, rc::Rc};

use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, prelude::OAuthClient, scopes};
use url::Url;
use yew::prelude::Callback as YewCallback;
use yew::prelude::*;
use yew_router::{AnyRoute, prelude::*};
// use playlist::fetch_playlist;
// use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, prelude::*, scopes};
// use score::search;
// use tracing::warn;
use yew::{Html, function_component, html, use_state};
// mod playlist;
// mod score;

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub token: Rc<Option<String>>,
    pub set_token: YewCallback<String>,
}

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
    let location = use_location().unwrap();
    let auth = use_context::<AuthContext>().expect("AuthContext missing");
    let query = location.query_str();
    let query = format!("http://a.com/{query}")
        .parse::<Url>()
        .unwrap()
        .query_pairs()
        .find_map(|(k, v)| {
            if k == "code" {
                Some(v.to_string())
            } else {
                None
            }
        })
        .unwrap();

    let q2 = query.clone();
    let tok = auth.token
    let set_token = auth.set_token.clone();
    // set_token.emit(query.to_string());

    html! {
        <div>
            <p>{ "callback page :)" }</p>
            <p>{ query }</p>
            <p>{ format!("{:?}", q2) }</p>
            <p>{ "auth context"}</p>
            <p>{ format!("{:?}", auth.token)}</p>
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
    let token = use_state(|| None);
    let context = AuthContext {
        token: Rc::new((*token).clone()),
        set_token: {
            let token = token.clone();
            YewCallback::from(move |new_token: String| {
                token.set(Some(new_token));
            })
        },
    };

    html! {
        <ContextProvider<AuthContext> context={context}>
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </ContextProvider<AuthContext>>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
