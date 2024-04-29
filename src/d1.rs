use std::future::{Future, IntoFuture};
use worker::d1::{D1Database, D1Result, D1PreparedStatement};
use worker::wasm_bindgen::JsValue;
use worker::worker_sys::D1ExecResult;
use worker::Error;
use serde::Deserialize;

pub struct D1(pub(crate) D1Database);

const _: (/* prepared statements */) = {
    impl D1 {
        pub fn query(&self, query: impl Into<String>) -> Statement {
            Statement(Ok(self.0.prepare(query)))
        }

        pub async fn batch<T: for<'row> Deserialize<'row>>(&self,
            statements: impl IntoIterator<Item = Statement>
        ) -> Result<BatchResult, Error> {
            let statements = statements.into_iter().map(|stmt| stmt.0)
                .collect::<Result<Vec<_>, _>>()?;
            self.0.batch(statements).await
                .map(BatchResult)
        }
    }

    pub struct BatchResult(Vec<D1Result>);

    impl Iterator for BatchResult {
        type Item = D1Result;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }
    impl<'b> Iterator for &'b BatchResult {
        type Item = &'b D1Result;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.iter().next()
        }
    }

    pub struct Statement(Result<D1PreparedStatement, Error>);

    impl Statement {
        pub fn bind(self, arguments: impl Arguments) -> Self {
            arguments.bind_to(self)
        }
    }

    pub trait Arguments {
        fn bind_to(self, statement: Statement) -> Statement;
    }
    const _: () = {
        macro_rules! single_argument {
            ($t:ty) => {
                impl Arguments for $t {
                    fn bind_to(self, statement: Statement) -> Statement {
                        match statement.0 {
                            Ok(stmt) => Statement(stmt.bind(&[Into::<JsValue>::into(self)])),
                            Err(err) => Statement(Err(err))
                        }
                    }
                }
            };
        }
        single_argument!(&str);
        single_argument!(String);
        single_argument!(u8);
        single_argument!(u16);
        single_argument!(u32);
        single_argument!(u64);
        single_argument!(u128);
        single_argument!(usize);
        single_argument!(i8);
        single_argument!(i16);
        single_argument!(i32);
        single_argument!(i64);
        single_argument!(i128);
        single_argument!(isize);
        impl<J: Into<JsValue>> Arguments for Option<J> {
            fn bind_to(self, statement: Statement) -> Statement {
                match statement.0 {
                    Ok(stmt) => Statement(stmt.bind(&[Into::<JsValue>::into(self.map(Into::into))])),
                    Err(err) => Statement(Err(err))
                }
            }
        }

        macro_rules! tuple_arguments {
            ($( $t:ident ),*) => {
                #[allow(non_snake_case)]
                impl<$( $t: Into<JsValue> ),*> Arguments for ( $( $t, )* ) {
                    fn bind_to(self, statement: Statement) -> Statement {
                        let ( $( $t, )* ) = self;
                        match statement.0 {
                            Ok(stmt) => Statement(stmt.bind(&[ $( $t.into() ),*])),
                            Err(err) => Statement(Err(err))
                        }
                    }
                }
            };
        }
        tuple_arguments!(A1);
        tuple_arguments!(A1, A2);
        tuple_arguments!(A1, A2, A3);
        tuple_arguments!(A1, A2, A3, A4);
        tuple_arguments!(A1, A2, A3, A4, A5);
        tuple_arguments!(A1, A2, A3, A4, A5, A6);
        tuple_arguments!(A1, A2, A3, A4, A5, A6, A7);
        tuple_arguments!(A1, A2, A3, A4, A5, A6, A7, A8);
    };

    impl Statement {
        pub async fn all<T: for<'row> Deserialize<'row>>(self) -> Result<Vec<T>, Error> {
            self.0?.all().await?
                .results()
        }

        pub async fn first<T: for<'row> Deserialize<'row>>(self) -> Result<Option<T>, Error> {
            self.0?.first(None).await
        }
    }

    impl IntoFuture for Statement {
        type Output     = Result<(), Error>;
        type IntoFuture = impl Future<Output = Self::Output>;

        fn into_future(self) -> Self::IntoFuture {
            async {
                self.0?.run().await?;
                Ok(())
            }
        }
    }
};

const _: (/* raw exec */) = {
    impl D1 {
        /// SAFETY: `query` has NO problem to be executed as written
        pub async unsafe fn exec(&self, query: &str) -> Result<ExecResult, Error> {
            self.0.exec(query).await.map(ExecResult)
        }
    }

    pub struct ExecResult(D1ExecResult);

    impl ExecResult {
        pub fn count(&self) -> Option<u32> {
            self.0.count()
        }

        pub fn duration(&self) -> Option<f64> {
            self.0.duration()
        }
    }
};
