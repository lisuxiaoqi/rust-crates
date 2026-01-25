#![allow(unused)]
use tt_call::{tt_call, tt_return};

macro_rules! pallet_a_produce{
    {
        $caller:tt
    }=>{
        tt_return!{
            $caller
            items = [{
                ItemA1,
                ItemA2,
            }]
        }
    };
}

macro_rules! mixer{
    {
        items = [{$($i:tt,)*}]
    }=>{
        #[derive(Debug, PartialEq)]
        enum Runtime{
            $($i),*
        }
    };
}

tt_call! {
    macro = [{pallet_a_produce}]
    ~~> mixer
}
#[test]
fn mix() {
    let r = Runtime::ItemA1;
    assert_eq!(r, Runtime::ItemA1);
}
