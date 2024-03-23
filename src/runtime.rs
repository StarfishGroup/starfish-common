use std::future::Future;
use tokio::runtime::{Builder, Handle, Runtime};

static mut INS: Option<Runtime> = None;

pub fn init() {
    let rt: Runtime = Builder::new_multi_thread()
        .thread_name("Starfish")
        .enable_all()
        .build()
        .unwrap();
    unsafe { INS = Some(rt) }
}

pub fn get() -> &'static Runtime {
    unsafe { INS.as_ref().unwrap() }
}

pub fn blocking<F: Future>(future: F) -> F::Output {
    tokio::task::block_in_place(move || Handle::current().block_on(future))
}
