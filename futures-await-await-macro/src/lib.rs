/// Ye Olde Await Macro
///
/// Basically a translation of polling to yielding. This crate's macro is
/// reexported in the `futures_await` crate, you should not use this crate
/// specifically. If I knew how to define this macro in the `futures_await`
/// crate I would. Ideally this crate would not exist.

// TODO: how to define this in the `futures_await` crate but have it still
// importable via `futurses_await::prelude::await`?

#[macro_export]
macro_rules! await {
    ($e:expr) => ({
        let mut future = $e;
        loop {
            let ctx = ::futures::__rt::get_ctx();
            match ::futures::__rt::Future::poll(&mut future, unsafe { &mut *ctx }) {
                ::futures::__rt::std::result::Result::Ok(::futures::__rt::Async::Ready(e)) => {
                    break ::futures::__rt::std::result::Result::Ok(e)
                }
                ::futures::__rt::std::result::Result::Ok(::futures::__rt::Async::Pending) => {}
                ::futures::__rt::std::result::Result::Err(e) => {
                    break ::futures::__rt::std::result::Result::Err(e)
                }
            }
            yield ::futures::__rt::Async::Pending
        }
    })
}

///
/// Await an item from the stream
/// Basically it does same as `await` macro, but for streams
///

#[macro_export]
macro_rules! await_item {
    ($e:expr) => ({
        loop {
            let ctx = ::futures::__rt::get_ctx();
            match ::futures::Stream::poll(&mut $e, unsafe { &mut *ctx }) {
                ::futures::__rt::std::result::Result::Ok(::futures::__rt::Async::Ready(e)) => {
                    break ::futures::__rt::std::result::Result::Ok(e)
                }
                ::futures::__rt::std::result::Result::Ok(::futures::__rt::Async::Pending) => {}
                ::futures::__rt::std::result::Result::Err(e) => {
                    break ::futures::__rt::std::result::Result::Err(e)
                }
            }

            yield ::futures::__rt::Async::Pending
        }
    })
}

// TODO: This macro needs to use an extra temporary variable because of
// rust-lang/rust#44197, once that's fixed this should just use $e directly
// inside the yield expression
#[macro_export]
macro_rules! stream_yield {
    ($e:expr) => ({
        let e = $e;
        yield ::futures::__rt::Async::Ready(e)
    })
}
