macro_rules! sign {
    ($bool:expr) => {
        (if $bool { 1 } else { -1 } as i8).into()
    };
}

pub(crate) use sign;
