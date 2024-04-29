use std::future::{Future, IntoFuture};
use serde::Serialize;
use worker::{Queue as RawQueue, QueueContentType, BatchMessageBuilder, MessageBuilder};
use worker::Error;

pub struct Queue(pub(crate) RawQueue);

const _: (/* send */) = {
    impl Queue {
        pub fn send<M: Serialize + Send>(& self, message: M) -> SendMessage<M> {
            SendMessage {
                queue:   self,
                message: MessageBuilder::new(message),
            }
        }
    }

    pub struct SendMessage<'q, M: Serialize + Send> {
        queue:   &'q Queue,
        message: MessageBuilder<M>,
    }

    impl<'q, M: Serialize + Send> SendMessage<'q, M> {
        pub fn delay_seconds(self, seconds: u32) -> Self {
            Self {
                message: self.message.delay_seconds(seconds),
                ..self
            }
        }

        pub fn as_text(self) -> Self {
            Self {
                message: self.message.content_type(QueueContentType::Text),
                ..self
            }
        }

        pub fn as_v8(self) -> Self {
            Self {
                message: self.message.content_type(QueueContentType::V8),
                ..self
            }
        }
    }

    impl<'q, M: Serialize + Send> IntoFuture for SendMessage<'q, M> {
        type Output     = Result<(), Error>;
        type IntoFuture = impl Future<Output = Self::Output>;

        fn into_future(self) -> Self::IntoFuture {
            self.queue.0.send(self.message.build())
        }
    }
};

const _: (/* send */) = {
    impl Queue {
        pub fn batch_send<M: Serialize + Send>(& self, messages: impl IntoIterator<Item = M>) -> BatchSendMessage<M> {
            BatchSendMessage {
                queue:    self,
                messages: BatchMessageBuilder::new().messages(messages),
            }
        }
    }

    pub struct BatchSendMessage<'q, M: Serialize + Send> {
        queue:   &'q Queue,
        messages: BatchMessageBuilder<M>,
    }

    impl<'q, M: Serialize + Send> BatchSendMessage<'q, M> {
        pub fn delay_seconds(self, seconds: u32) -> Self {
            Self {
                messages: self.messages.delay_seconds(seconds),
                ..self
            }
        }
    }

    impl<'q, M: Serialize + Send> IntoFuture for BatchSendMessage<'q, M> {
        type Output     = Result<(), Error>;
        type IntoFuture = impl Future<Output = Self::Output>;

        fn into_future(self) -> Self::IntoFuture {
            self.queue.0.send_batch(self.messages.build())
        }
    }
};
