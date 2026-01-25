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

macro_rules! pallet_b_produce{
    {
        $caller:tt
    }=>{
        tt_return!{
            $caller
            items = [{
                ItemB1,
            }]
        }
    };
}

//tt muncher
macro_rules! mixer{
    {
        $caller:tt
        parts = [{ }]
        pre = [{ $($pre:tt,)* }]
        items = [{ $($new:tt,)* }]
    }=>{
        #[derive(PartialEq, Debug)]
        enum Runtime{
            $($pre,)*
            $($new,)*
        }
    };

    {
        $caller:tt
        parts = [{ $first:tt $($rest:tt)* }]
        pre = [{ $($pre:tt,)* }]
        items = [{ $($new:tt,)* }]
    }=>{
        tt_call!{
            macro = [{ $first }]
            ~~>mixer!{
                $caller
                parts = [{ $($rest)* }]
                pre = [{
                    $($pre,)*
                    $($new,)*
                }]
            }
        }
    };
}

tt_call! {
    macro = [{ mixer }]
    parts = [{pallet_a_produce pallet_b_produce}]
    pre = [{}]
    items = [{}]
}

#[test]
fn test() {
    let a = Runtime::ItemA1;
    let b = Runtime::ItemB1;
    assert_ne!(a, b);
}
