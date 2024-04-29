use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use serde::Deserialize;
use worker::kv::{GetOptionsBuilder, KvStore, ListOptionsBuilder, ListResponse, PutOptionsBuilder, ToRawKvValue};
use worker::Error;

pub struct KV(pub(crate) KvStore);

const _: (/* get text */) = {
    impl KV {
        pub fn get(&self, key: &str) -> Get {
            Get(self.0.get(key))
        }
    }

    pub struct Get(GetOptionsBuilder);

    impl Get {
        pub fn cache_ttl(self, ttl: u64) -> Self {
            Self(self.0.cache_ttl(ttl))
        }
    }

    impl IntoFuture for Get {
        type Output     = Result<Option<String>, Error>;
        type IntoFuture = impl Future<Output = Self::Output>;

        fn into_future(self) -> Self::IntoFuture {
            async {
                self.0.text().await.map_err(Into::into)
            }
        }
    }
};

const _: (/* get json */) = {
    impl KV {
        pub fn get_as<T: for<'de> Deserialize<'de>>(&self, key: &str) -> GetAs<T> {
            GetAs(self.0.get(key), PhantomData)
        }
    }

    pub struct GetAs<T: for<'de> Deserialize<'de>>(
        GetOptionsBuilder,
        PhantomData<fn()->T>
    );

    impl<T: for<'de> Deserialize<'de>> GetAs<T> {
        pub fn cache_ttl(self, ttl: u64) -> Self {
            Self(self.0.cache_ttl(ttl), PhantomData)
        }
    }

    impl<T: for<'de> Deserialize<'de>> IntoFuture for GetAs<T> {
        type Output     = Result<Option<T>, Error>;
        type IntoFuture = impl Future<Output = Self::Output>;

        fn into_future(self) -> Self::IntoFuture {
            async {
                self.0.json().await.map_err(Into::into)
            }
        }
    }
};

const _: (/* put */) = {
    impl KV {
        pub fn put(&self, key: &str, value: impl ToRawKvValue) -> Put {
            Put(self.0.put(key, value).map_err(Into::into))
        }
    }

    pub struct Put(Result<PutOptionsBuilder, Error>);

    impl Put {
        pub fn expiration(self, timestamp: u64) -> Self {
            Self(self.0.map(|put| put.expiration(timestamp)))
        }

        pub fn expiration_ttl(self, ttl: u64) -> Self {
            Self(self.0.map(|put| put.expiration_ttl(ttl)))
        }

        pub fn metadata(self, metadata: impl serde::Serialize) -> Self {
            Self(match self.0 {
                Ok(put) => put.metadata(metadata).map_err(Into::into),
                Err(e)  => Err(e),
            })
        }
    }

    impl IntoFuture for Put {
        type Output     = Result<(), Error>;
        type IntoFuture = impl Future<Output = Self::Output>;

        fn into_future(self) -> Self::IntoFuture {
            async {
                self.0.map_err(Into::<Error>::into)?
                    .execute().await.map_err(Into::<Error>::into)
            }
        }
    }
};

const _: (/* list */) = {
    impl KV {
        pub fn list(&self) -> List {
            List(self.0.list())
        }
    }

    pub struct List(ListOptionsBuilder);

    impl List {
        pub fn cursor(self, cursor: impl Into<String>) -> Self {
            Self(self.0.cursor(cursor.into()))
        }

        pub fn prefix(self, prefix: impl Into<String>) -> Self {
            Self(self.0.prefix(prefix.into()))
        }

        pub fn limit(self, limit: u64) -> Self {
            Self(self.0.limit(limit))
        }
    }

    impl IntoFuture for List {
        type Output     = Result<ListResponse, Error>;
        type IntoFuture = impl Future<Output = Self::Output>;

        fn into_future(self) -> Self::IntoFuture {
            async {
                self.0.execute().await
                    .map_err(Into::into)
            }
        }
    }
};

const _: (/* delete */) = {
    impl KV {
        pub async fn delete(&self, key: &str) -> Result<(), Error> {
            self.0.delete(key).await.map_err(Into::into)
        }
    }
};
