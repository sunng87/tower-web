use super::{
    Chain,
    Collect,
    ForEach,
    FromBufStream,
    SizeHint,
};

use bytes::Buf;
use futures::{IntoFuture, Poll};

pub trait BufStream {
    type Item: Buf;
    type Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>;

    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }

    fn chain<T>(self, other: T) -> Chain<Self, T>
    where
        Self: Sized,
        T: BufStream<Error = Self::Error>,
    {
        Chain::new(self, other)
    }

    fn collect<T>(self) -> Collect<Self, T>
    where
        Self: Sized,
        T: FromBufStream,
    {
        Collect::new(self)
    }

    fn for_each<F, U>(self, f: F) -> ForEach<Self, F, U>
        where F: FnMut(Self::Item) -> U,
              U: IntoFuture<Item=(), Error = Self::Error>,
              Self: Sized
    {
        ForEach::new(self, f)
}
}
