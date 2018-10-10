use std::{fmt};
use std::slice::Iter;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PtrParam {
    PARAM_OBJ_VTX = 1,
    PARAM_CHAR_PTR = 5,
}

impl PtrParam {
    const TOTAL: usize = 2;

    pub fn iter() -> Iter<'static, (PtrParam, u32)> {
        static VARIANTS: [(PtrParam, u32); PtrParam::TOTAL] = [
            (PtrParam::PARAM_OBJ_VTX, 1),
            (PtrParam::PARAM_CHAR_PTR, 5),
        ];
        VARIANTS.iter()
    }
}

impl fmt::Display for PtrParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u32> for PtrParam {
    fn from(n: u32) -> Self {
        match n {
            1  => PtrParam::PARAM_OBJ_VTX,
            5  => PtrParam::PARAM_CHAR_PTR,
            u @ _ => panic!("Unknown SetParamPtr Parameter {}", u), 
        }
    }
}