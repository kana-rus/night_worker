pub trait DurableObject: worker::wasm_bindgen::convert::TryFromJsValue {
    fn init(state: worker::State, env: worker::Env) -> Self;

    #[allow(async_fn_in_trait)]
    async fn fetch(&mut self, req: worker::Request) -> Result<worker::Response, crate::Error>;
}

// <plan>

/*

#[DurableObject]
struct UserIDs {
    ids:   Vec<usize>,
    state: worker::State,
    env:   worker::Env,
}

impl DurableObject for UserIDs {
    fn init(state: worker::State, env: worker::Env) -> Self {
        Self { ids: Vec::new(), state, env }
    }

    async fn fetch(&mut self, _req: worker::Request) -> Result<worker::Response, crate::Error> {
        worker::Response::ok(format!("{} active users", self.ids.len())).map_err(crate::Error::Worker)
    }
}

*/

// ↓ expand

/*

#[::worker::durable_object] // <-- This impls some traits including `TryFromJsValue`
struct UserIDs {
    ids:   Vec<usize>,
    state: worker::State,
    env:   worker::Env,
}

impl DurableObject for UserIDs {
    fn init(state: worker::State, env: worker::Env) -> Self {
        Self { ids: Vec::new(), state, env }
    }

    async fn fetch(&mut self, _req: worker::Request) -> Result<worker::Response, worker::Error> {
        worker::Response::ok(format!("{} active users", self.ids.len()))
    }
}

const _: () = {
    use ::worker::{wasm_bindgen, wasm_bindgen_futures, async_trait};

    #[::worker::durable_object]
    impl ::worker::DurableObject for UserIDs {
        fn new(state: ::worker::State, env: ::worker::Env) -> Self {
            <Self as ::night_worker::durable_object::DuarbleObject>::init(state, env)
        }

        async fn fetch(&mut self, _req: ::worker::Request) -> ::worker::Result<::worker::Response> {
            <Self as ::night_worker::durable_object::DuarbleObject>::fetch(_req)
        }
    }
};

*/
