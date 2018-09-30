#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum DObjType {
    D_CAR_DYNAMICS  = 0,
    D_NET           = 1,
    D_JOINT         = 2,
    D_ANOTHER_JOINT = 3,
    D_CAMERA        = 4,
    D_VERTEX        = 5,
    D_FACE          = 6,
    D_PLANE         = 7,
    D_BONE          = 8,
    D_MATERIAL      = 9,
    D_SHAPE         = 10,
    D_GADGET        = 11,
    D_LABEL         = 12,
    D_VIEW          = 13,
    D_ANIMATOR      = 14,
    D_DIFF_GRP      = 15,   // different group type
    D_PARTICLE      = 16,
    D_LIGHT         = 17,
    D_GROUP         = 18,
}

impl From<u32> for DObjType {
    fn from(n: u32) -> Self {
        match n {
            0  => DObjType::D_CAR_DYNAMICS,
            1  => DObjType::D_NET,
            2  => DObjType::D_JOINT,
            3  => DObjType::D_ANOTHER_JOINT,
            4  => DObjType::D_CAMERA,
            5  => DObjType::D_VERTEX,
            6  => DObjType::D_FACE,
            7  => DObjType::D_PLANE,
            8  => DObjType::D_BONE,
            9  => DObjType::D_MATERIAL,
            10 => DObjType::D_SHAPE,
            11 => DObjType::D_GADGET,
            12 => DObjType::D_LABEL,
            13 => DObjType::D_VIEW,
            14 => DObjType::D_ANIMATOR,
            15 => DObjType::D_DIFF_GRP,
            16 => DObjType::D_PARTICLE,
            17 => DObjType::D_LIGHT,
            18 => DObjType::D_GROUP,
            u @ _ => panic!("Unknown DynObj Type {}", u), 
        }
    }
}