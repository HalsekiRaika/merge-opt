macro_rules! merge_opt {
    ( $m:ident, $( $x:tt ),+ ) => (
        #[allow(non_snake_case, clippy::too_many_arguments)]
        pub fn $m<$( $x ),+, F, O>($($x: Option<$x>),+, func: F) -> Option<O> 
            where F: Fn($( Option<$x> ),+) -> Option<O>
        {
            func($($x),+)
        }
    )
}

merge_opt!(merge_opt_i2,  I1, I2);
#[cfg(feature = "i3")]
merge_opt!(merge_opt_i3,  I1, I2, I3);
#[cfg(feature = "i4")]
merge_opt!(merge_opt_i4,  I1, I2, I3, I4);
#[cfg(feature = "i5")]
merge_opt!(merge_opt_i5,  I1, I2, I3, I4, I5);
#[cfg(feature = "i6")]
merge_opt!(merge_opt_i6,  I1, I2, I3, I4, I5, I6);
#[cfg(feature = "i7")]
merge_opt!(merge_opt_i7,  I1, I2, I3, I4, I5, I6, I7);
#[cfg(feature = "i8")]
merge_opt!(merge_opt_i8,  I1, I2, I3, I4, I5, I6, I7, I8);
#[cfg(feature = "i9")]
merge_opt!(merge_opt_i9,  I1, I2, I3, I4, I5, I6, I7, I8, I9);
#[cfg(feature = "i10")]
merge_opt!(merge_opt_i10, I1, I2, I3, I4, I5, I6, I7, I8, I9, I10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i2() {
        let s1 = Some(1);
        let s2 = Some(2);

        let merged = merge_opt_i2(s1, s2, |i1, i2| {
            (i1? + i2?).into()
        }).unwrap();

        assert_eq!(merged, 3);
    }

    #[test]
    fn test_i3() {
        let s1 = Some(1);
        let s2 = Some(2);
        let s3 = Some(3);

        let merged = merge_opt_i3(s1, s2, s3, |i1, i2, i3| {
            (i1? + i2? + i3?).into()
        }).unwrap();

        assert_eq!(merged, 6);
    }

    #[test]
    fn test_i4() {
        let s1 = Some(1);
        let s2 = Some(2);
        let s3 = Some(3);
        let s4 = Some(4);

        let merged = merge_opt_i4(s1, s2, s3, s4, |i1, i2, i3, i4| {
            (i1? + i2? + i3? + i4?).into()
        }).unwrap();

        assert_eq!(merged, 10);
    }

    #[test]
    fn test_i5() {
        let s1 = Some(1);
        let s2 = Some(2);
        let s3 = Some(3);
        let s4 = Some(4);
        let s5 = Some(5);

        let merged = merge_opt_i5(s1, s2, s3, s4, s5,|i1, i2, i3, i4, i5| {
            (i1? + i2? + i3? + i4? + i5?).into()
        }).unwrap();

        assert_eq!(merged, 15);
    }

    #[test]
    fn test_i6() {
        let s1 = Some(1);
        let s2 = Some(2);
        let s3 = Some(3);
        let s4 = Some(4);
        let s5 = Some(5);
        let s6 = Some(6);

        let merged = merge_opt_i6(s1, s2, s3, s4, s5, s6, |i1, i2, i3, i4, i5, i6| {
            (i1? + i2? + i3? + i4? + i5? + i6?).into()
        }).unwrap();

        assert_eq!(merged, 21);
    }

    #[test]
    fn test_i7() {
        let s1 = Some(1);
        let s2 = Some(2);
        let s3 = Some(3);
        let s4 = Some(4);
        let s5 = Some(5);
        let s6 = Some(6);
        let s7 = Some(7);

        let merged = merge_opt_i7(s1, s2, s3, s4, s5, s6, s7, |i1, i2, i3, i4, i5, i6, i7| {
            (i1? + i2? + i3? + i4? + i5? + i6? + i7?).into()
        }).unwrap();

        assert_eq!(merged, 28);
    }

    #[test]
    fn test_i8() {
        let s1 = Some(1);
        let s2 = Some(2);
        let s3 = Some(3);
        let s4 = Some(4);
        let s5 = Some(5);
        let s6 = Some(6);
        let s7 = Some(7);
        let s8 = Some(8);

        let merged = merge_opt_i8(s1, s2, s3, s4, s5, s6, s7, s8, |i1, i2, i3, i4, i5, i6, i7, i8| {
            (i1? + i2? + i3? + i4? + i5? + i6? + i7? + i8?).into()
        }).unwrap();

        assert_eq!(merged, 36);
    }

    #[test]
    fn test_i9() {
        let s1 = Some(1);
        let s2 = Some(2);
        let s3 = Some(3);
        let s4 = Some(4);
        let s5 = Some(5);
        let s6 = Some(6);
        let s7 = Some(7);
        let s8 = Some(8);
        let s9 = Some(9);

        let merged = merge_opt_i9(s1, s2, s3, s4, s5, s6, s7, s8, s9,|i1, i2, i3, i4, i5, i6, i7, i8, i9| {
            (i1? + i2? + i3? + i4? + i5? + i6? + i7? + i8? + i9?).into()
        }).unwrap();

        assert_eq!(merged, 45);
    }

    #[test]
    fn test_i10() {
        let s1 = Some(1);
        let s2 = Some(2);
        let s3 = Some(3);
        let s4 = Some(4);
        let s5 = Some(5);
        let s6 = Some(6);
        let s7 = Some(7);
        let s8 = Some(8);
        let s9 = Some(9);
        let s10 = Some(10);

        let merged = merge_opt_i10(s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, |i1, i2, i3, i4, i5, i6, i7, i8, i9, i10| {
            (i1? + i2? + i3? + i4? + i5? + i6? + i7? + i8? + i9? + i10?).into()
        }).unwrap();

        assert_eq!(merged, 55);
    }
}
