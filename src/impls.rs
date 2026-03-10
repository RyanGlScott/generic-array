use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};

use super::{ArrayLength, GenericArray};

use crate::functional::*;
use crate::sequence::*;

impl<T: Default, N> Default for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    #[inline(always)]
    fn default() -> Self {
        Self::generate(|_| T::default())
    }
}

impl<T: Clone, N> Clone for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn clone(&self) -> GenericArray<T, N> {
        self.map(Clone::clone)
    }
}

impl<T: Copy, N> Copy for GenericArray<T, N>
where
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
}

impl<T: PartialEq, N> PartialEq for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}
impl<T: Eq, N> Eq for GenericArray<T, N> where N: ArrayLength<T> {}

impl<T: PartialOrd, N> PartialOrd for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn partial_cmp(&self, other: &GenericArray<T, N>) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.as_slice(), other.as_slice())
    }
}

impl<T: Ord, N> Ord for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn cmp(&self, other: &GenericArray<T, N>) -> Ordering {
        Ord::cmp(self.as_slice(), other.as_slice())
    }
}

impl<T: Debug, N> Debug for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self[..].fmt(fmt)
    }
}

impl<T, N> Borrow<[T]> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    #[inline(always)]
    fn borrow(&self) -> &[T] {
        &self[..]
    }
}

impl<T, N> BorrowMut<[T]> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T, N> AsRef<[T]> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        &self[..]
    }
}

impl<T, N> AsMut<[T]> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T: Hash, N> Hash for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Hash::hash(&self[..], state)
    }
}

impl<T, U, const N: usize> From<[T; N]> for GenericArray<T, U>
where
    U: ArrayLength<T, ArrayType = [T; N]>,
{
    #[inline]
    fn from(arr: [T; N]) -> GenericArray<T, U> {
        GenericArray { data: arr }
    }
}

impl<T, U, const N: usize> From<GenericArray<T, U>> for [T; N]
where
    U: ArrayLength<T, ArrayType = [T; N]>,
{
    #[inline]
    fn from(arr: GenericArray<T, U>) -> [T; N] {
        arr.data
    }
}

