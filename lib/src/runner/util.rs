use std::mem;

pub unsafe fn prolong_lifetime<'a, T: ?Sized>(t: &'a T) -> &'static T {
    mem::transmute::<&T, &'static T>(t)
}
