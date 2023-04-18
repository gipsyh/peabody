pub fn and(l: Option<bool>, r: Option<bool>) -> Option<bool> {
    match (l, r) {
        (Some(true), Some(true)) => Some(true),
        (Some(false), _) => Some(false),
        (_, Some(false)) => Some(false),
        _ => None,
    }
}

pub fn or(l: Option<bool>, r: Option<bool>) -> Option<bool> {
    match (l, r) {
        (Some(false), Some(false)) => Some(false),
        (Some(true), _) => Some(true),
        (_, Some(true)) => Some(true),
        _ => None,
    }
}

pub fn imp(l: Option<bool>, r: Option<bool>) -> Option<bool> {
    match (l, r) {
        (Some(true), Some(false)) => Some(false),
        (Some(false), _) => Some(true),
        (_, Some(true)) => Some(true),
        _ => None,
    }
}

pub fn iff(l: Option<bool>, r: Option<bool>) -> Option<bool> {
    match (l, r) {
        (Some(l), Some(r)) => Some(l == r),
        _ => None,
    }
}

pub fn xor(l: Option<bool>, r: Option<bool>) -> Option<bool> {
    match (l, r) {
        (Some(l), Some(r)) => Some(l ^ r),
        _ => None,
    }
}

pub fn and_not(l: Option<bool>, r: Option<bool>) -> Option<bool> {
    match (l, r) {
        (Some(false), _) => Some(false),
        (_, Some(true)) => Some(false),
        (Some(true), Some(false)) => Some(true),
        _ => None,
    }
}