impl<'a, T, U, const N: usize> From<&'a [T; N]> for &'a GenericArray<T, U>
where
    U: ArrayLength<T, ArrayType = [T; N]>,
{
    #[inline]
    fn from(array_ref: &'a [T; N]) -> &'a GenericArray<T, U> {
        // SAFETY: `Self` is a `repr(transparent)` newtype for `[T; $len]`
        unsafe { &*array_ref.as_ptr().cast() }
    }
}

impl<'a, T, U, const N: usize> From<&'a GenericArray<T, U>> for &'a [T; N]
where
    U: ArrayLength<T, ArrayType = [T; N]>,
{
    #[inline]
    fn from(array_ref: &'a GenericArray<T, U>) -> &'a [T; N] {
        array_ref.as_ref()
    }
}

impl<'a, T, U, const N: usize> From<&'a mut [T; N]> for &'a mut GenericArray<T, U>
where
    U: ArrayLength<T, ArrayType = [T; N]>,
{
    #[inline]
    fn from(array_ref: &'a mut [T; N]) -> &'a mut GenericArray<T, U> {
        // SAFETY: `Self` is a `repr(transparent)` newtype for `[T; $len]`
        unsafe { &mut *array_ref.as_mut_ptr().cast() }
    }
}

impl<'a, T, U, const N: usize> From<&'a mut GenericArray<T, U>> for &'a mut [T; N]
where
    U: ArrayLength<T, ArrayType = [T; N]>,
{
    #[inline]
    fn from(array_ref: &'a mut GenericArray<T, U>) -> &'a mut [T; N] {
        array_ref.as_mut()
    }
}

impl<T, U, const N: usize> AsRef<[T; N]> for GenericArray<T, U>
where
    U: ArrayLength<T, ArrayType = [T; N]>,
{
    #[inline]
    fn as_ref(&self) -> &[T; N] {
        &self.data
    }
}

impl<T, U, const N: usize> AsMut<[T; N]> for GenericArray<T, U>
where
    U: ArrayLength<T, ArrayType = [T; N]>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.data
    }
}

macro_rules! impl_from {
    ($($n: expr => $ty: ty),*) => {
        $(
            unsafe impl<T> ArrayLength<T> for $ty {
                type ArrayType = [T; $n];
            }
        )*
    }
}

impl_from! {
    0 => ::typenum::U0,
    1 => ::typenum::U1,
    2 => ::typenum::U2,
    3 => ::typenum::U3,
    4 => ::typenum::U4,
    5 => ::typenum::U5,
    6 => ::typenum::U6,
    7 => ::typenum::U7,
    8 => ::typenum::U8,
    9 => ::typenum::U9,
    10 => ::typenum::U10,
    11 => ::typenum::U11,
    12 => ::typenum::U12,
    13 => ::typenum::U13,
    14 => ::typenum::U14,
    15 => ::typenum::U15,
    16 => ::typenum::U16,
    17 => ::typenum::U17,
    18 => ::typenum::U18,
    19 => ::typenum::U19,
    20 => ::typenum::U20,
    21 => ::typenum::U21,
    22 => ::typenum::U22,
    23 => ::typenum::U23,
    24 => ::typenum::U24,
    25 => ::typenum::U25,
    26 => ::typenum::U26,
    27 => ::typenum::U27,
    28 => ::typenum::U28,
    29 => ::typenum::U29,
    30 => ::typenum::U30,
    31 => ::typenum::U31,
    32 => ::typenum::U32,
    33 => ::typenum::U33,
    34 => ::typenum::U34,
    35 => ::typenum::U35,
    36 => ::typenum::U36,
    37 => ::typenum::U37,
    38 => ::typenum::U38,
    39 => ::typenum::U39,
    40 => ::typenum::U40,
    41 => ::typenum::U41,
    42 => ::typenum::U42,
    43 => ::typenum::U43,
    44 => ::typenum::U44,
    45 => ::typenum::U45,
    46 => ::typenum::U46,
    47 => ::typenum::U47,
    48 => ::typenum::U48,
    49 => ::typenum::U49,
    50 => ::typenum::U50,
    51 => ::typenum::U51,
    52 => ::typenum::U52,
    53 => ::typenum::U53,
    54 => ::typenum::U54,
    55 => ::typenum::U55,
    56 => ::typenum::U56,
    57 => ::typenum::U57,
    58 => ::typenum::U58,
    59 => ::typenum::U59,
    60 => ::typenum::U60,
    61 => ::typenum::U61,
    62 => ::typenum::U62,
    63 => ::typenum::U63,
    64 => ::typenum::U64,
    65 => ::typenum::U65,
    66 => ::typenum::U66,
    67 => ::typenum::U67,
    68 => ::typenum::U68,
    69 => ::typenum::U69,
    70 => ::typenum::U70,
    71 => ::typenum::U71,
    72 => ::typenum::U72,
    73 => ::typenum::U73,
    74 => ::typenum::U74,
    75 => ::typenum::U75,
    76 => ::typenum::U76,
    77 => ::typenum::U77,
    78 => ::typenum::U78,
    79 => ::typenum::U79,
    80 => ::typenum::U80,
    81 => ::typenum::U81,
    82 => ::typenum::U82,
    83 => ::typenum::U83,
    84 => ::typenum::U84,
    85 => ::typenum::U85,
    86 => ::typenum::U86,
    87 => ::typenum::U87,
    88 => ::typenum::U88,
    89 => ::typenum::U89,
    90 => ::typenum::U90,
    91 => ::typenum::U91,
    92 => ::typenum::U92,
    93 => ::typenum::U93,
    94 => ::typenum::U94,
    95 => ::typenum::U95,
    96 => ::typenum::U96,
    97 => ::typenum::U97,
    98 => ::typenum::U98,
    99 => ::typenum::U99,
    100 => ::typenum::U100,
    101 => ::typenum::U101,
    102 => ::typenum::U102,
    103 => ::typenum::U103,
    104 => ::typenum::U104,
    105 => ::typenum::U105,
    106 => ::typenum::U106,
    107 => ::typenum::U107,
    108 => ::typenum::U108,
    109 => ::typenum::U109,
    110 => ::typenum::U110,
    111 => ::typenum::U111,
    112 => ::typenum::U112,
    113 => ::typenum::U113,
    114 => ::typenum::U114,
    115 => ::typenum::U115,
    116 => ::typenum::U116,
    117 => ::typenum::U117,
    118 => ::typenum::U118,
    119 => ::typenum::U119,
    120 => ::typenum::U120,
    121 => ::typenum::U121,
    122 => ::typenum::U122,
    123 => ::typenum::U123,
    124 => ::typenum::U124,
    125 => ::typenum::U125,
    126 => ::typenum::U126,
    127 => ::typenum::U127,
    128 => ::typenum::U128,
    129 => ::typenum::U129,
    130 => ::typenum::U130,
    131 => ::typenum::U131,
    132 => ::typenum::U132,
    133 => ::typenum::U133,
    134 => ::typenum::U134,
    135 => ::typenum::U135,
    136 => ::typenum::U136,
    137 => ::typenum::U137,
    138 => ::typenum::U138,
    139 => ::typenum::U139,
    140 => ::typenum::U140,
    141 => ::typenum::U141,
    142 => ::typenum::U142,
    143 => ::typenum::U143,
    144 => ::typenum::U144,
    145 => ::typenum::U145,
    146 => ::typenum::U146,
    147 => ::typenum::U147,
    148 => ::typenum::U148,
    149 => ::typenum::U149,
    150 => ::typenum::U150,
    151 => ::typenum::U151,
    152 => ::typenum::U152,
    153 => ::typenum::U153,
    154 => ::typenum::U154,
    155 => ::typenum::U155,
    156 => ::typenum::U156,
    157 => ::typenum::U157,
    158 => ::typenum::U158,
    159 => ::typenum::U159,
    160 => ::typenum::U160,
    161 => ::typenum::U161,
    162 => ::typenum::U162,
    163 => ::typenum::U163,
    164 => ::typenum::U164,
    165 => ::typenum::U165,
    166 => ::typenum::U166,
    167 => ::typenum::U167,
    168 => ::typenum::U168,
    169 => ::typenum::U169,
    170 => ::typenum::U170,
    171 => ::typenum::U171,
    172 => ::typenum::U172,
    173 => ::typenum::U173,
    174 => ::typenum::U174,
    175 => ::typenum::U175,
    176 => ::typenum::U176,
    177 => ::typenum::U177,
    178 => ::typenum::U178,
    179 => ::typenum::U179,
    180 => ::typenum::U180,
    181 => ::typenum::U181,
    182 => ::typenum::U182,
    183 => ::typenum::U183,
    184 => ::typenum::U184,
    185 => ::typenum::U185,
    186 => ::typenum::U186,
    187 => ::typenum::U187,
    188 => ::typenum::U188,
    189 => ::typenum::U189,
    190 => ::typenum::U190,
    191 => ::typenum::U191,
    192 => ::typenum::U192,
    193 => ::typenum::U193,
    194 => ::typenum::U194,
    195 => ::typenum::U195,
    196 => ::typenum::U196,
    197 => ::typenum::U197,
    198 => ::typenum::U198,
    199 => ::typenum::U199,
    200 => ::typenum::U200,
    201 => ::typenum::U201,
    202 => ::typenum::U202,
    203 => ::typenum::U203,
    204 => ::typenum::U204,
    205 => ::typenum::U205,
    206 => ::typenum::U206,
    207 => ::typenum::U207,
    208 => ::typenum::U208,
    209 => ::typenum::U209,
    210 => ::typenum::U210,
    211 => ::typenum::U211,
    212 => ::typenum::U212,
    213 => ::typenum::U213,
    214 => ::typenum::U214,
    215 => ::typenum::U215,
    216 => ::typenum::U216,
    217 => ::typenum::U217,
    218 => ::typenum::U218,
    219 => ::typenum::U219,
    220 => ::typenum::U220,
    221 => ::typenum::U221,
    222 => ::typenum::U222,
    223 => ::typenum::U223,
    224 => ::typenum::U224,
    225 => ::typenum::U225,
    226 => ::typenum::U226,
    227 => ::typenum::U227,
    228 => ::typenum::U228,
    229 => ::typenum::U229,
    230 => ::typenum::U230,
    231 => ::typenum::U231,
    232 => ::typenum::U232,
    233 => ::typenum::U233,
    234 => ::typenum::U234,
    235 => ::typenum::U235,
    236 => ::typenum::U236,
    237 => ::typenum::U237,
    238 => ::typenum::U238,
    239 => ::typenum::U239,
    240 => ::typenum::U240,
    241 => ::typenum::U241,
    242 => ::typenum::U242,
    243 => ::typenum::U243,
    244 => ::typenum::U244,
    245 => ::typenum::U245,
    246 => ::typenum::U246,
    247 => ::typenum::U247,
    248 => ::typenum::U248,
    249 => ::typenum::U249,
    250 => ::typenum::U250,
    251 => ::typenum::U251,
    252 => ::typenum::U252,
    253 => ::typenum::U253,
    254 => ::typenum::U254,
    255 => ::typenum::U255,
    256 => ::typenum::U256,
    257 => ::typenum::U257,
    258 => ::typenum::U258,
    259 => ::typenum::U259,
    260 => ::typenum::U260,
    261 => ::typenum::U261,
    262 => ::typenum::U262,
    263 => ::typenum::U263,
    264 => ::typenum::U264,
    265 => ::typenum::U265,
    266 => ::typenum::U266,
    267 => ::typenum::U267,
    268 => ::typenum::U268,
    269 => ::typenum::U269,
    270 => ::typenum::U270,
    271 => ::typenum::U271,
    272 => ::typenum::U272,
    273 => ::typenum::U273,
    274 => ::typenum::U274,
    275 => ::typenum::U275,
    276 => ::typenum::U276,
    277 => ::typenum::U277,
    278 => ::typenum::U278,
    279 => ::typenum::U279,
    280 => ::typenum::U280,
    281 => ::typenum::U281,
    282 => ::typenum::U282,
    283 => ::typenum::U283,
    284 => ::typenum::U284,
    285 => ::typenum::U285,
    286 => ::typenum::U286,
    287 => ::typenum::U287,
    288 => ::typenum::U288,
    289 => ::typenum::U289,
    290 => ::typenum::U290,
    291 => ::typenum::U291,
    292 => ::typenum::U292,
    293 => ::typenum::U293,
    294 => ::typenum::U294,
    295 => ::typenum::U295,
    296 => ::typenum::U296,
    297 => ::typenum::U297,
    298 => ::typenum::U298,
    299 => ::typenum::U299,
    300 => ::typenum::U300,
    301 => ::typenum::U301,
    302 => ::typenum::U302,
    303 => ::typenum::U303,
    304 => ::typenum::U304,
    305 => ::typenum::U305,
    306 => ::typenum::U306,
    307 => ::typenum::U307,
    308 => ::typenum::U308,
    309 => ::typenum::U309,
    310 => ::typenum::U310,
    311 => ::typenum::U311,
    312 => ::typenum::U312,
    313 => ::typenum::U313,
    314 => ::typenum::U314,
    315 => ::typenum::U315,
    316 => ::typenum::U316,
    317 => ::typenum::U317,
    318 => ::typenum::U318,
    319 => ::typenum::U319,
    320 => ::typenum::U320,
    321 => ::typenum::U321,
    322 => ::typenum::U322,
    323 => ::typenum::U323,
    324 => ::typenum::U324,
    325 => ::typenum::U325,
    326 => ::typenum::U326,
    327 => ::typenum::U327,
    328 => ::typenum::U328,
    329 => ::typenum::U329,
    330 => ::typenum::U330,
    331 => ::typenum::U331,
    332 => ::typenum::U332,
    333 => ::typenum::U333,
    334 => ::typenum::U334,
    335 => ::typenum::U335,
    336 => ::typenum::U336,
    337 => ::typenum::U337,
    338 => ::typenum::U338,
    339 => ::typenum::U339,
    340 => ::typenum::U340,
    341 => ::typenum::U341,
    342 => ::typenum::U342,
    343 => ::typenum::U343,
    344 => ::typenum::U344,
    345 => ::typenum::U345,
    346 => ::typenum::U346,
    347 => ::typenum::U347,
    348 => ::typenum::U348,
    349 => ::typenum::U349,
    350 => ::typenum::U350,
    351 => ::typenum::U351,
    352 => ::typenum::U352,
    353 => ::typenum::U353,
    354 => ::typenum::U354,
    355 => ::typenum::U355,
    356 => ::typenum::U356,
    357 => ::typenum::U357,
    358 => ::typenum::U358,
    359 => ::typenum::U359,
    360 => ::typenum::U360,
    361 => ::typenum::U361,
    362 => ::typenum::U362,
    363 => ::typenum::U363,
    364 => ::typenum::U364,
    365 => ::typenum::U365,
    366 => ::typenum::U366,
    367 => ::typenum::U367,
    368 => ::typenum::U368,
    369 => ::typenum::U369,
    370 => ::typenum::U370,
    371 => ::typenum::U371,
    372 => ::typenum::U372,
    373 => ::typenum::U373,
    374 => ::typenum::U374,
    375 => ::typenum::U375,
    376 => ::typenum::U376,
    377 => ::typenum::U377,
    378 => ::typenum::U378,
    379 => ::typenum::U379,
    380 => ::typenum::U380,
    381 => ::typenum::U381,
    382 => ::typenum::U382,
    383 => ::typenum::U383,
    384 => ::typenum::U384,
    385 => ::typenum::U385,
    386 => ::typenum::U386,
    387 => ::typenum::U387,
    388 => ::typenum::U388,
    389 => ::typenum::U389,
    390 => ::typenum::U390,
    391 => ::typenum::U391,
    392 => ::typenum::U392,
    393 => ::typenum::U393,
    394 => ::typenum::U394,
    395 => ::typenum::U395,
    396 => ::typenum::U396,
    397 => ::typenum::U397,
    398 => ::typenum::U398,
    399 => ::typenum::U399,
    400 => ::typenum::U400,
    401 => ::typenum::U401,
    402 => ::typenum::U402,
    403 => ::typenum::U403,
    404 => ::typenum::U404,
    405 => ::typenum::U405,
    406 => ::typenum::U406,
    407 => ::typenum::U407,
    408 => ::typenum::U408,
    409 => ::typenum::U409,
    410 => ::typenum::U410,
    411 => ::typenum::U411,
    412 => ::typenum::U412,
    413 => ::typenum::U413,
    414 => ::typenum::U414,
    415 => ::typenum::U415,
    416 => ::typenum::U416,
    417 => ::typenum::U417,
    418 => ::typenum::U418,
    419 => ::typenum::U419,
    420 => ::typenum::U420,
    421 => ::typenum::U421,
    422 => ::typenum::U422,
    423 => ::typenum::U423,
    424 => ::typenum::U424,
    425 => ::typenum::U425,
    426 => ::typenum::U426,
    427 => ::typenum::U427,
    428 => ::typenum::U428,
    429 => ::typenum::U429,
    430 => ::typenum::U430,
    431 => ::typenum::U431,
    432 => ::typenum::U432,
    433 => ::typenum::U433,
    434 => ::typenum::U434,
    435 => ::typenum::U435,
    436 => ::typenum::U436,
    437 => ::typenum::U437,
    438 => ::typenum::U438,
    439 => ::typenum::U439,
    440 => ::typenum::U440,
    441 => ::typenum::U441,
    442 => ::typenum::U442,
    443 => ::typenum::U443,
    444 => ::typenum::U444,
    445 => ::typenum::U445,
    446 => ::typenum::U446,
    447 => ::typenum::U447,
    448 => ::typenum::U448,
    449 => ::typenum::U449,
    450 => ::typenum::U450,
    451 => ::typenum::U451,
    452 => ::typenum::U452,
    453 => ::typenum::U453,
    454 => ::typenum::U454,
    455 => ::typenum::U455,
    456 => ::typenum::U456,
    457 => ::typenum::U457,
    458 => ::typenum::U458,
    459 => ::typenum::U459,
    460 => ::typenum::U460,
    461 => ::typenum::U461,
    462 => ::typenum::U462,
    463 => ::typenum::U463,
    464 => ::typenum::U464,
    465 => ::typenum::U465,
    466 => ::typenum::U466,
    467 => ::typenum::U467,
    468 => ::typenum::U468,
    469 => ::typenum::U469,
    470 => ::typenum::U470,
    471 => ::typenum::U471,
    472 => ::typenum::U472,
    473 => ::typenum::U473,
    474 => ::typenum::U474,
    475 => ::typenum::U475,
    476 => ::typenum::U476,
    477 => ::typenum::U477,
    478 => ::typenum::U478,
    479 => ::typenum::U479,
    480 => ::typenum::U480,
    481 => ::typenum::U481,
    482 => ::typenum::U482,
    483 => ::typenum::U483,
    484 => ::typenum::U484,
    485 => ::typenum::U485,
    486 => ::typenum::U486,
    487 => ::typenum::U487,
    488 => ::typenum::U488,
    489 => ::typenum::U489,
    490 => ::typenum::U490,
    491 => ::typenum::U491,
    492 => ::typenum::U492,
    493 => ::typenum::U493,
    494 => ::typenum::U494,
    495 => ::typenum::U495,
    496 => ::typenum::U496,
    497 => ::typenum::U497,
    498 => ::typenum::U498,
    499 => ::typenum::U499,
    500 => ::typenum::U500,
    501 => ::typenum::U501,
    502 => ::typenum::U502,
    503 => ::typenum::U503,
    504 => ::typenum::U504,
    505 => ::typenum::U505,
    506 => ::typenum::U506,
    507 => ::typenum::U507,
    508 => ::typenum::U508,
    509 => ::typenum::U509,
    510 => ::typenum::U510,
    511 => ::typenum::U511,
    512 => ::typenum::U512,
    528 => ::typenum::U528,
    536 => ::typenum::U536,
    544 => ::typenum::U544,
    560 => ::typenum::U560,
    568 => ::typenum::U568,
    576 => ::typenum::U576,
    592 => ::typenum::U592,
    608 => ::typenum::U608,
    624 => ::typenum::U624,
    640 => ::typenum::U640,
    656 => ::typenum::U656,
    672 => ::typenum::U672,
    688 => ::typenum::U688,
    704 => ::typenum::U704,
    720 => ::typenum::U720,
    736 => ::typenum::U736,
    752 => ::typenum::U752,
    768 => ::typenum::U768,
    784 => ::typenum::U784,
    800 => ::typenum::U800,
    816 => ::typenum::U816,
    832 => ::typenum::U832,
    848 => ::typenum::U848,
    864 => ::typenum::U864,
    880 => ::typenum::U880,
    896 => ::typenum::U896,
    912 => ::typenum::U912,
    928 => ::typenum::U928,
    944 => ::typenum::U944,
    960 => ::typenum::U960,
    976 => ::typenum::U976,
    992 => ::typenum::U992,
    1008 => ::typenum::U1008,
    1024 => ::typenum::U1024,
    2048 => ::typenum::U2048,
    4096 => ::typenum::U4096,
    8192 => ::typenum::U8192
}
