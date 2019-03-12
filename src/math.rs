#[cfg(feature = "no_std")]
use core::intrinsics;

#[cfg(not(feature = "no_std"))]
pub fn ceil(n: f32) -> f32 {
    n.ceil()
}

#[cfg(feature = "no_std")]
pub fn ceil(n: f32) -> f32 {
    unsafe { intrinsics::ceilf32(n) }
}

#[cfg(not(feature = "no_std"))]
pub fn floor(n: f32) -> f32 {
    n.floor()
}

#[cfg(feature = "no_std")]
pub fn floor(n: f32) -> f32 {
    unsafe { intrinsics::floorf32(n) }
}


#[cfg(not(feature = "no_std"))]
pub fn sqrt(n: f32) -> f32 {
    n.sqrt()
}

#[cfg(feature = "no_std")]
pub fn sqrt(n: f32) -> f32 {
    unsafe { intrinsics::sqrtf32(n) }
}
