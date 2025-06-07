// #![allow(clippy::assigning_clones)]
// use futures_util::StreamExt;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

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

#[derive(Debug, Clone)]
struct AuthCodePkceSpotifyEq(AuthCodePkceSpotify);

impl PartialEq for AuthCodePkceSpotifyEq {
    fn eq(&self, other: &Self) -> bool {
        self.0.creds.id == other.0.creds.id && self.0.creds.secret == other.0.creds.secret
    }
}

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    // pub token: RefCell<Option<String>>,
    pub spotify: RefCell<Option<AuthCodePkceSpotifyEq>>,
}
impl AuthContext {
    pub fn new(
        token: Option<String>,
        client_id: String,
        redirect_uri: String,
        scopes: HashSet<String>,
    ) -> Self {
        let spotify = AuthCodePkceSpotify::new(
            Credentials {
                id: client_id,
                secret: None,
            },
            OAuth {
                redirect_uri: "http://127.0.0.1:8888/callback".into(),
                scopes: scopes!("user-read-playback-state"),
                ..Default::default()
            },
        );

        Self {
            // token: RefCell::new(token),
            spotify: RefCell::new(Some(AuthCodePkceSpotifyEq(spotify))),
        }
    }
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
//

fn home_no_auth(auth_context: AuthContext) -> Html {
    let mut spotify = AuthCodePkceSpotify::new(
        Credentials {
            id: "0613391cb83444989583bf6009fecef6".to_string(),
            secret: None,
        },
        OAuth {
            redirect_uri: "http://127.0.0.1:8888/callback".into(),
            scopes: scopes!("user-read-playback-state"),
            ..Default::default()
        },
    );
    // let ctx = AuthContext::new(
    //     None,
    //     "0613391cb83444989583bf6009fecef6".to_string(),
    //     "http://127.0.0.1:8888/callback".to_string(),
    //     scopes!("user-read-playback-state"),
    // );

    let url = { spotify.get_authorize_url(None).unwrap() };

    auth_context
        .spotify
        .replace(Some(AuthCodePkceSpotifyEq(spotify)));

    let html_ret = html! {
        <div>
            <a href={url}>{ "spotify auth" }</a>
        </div>
    };

    html_ret
}

#[function_component(Home)]
fn home() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext missing");
    let auth_borrow = auth.spotify.borrow();
    let html_ret = match auth_borrow.as_ref() {
        Some(auth) => html!(<div>{ "authenticated" }</div>),
        None => home_no_auth(auth),
    };

    html_ret
    // if auth.token.borrow().is_some() {}
    // let creds = Credentials::new("0613391cb83444989583bf6009fecef6", "");
    // let oauth = OAuth {
    //     redirect_uri: "http://127.0.0.1:8888/callback".into(),
    //     scopes: scopes!("user-read-playback-state"),
    //     ..Default::default()
    // };
    // // let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();
    // let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());
    // let url = spotify.get_authorize_url(None).unwrap();
    // html! {
    //     <div>
    //         <a href={url}>{ "spotify auth" }</a>
    //     </div>
    // }
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

    // let q2 = query.clone();
    {
        let mut tok = auth.token.try_borrow_mut().unwrap();
        *tok = Some(query);
    }

    html! {
        <div>
            <p>{ "callback page :)" }</p>
            // <p>{ query }</p>
            <p>{ format!("{:?}", auth.token) }</p>
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
    // let token = use_state(|| None);
    let context = AuthContext {
        token: RefCell::new(None),
        // set_token: {
        //     let token = token.clone();
        //     YewCallback::from(move |new_token: String| {
        //         token.set(Some(new_token));
        //     })
        // },
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
