use worker::{send::SendFuture, Fetcher, Headers, Method, Request, RequestInit, Response};
use worker::wasm_bindgen::JsValue;
use std::future::{Future, IntoFuture};
use crate::Error;

pub struct ServiceBinding(pub(crate) Fetcher);
unsafe impl Send for ServiceBinding {}
unsafe impl Sync for ServiceBinding {}

const _: (/* direct fetch */) = {
    impl ServiceBinding {
        pub fn fetch(&self, request: Request) -> impl Future<Output = Result<Response, Error>> + Send + '_ {
            SendFuture::new(async {
                self.0.fetch_request(request).await.map_err(Error::Worker)
            })
        }
    }
};

const _: (/* building fetch */) = {
    macro_rules! init_fetch_service {
        ( $( $method:ident as $worker_method:ident ),* ) => {
            #[allow(non_snake_case)]
            impl ServiceBinding {
                $(
                    pub fn $method(&self, url: impl Into<String>) -> FetchService {
                        let mut req = RequestInit::new();
                        req.with_method(Method::$worker_method);
                        FetchService {
                            service: self,
                            url: url.into(),
                            req
                        }
                    }
                )*
            }
        };
    } init_fetch_service! {
        GET as Get,
        PUT as Put,
        POST as Post,
        PATCH as Patch,
        DELETE as Delete
    }

    pub struct FetchService<'s> {
        service: &'s ServiceBinding,
        url: String,
        req: RequestInit,
    }

    impl<'s> FetchService<'s> {
        pub fn headers(mut self, headers: Headers) -> Self {
            self.req.with_headers(headers);
            self
        }

        pub fn content(mut self, content: impl Into<JsValue>) -> Self {
            self.req.with_body(Some(content.into()));
            self
        }
    }

    impl<'s> IntoFuture for FetchService<'s> {
        type Output     = Result<Response, Error>;
        type IntoFuture = impl Future<Output = Self::Output> + Send;

        fn into_future(self) -> Self::IntoFuture {
            SendFuture::new(async {
                self.service.0.fetch(self.url, Some(self.req)).await.map_err(Error::Worker)
            })
        }
    }
};
