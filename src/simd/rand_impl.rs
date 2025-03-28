use crate::simd::*;

// Given two token streams in the format `ignore_snd!([first_token_tree], [second])` will simply
// expand to the first one. This is useful in order to allow the repetition of some statement
// according to some repetition variable, without using the repetition variables.
macro_rules! ignore_snd (
    ([$($fst: tt)*], [$($snd: tt)*]) => ($($fst)*)
);

macro_rules! impl_rand_auto_simd (
    ($($t: ty, $($i: ident),*;)*) => ($(
        impl rand::distributions::Distribution<AutoSimd<$t>> for rand::distributions::Standard {
            #[inline(always)]
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> AutoSimd<$t> {
                AutoSimd::<$t>::new($(
                    ignore_snd!([self.sample(rng)], [$i])
                ),*)
            }
        }
    )*)
);

impl_rand_auto_simd!(
    [f32;  2], _0, _1;
    [f32;  4], _0, _1, _2, _3;
    [f32;  8], _0, _1, _2, _3, _4, _5, _6, _7;
    [f32; 16], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [f64;  2], _0, _1;
    [f64;  4], _0, _1, _2, _3;
    [f64;  8], _0, _1, _2, _3, _4, _5, _6, _7;
    [i128; 1], _0;
    [i128; 2], _0, _1;
    [i128; 4], _0, _1, _2, _3;
    [i16; 2], _0, _1;
    [i16; 4], _0, _1, _2, _3;
    [i16; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [i16; 16], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [i16; 32], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    [i32; 2], _0, _1;
    [i32; 4], _0, _1, _2, _3;
    [i32; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [i32; 16], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [i64; 2], _0, _1;
    [i64; 4], _0, _1, _2, _3;
    [i64; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [i8; 2], _0, _1;
    [i8; 4], _0, _1, _2, _3;
    [i8; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [i8; 16], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [i8; 32], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    // [i8; 64], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    [isize; 2], _0, _1;
    [isize; 4], _0, _1, _2, _3;
    [isize; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [u128; 1], _0;
    [u128; 2], _0, _1;
    [u128; 4], _0, _1, _2, _3;
    [u16; 2], _0, _1;
    [u16; 4], _0, _1, _2, _3;
    [u16; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [u16; 16], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [u16; 32], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    [u32; 2], _0, _1;
    [u32; 4], _0, _1, _2, _3;
    [u32; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [u32; 16], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [u64; 2], _0, _1;
    [u64; 4], _0, _1, _2, _3;
    [u64; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [u8; 2], _0, _1;
    [u8; 4], _0, _1, _2, _3;
    [u8; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [u8; 16], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [u8; 32], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    // [u8; 64], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    [usize; 2], _0, _1;
    [usize; 4], _0, _1, _2, _3;
    [usize; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [bool; 1], _0;
    [bool; 2], _0, _1;
    [bool; 4], _0, _1, _2, _3;
    [bool; 8], _0, _1, _2, _3, _4, _5, _6, _7;
    [bool; 16], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    [bool; 32], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    // [bool; 64], _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
);

#[cfg(feature = "wide")]
macro_rules! impl_rand_wide_simd (
    ($($t: ident, $wrapped: ty, $arr: ty;)*) => ($(
        impl rand::distributions::Distribution<$t> for rand::distributions::Standard {
            #[inline(always)]
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $t {
                $t(<$wrapped as From<$arr>>::from(self.sample(rng)))
            }
        }
    )*)
);

#[cfg(feature = "wide")]
impl_rand_wide_simd!(
    WideF32x4, wide::f32x4, [f32; 4];
    WideF64x4, wide::f64x4, [f64; 4];
    WideF32x8, wide::f32x8, [f32; 8];
);

#[cfg(feature = "portable_simd")]
macro_rules! impl_rand_portable_simd (
    ($($wrapped: ty, $($i: ident),*;)*) => ($(
        impl rand::distributions::Distribution<$wrapped> for rand::distributions::Standard {
            #[inline(always)]
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $wrapped {
                <$wrapped>::new($(
                    ignore_snd!([self.sample(rng)], [$i])
                ),*)
            }
        }
    )*)
);

#[cfg(feature = "portable_simd")]
impl_rand_portable_simd!(
    f32x2, _0, _1;
    f32x4, _0, _1, _2, _3;
    f32x8, _0, _1, _2, _3, _4, _5, _6, _7;
    f32x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    f64x2, _0, _1;
    f64x4, _0, _1, _2, _3;
    f64x8, _0, _1, _2, _3, _4, _5, _6, _7;
    i16x2, _0, _1;
    i16x4, _0, _1, _2, _3;
    i16x8, _0, _1, _2, _3, _4, _5, _6, _7;
    i16x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    i16x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    i32x2, _0, _1;
    i32x4, _0, _1, _2, _3;
    i32x8, _0, _1, _2, _3, _4, _5, _6, _7;
    i32x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    i64x2, _0, _1;
    i64x4, _0, _1, _2, _3;
    i64x8, _0, _1, _2, _3, _4, _5, _6, _7;
    i8x2, _0, _1;
    i8x4, _0, _1, _2, _3;
    i8x8, _0, _1, _2, _3, _4, _5, _6, _7;
    i8x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    i8x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    i8x64, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    isizex2, _0, _1;
    isizex4, _0, _1, _2, _3;
    isizex8, _0, _1, _2, _3, _4, _5, _6, _7;
    u16x2, _0, _1;
    u16x4, _0, _1, _2, _3;
    u16x8, _0, _1, _2, _3, _4, _5, _6, _7;
    u16x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    u16x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    u32x2, _0, _1;
    u32x4, _0, _1, _2, _3;
    u32x8, _0, _1, _2, _3, _4, _5, _6, _7;
    u32x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    u64x2, _0, _1;
    u64x4, _0, _1, _2, _3;
    u64x8, _0, _1, _2, _3, _4, _5, _6, _7;
    u8x2, _0, _1;
    u8x4, _0, _1, _2, _3;
    u8x8, _0, _1, _2, _3, _4, _5, _6, _7;
    u8x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    u8x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    u8x64, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    usizex2, _0, _1;
    usizex4, _0, _1, _2, _3;
    usizex8, _0, _1, _2, _3, _4, _5, _6, _7;
    mask16x2, _0, _1;
    mask16x4, _0, _1, _2, _3;
    mask16x8, _0, _1, _2, _3, _4, _5, _6, _7;
    mask16x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    mask16x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    mask32x2, _0, _1;
    mask32x4, _0, _1, _2, _3;
    mask32x8, _0, _1, _2, _3, _4, _5, _6, _7;
    mask32x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    mask64x2, _0, _1;
    mask64x4, _0, _1, _2, _3;
    mask64x8, _0, _1, _2, _3, _4, _5, _6, _7;
    mask8x2, _0, _1;
    mask8x4, _0, _1, _2, _3;
    mask8x8, _0, _1, _2, _3, _4, _5, _6, _7;
    mask8x16, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15;
    mask8x32, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31;
    mask8x64, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59, _60, _61, _62, _63;
    masksizex2, _0, _1;
    masksizex4, _0, _1, _2, _3;
    masksizex8, _0, _1, _2, _3, _4, _5, _6, _7;
);
