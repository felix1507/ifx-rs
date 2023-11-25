pub mod error;

use core::ops::Sub;
use error::*;

macro_rules! impl_sup_types {
    ($($t:ty),*) => {
        $(
            impl UType for $t {}
        )*
    };
}

pub trait UType: Clone + Copy + PartialOrd + Into<u32> + Sub { }

impl_sup_types!(u8, u16);

#[derive(Debug, Clone)]
pub struct DpSearchResult {
    index: usize,
    ratio: u16,
}

impl DpSearchResult {
    pub fn search_u8(val: u8, x_axis: &[u8]) -> Result<Self> {
        let mut result = DpSearchResult { index: 0, ratio: 0};
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
        }
        else {
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

            let left = u32::from(val) - u32::from(x_axis[lower]);
            let right = u32::from(x_axis[lower + 1]) - u32::from(x_axis[lower]);
            let ratio = (left * 0x10000) / right;

            match u16::try_from(ratio) {
                Ok(res) => result.ratio = res,
                Err(_) => return Err(Error::UnconditionalState),
            }

            Ok(result)
        }
    }

    pub fn search<T>(val: T, x_axis: &[T]) -> Result<Self>
    where
        T: UType
    {
        let mut result = DpSearchResult { index: 0, ratio: 0};
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
                Err(_) => Err(Error::UnconditionalState)
            }
        }
    }
}

pub fn rs_ipo_cur_u8(dp_res: &DpSearchResult, y_axis: &[u8]) -> Result<u8> {
    if 0 == dp_res.ratio {
        Ok(y_axis[dp_res.index])
    } else {
        if y_axis[dp_res.index] <= y_axis[dp_res.index + 1] {
            let diff = u32::from(y_axis[dp_res.index + 1] - y_axis[dp_res.index]);
            let offset = (diff * u32::from(dp_res.ratio)) / 0x10000;
            
            match u8::try_from(offset) {
                Ok(offset) => Ok(y_axis[dp_res.index] + offset),
                Err(_) => Err(Error::UnconditionalState),
            }
        } else {
            let diff = u32::from(y_axis[dp_res.index] - y_axis[dp_res.index + 1]);
            let offset = (diff * u32::from(dp_res.ratio)) / 0x10000;
            
            match u8::try_from(offset) {
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
    fn dp_search_test() -> Result<()> {
        let axis = [10, 20, 30, 40];

        let rs_result = DpSearchResult::search_u8(5, &axis)?;
        assert_eq!(rs_result.index, 0);
        assert_eq!(rs_result.ratio, 0);

        let rs_result = DpSearchResult::search_u8(30, &axis)?;
        assert_eq!(rs_result.index, 2);
        assert_eq!(rs_result.ratio, 0);

        let rs_result = DpSearchResult::search_u8(25, &axis)?;
        assert_eq!(rs_result.index, 1);
        assert_eq!(rs_result.ratio, 0x8000);

        Ok(())
    }

    #[test]
    fn test_ipo_cur() -> Result<()> {
        let y_axis = [10, 20, 30, 40];

        let rs_dp_res = DpSearchResult{ index: 1, ratio: 0x8000};
        let rs_ret_val = rs_ipo_cur_u8(&rs_dp_res, &y_axis)?;
        assert_eq!(rs_ret_val, 25);

        let y_axis = [0, 100];

        let rs_dp_res = DpSearchResult{ index: 0, ratio: 0x4000};
        let rs_ret_val = rs_ipo_cur_u8(&rs_dp_res, &y_axis)?;
        assert_eq!(rs_ret_val, 25);

        let rs_dp_res = DpSearchResult{ index: 0, ratio: 0x8000};
        let rs_ret_val = rs_ipo_cur_u8(&rs_dp_res, &y_axis)?;
        assert_eq!(rs_ret_val, 50);

        let rs_dp_res = DpSearchResult{ index: 0, ratio: 0xC000};
        let rs_ret_val = rs_ipo_cur_u8(&rs_dp_res, &y_axis)?;
        assert_eq!(rs_ret_val, 75);

        Ok(())
    }

    #[test]
    fn test_gen_usearch() -> Result<()> {
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

}
