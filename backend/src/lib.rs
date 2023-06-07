use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::default();
}

#[ic_cdk_macros::query]
fn get() -> u32 {
    COUNTER.with(|counter| *counter.borrow())
}

#[ic_cdk_macros::update]
fn inc() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1)
}

#[ic_cdk_macros::update]
fn set(value: u32) {
    COUNTER.with(|counter| *counter.borrow_mut() = value)
}
