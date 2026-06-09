macro_rules! sign {
    ($bool:expr) => {
        if $bool { 1 } else { -1 }
    };
}

pub(crate) use sign;
