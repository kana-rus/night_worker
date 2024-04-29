/* Execute static tests for sample codes in README */
#![cfg_attr(feature="DEBUG", doc = include_str!("../README.md"))]

#![feature(impl_trait_in_assoc_type)]

pub mod kv;
pub mod service_binding;
// pub mod durable_object;
#[cfg(feature="d1")] pub mod d1;
#[cfg(feature="queue")] pub mod queue;

use std::future::Future;
use worker::{wasm_bindgen::JsValue, Error};

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
        pub fn var(&self, name: &'static str) -> Result<String, Error> {
            let string_binding = self.env.var(name)?;
            Into::<JsValue>::into(string_binding).as_string().ok_or_else(|| Error::BindingError(format!("Variable `{name}` is not found")))
        }

        pub fn secret(&self, name: &'static str) -> Result<String, Error> {
            let string_binding = self.env.secret(name)?;
            Into::<JsValue>::into(string_binding).as_string().ok_or_else(|| Error::BindingError(format!("Secret `{name}` is not found")))
        }

        pub fn Service(&self, name: &'static str) -> Result<service_binding::ServiceBinding, Error> {
            self.env.service(name).map(service_binding::ServiceBinding)
        }

        pub fn KV(&self, name: &'static str) -> Result<kv::KV, Error> {
            self.env.kv(name).map(kv::KV)
        }

        #[cfg(feature="d1")]
        pub fn D1(&self, name: &'static str) -> Result<d1::D1, Error> {
            self.env.d1(name).map(d1::D1)
        }

        #[cfg(feature="queue")]
        pub fn Queue(&self, name: &'static str) -> Result<queue::Queue, Error> {
            self.env.queue(name).map(queue::Queue)
        }
    }
};
