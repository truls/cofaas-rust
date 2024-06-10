use std::any::Any;

pub use async_trait::async_trait;

pub struct Request<T> {
    value: T,
}

pub struct Response<T> {
    value: T,
}

pub struct Status {
    msg: String
}

impl Status {
    pub fn unknown(message: impl Into<String>) -> Status{
        Status{msg: message.into()}
    }

    pub fn message(&self) -> &str {
        self.msg.as_str()
    }
}


impl<T> Request<T>
where
    T: Clone,
{
    pub fn new(val: T) -> Request<T> {
        Request { value: val }
    }

    pub fn into_inner(&self) -> T {
        self.value.clone()
    }
}

impl<T> Response<T>
where
    T: Clone,
{
    pub fn new(val: T) -> Response<T> {
        Response { value: val }
    }

    pub fn into_inner(&self) -> T {
        self.value.clone()
    }
}

pub trait BoxedAnyImpl {
    fn from_inner(&mut self) -> Box<dyn Any>;
}

pub mod transport {
    use std::{any::Any, fmt::Error, net::SocketAddr};

    use super::BoxedAnyImpl;

    pub struct Server {}

    impl Server {
        pub fn builder() -> Self {
            Self {}
        }

        pub fn add_service(self, f: fn(Box<dyn Any>), svc: &mut dyn BoxedAnyImpl) -> Self {
            //let b = svc.from_inner();
            f(svc.from_inner());
            self
        }

        pub async fn serve(self, _addr: SocketAddr) -> Result<(), Error> {
            Ok(())
        }
    }
}

// pub trait IntoRequest<T> {
//     /// Wrap the input message `T` in a `tonic::Request`
//     fn into_request(self) -> Request<T>;
// }

// impl<T> IntoRequest<T> for T {
//     fn into_request(self) -> Request<Self> {
//         Request::new(self)
//     }
// }

// impl<T> IntoRequest<T> for Request<T> {
//     fn into_request(self) -> Request<T> {
//         self
//     }
// }
