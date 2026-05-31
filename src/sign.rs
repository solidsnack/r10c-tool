macro_rules! sign {
    ($bool:expr) => { if $bool { 1.0 } else { -1.0 }}
}

pub(crate) use sign;
