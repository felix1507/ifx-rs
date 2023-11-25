#![cfg_attr(not(test), no_std)]
pub mod error;

use core::ops::{Add, Sub};
use error::*;

macro_rules! impl_unsigned_types {
    ($($t:ty),*) => {
        $(
            impl BaseType for $t {}
            impl UType for $t {}
        )*
    };
}

pub trait BaseType: Clone + Copy + PartialOrd + Sub<Output = Self> + Add<Output = Self> {}

pub trait UType: BaseType + Into<u32> + TryFrom<u32> {}

impl_unsigned_types!(u8, u16, u32);

#[derive(Debug, Clone)]
pub struct DpSearchResult {
    index: usize,
    ratio: u16,
}

impl DpSearchResult {
    pub fn search<T>(val: T, x_axis: &[T]) -> Result<Self>
    where
        T: UType,
    {
        let mut result = DpSearchResult { index: 0, ratio: 0 };
        let len = x_axis.len();

        if len < 1 {
            Err(Error::AxisToShort)
        } else if val <= x_axis[0] {
            result.index = 0;
            result.ratio = 0;

            Ok(result)
        } else if val >= x_axis[len - 1] {
            result.index = len - 1;
            result.ratio = 0;

            Ok(result)
        } else {
            let mut lower = 0;
            let mut upper = len - 1;
            let mut mid: usize;

            while upper - lower > 1 {
                mid = (lower + upper) / 2;

                if val < x_axis[mid] {
                    upper = mid;
                } else {
                    lower = mid;
                }
            }

            result.index = lower;

            let left = val.into() - x_axis[lower].into();
            let right = x_axis[lower + 1].into() - x_axis[lower].into();
            let ratio = (left * 0x10000) / right;

            match u16::try_from(ratio) {
                Ok(ratio) => {
                    result.ratio = ratio;
                    Ok(result)
                }
                Err(_) => Err(Error::UnconditionalState),
            }
        }
    }
}

pub fn ipo_cur_u<T>(dp_res: &DpSearchResult, y_axis: &[T]) -> Result<T>
where
    T: UType,
{
    if 0 == dp_res.ratio {
        Ok(y_axis[dp_res.index])
    } else {
        if y_axis[dp_res.index] <= y_axis[dp_res.index + 1] {
            let diff = (y_axis[dp_res.index + 1] - y_axis[dp_res.index]).into();
            let offset = (diff * u32::from(dp_res.ratio)) / 0x10000;

            match T::try_from(offset) {
                Ok(offset) => Ok(y_axis[dp_res.index] + offset),
                Err(_) => Err(Error::UnconditionalState),
            }
        } else {
            let diff = (y_axis[dp_res.index] - y_axis[dp_res.index + 1]).into();
            let offset = (diff * u32::from(dp_res.ratio)) / 0x10000;

            match T::try_from(offset) {
                Ok(offset) => Ok(y_axis[dp_res.index] - offset),
                Err(_) => Err(Error::UnconditionalState),
            }
        }
    }
}

pub fn ipo_map_u8() -> Result<u8> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datapoint_search_u_test() -> Result<()> {
        const X_AXIS_U8: [u8; 4] = [10, 20, 30, 40];
        const X_AXIS_U16: [u16; 4] = [100, 200, 300, 400];

        let result_u8 = DpSearchResult::search(5, &X_AXIS_U8)?;
        let result_u16 = DpSearchResult::search(50, &X_AXIS_U16)?;
        assert_eq!(result_u8.index, 0);
        assert_eq!(result_u8.ratio, 0);
        assert_eq!(result_u16.index, 0);
        assert_eq!(result_u16.ratio, 0);

        let result_u8 = DpSearchResult::search(35, &X_AXIS_U8)?;
        let result_u16 = DpSearchResult::search(350, &X_AXIS_U16)?;
        assert_eq!(result_u8.index, 2);
        assert_eq!(result_u8.ratio, 0x8000);
        assert_eq!(result_u16.index, 2);
        assert_eq!(result_u16.ratio, 0x8000);

        Ok(())
    }

    #[test]
    fn ipo_cur_u16_test() -> Result<()> {
        let y_axis_1: [u16; 4] = [100, 200, 300, 400];
        let y_axis_2: [u16; 4] = [400, 300, 200, 100];

        let mut dp_res = DpSearchResult { index: 1, ratio: 0 };
        let result = ipo_cur_u(&dp_res, &y_axis_1)?;
        assert_eq!(result, 200);

        dp_res.index = 2;
        dp_res.ratio = 0x8000;
        let result = ipo_cur_u(&dp_res, &y_axis_1)?;
        assert_eq!(result, 350);

        let result = ipo_cur_u(&dp_res, &y_axis_2)?;
        assert_eq!(result, 150);

        Ok(())
    }

    #[test]
    fn ipo_cur_u8_test() -> Result<()> {
        let y_axis_1: [u8; 4] = [10, 20, 30, 40];
        let y_axis_2: [u8; 4] = [40, 30, 20, 10];

        let mut dp_res = DpSearchResult { index: 1, ratio: 0 };
        let result = ipo_cur_u(&dp_res, &y_axis_1)?;
        assert_eq!(result, 20);

        dp_res.index = 2;
        dp_res.ratio = 0x8000;
        let result = ipo_cur_u(&dp_res, &y_axis_1)?;
        assert_eq!(result, 35);

        let result = ipo_cur_u(&dp_res, &y_axis_2)?;
        assert_eq!(result, 15);

        Ok(())
    }
}
