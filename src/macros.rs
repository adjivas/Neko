#[macro_export]
macro_rules! parse_number {
    ($table: expr) => ({
        use ::dynamic::SPEC_PRIORITY;
        use ::dynamic::SPEC_PRIORITY_NAME;
        if let Some(ref value) = $table.get(SPEC_PRIORITY_NAME) {
            value.as_integer()
        } else {
            Some(SPEC_PRIORITY)
        }
    });
}

#[macro_export]
macro_rules! parse_name {
    ($start: expr) => ({
        use std::ops::BitOr;
        if let (Some(middle), true) = (
            $start.rfind('/'),
            $start.ends_with(".git")
        ) {
            let right = $start.len()-4;
            $start.split_at(middle).0.rfind(|c: char|
                                           c.eq(&':').bitor(c.eq(&'/'))
            ).and_then(|left| unsafe {
                let all = $start.slice_unchecked(left+1, right).to_lowercase()
                                                               .replace("/", "@");
                let left = $start.slice_unchecked(left+1, middle).to_lowercase();
                let right = $start.slice_unchecked(middle+1, right).to_lowercase();
                if left.is_empty().bitor(right.is_empty()) {
                  None
                } else {
                  Some((all, left, right))
                }
            })
        } else {
            None
        }
    });
}
