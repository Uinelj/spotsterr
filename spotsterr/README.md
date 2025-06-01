TODO:
- read/write auth code
```
Full Setup: Shared mutable token via Context

We'll:

    Define a context type that includes both the token and a setter.

    Provide it at a high level (e.g. in <App />).

    Consume and mutate it in any child (e.g. after a login).

1. Define the context

use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub token: Rc<String>,
    pub set_token: Callback<String>,
}

    Rc<String> is used so the context is Clone and PartialEq friendly.

2. Provide it in <App />

#[function_component(App)]
fn app() -> Html {
    let token = use_state(|| "".to_string());

    let context = AuthContext {
        token: Rc::new((*token).clone()),
        set_token: {
            let token = token.clone();
            Callback::from(move |new_token: String| {
                token.set(new_token);
            })
        },
    };

    html! {
        <ContextProvider<AuthContext> context={context}>
            <AppRoutes />
        </ContextProvider<AuthContext>>
    }
}

3. Consume and mutate in any component
Reading the token:

#[function_component(Profile)]
fn profile() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext missing");

    html! {
        <div>{ format!("Current token: {}", auth.token) }</div>
    }
}

Updating the token (e.g. on login):

#[function_component(Login)]
fn login() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext missing");

    let onclick = {
        let set_token = auth.set_token.clone();
        Callback::from(move |_| {
            set_token.emit("new_access_token_123".to_string());
        })
    };

    html! {
        <button {onclick}>{ "Login" }</button>
    }
}

✅ Benefits of this setup:

    Easy access and updates from anywhere.

    Re-renders on update.

    Clean and testable.

Let me know if you want to persist the token (e.g., localStorage) or fetch it asynchronously.
```
- persist code on localstorage?
- tailwind/https://daisyui.com/
