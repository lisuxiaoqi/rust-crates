use tt_call::{tt_call, tt_return};

macro_rules! is_lowercase_self {
    // Input token is `self`.
    {
        $caller:tt
        input = [{ self }]
    } => {
        tt_return! {
            $caller
            is = [{ true }]
        }
    };

    // Input token is anything other than `self`.
    {
        $caller:tt
        input = [{ $other:tt }]
    } => {
        tt_return! {
            $caller
            is = [{ false }]
        }
    };
}

macro_rules! is_lowercase_self_noret {
    // Input token is `self`.
    {
        $caller:tt
        input = [{ self }]
    } => {
             true
    };

    // Input token is anything other than `self`.
    {
        $caller:tt
        input = [{ $other:tt }]
    } => {
            false
        };
}

#[test]
fn basic() {
    let ss = tt_call! {
        macro = [{ is_lowercase_self }]
        input = [{ self }]
    };
    println!("{}", ss);

    let ss_noret = tt_call! {
        macro = [{ is_lowercase_self_noret }]
        input = [{ self }]
    };
    println!("{}", ss_noret);
}
