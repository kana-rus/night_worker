#![feature(impl_trait_in_assoc_type)]

pub mod d1;
pub mod kv;
pub mod queue;
pub mod durable_objects;

use std::future::Future;
use worker::wasm_bindgen::JsValue;

pub struct Worker<'w> {
    env: &'w worker::Env,
    ctx: &'w worker::Context,
}

const _: () = {
    impl<'w> Worker<'w> {
        pub const fn take_over(env: &'w worker::Env, ctx: &'w worker::Context) -> Self {
            Self { env, ctx }
        }
    }

    impl<'w> Worker<'w> {
        pub fn wait(&self, task: impl Future<Output = ()> + 'static) {
            self.ctx.wait_until(task)
        }
    }
    
    #[allow(non_snake_case)]
    impl<'w> Worker<'w> {
        pub fn KV(&self, name: &'static str) -> Result<kv::KV, Error> {
            self.env.kv(name).map_err(Error::Worker).map(kv::KV)
        }
        
        pub fn D1(&self, name: &'static str) -> Result<d1::D1, Error> {
            self.env.d1(name).map_err(Error::Worker).map(d1::D1)
        }

        pub fn Queue(&self, name: &'static str) -> Result<queue::Queue, Error> {
            self.env.queue(name).map_err(Error::Worker).map(queue::Queue)
        }

        pub fn var(&self, name: &'static str) -> Result<String, Error> {
            let string_binding = self.env.var(name).map_err(Error::Worker)?;
            Into::<JsValue>::into(string_binding).as_string().ok_or_else(|| Error::NotFound { what:"Environment variable", name })
        }

        pub fn secret(&self, name: &'static str) -> Result<String, Error> {
            let string_binding = self.env.secret(name).map_err(Error::Worker)?;
            Into::<JsValue>::into(string_binding).as_string().ok_or_else(|| Error::NotFound { what:"Secret", name })
        }
    }
};

pub enum Error {
    Worker(worker::Error),
    KV(worker::kv::KvError),
    NotFound { what: &'static str, name: &'static str },
}

const _: () = {
    unsafe impl Send for Error {}
    unsafe impl Sync for Error {}

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::KV(e)               => e.fmt(f),
                Self::Worker(e)           => e.fmt(f),
                Self::NotFound { what, name } => Ok({
                    f.write_str(what)?;
                    f.write_str(" `")?;
                    f.write_str(name)?;
                    f.write_str("` is not found")?;
                })
            }
        }
    }
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::KV(e)               => e.fmt(f),
                Self::Worker(e)           => e.fmt(f),
                Self::NotFound { what, name } => Ok({
                    f.write_str(what)?;
                    f.write_str(" `")?;
                    f.write_str(name)?;
                    f.write_str("` is not found")?;
                })
            }
        }
    }
    impl std::error::Error for Error {}
};
