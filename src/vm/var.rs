use std::ops;

use super::VarV;

impl Ord for VarV {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => a.cmp(b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl PartialOrd for VarV {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
impl ops::Not for VarV {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            VarV::Bool(b) => VarV::Bool(!b),
            VarV::Num(v) => VarV::Num(-v),
            VarV::Tuple(_) => todo!(),
        }
    }
}
impl ops::Add for VarV {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a + b),
            (VarV::Bool(a), VarV::Bool(b)) => VarV::Bool(a || b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl ops::Sub for VarV {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a - b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl ops::Mul for VarV {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a * b),
            (VarV::Bool(a), VarV::Bool(b)) => VarV::Bool(a && b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl ops::Div for VarV {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a / b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl ops::Rem for VarV {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a % b),
            _ => panic!("Type mismatch"),
        }
    }
    
}
impl ops::BitOr for VarV {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Bool(a), VarV::Bool(b)) => VarV::Bool(a || b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl ops::BitAnd for VarV {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Bool(a), VarV::Bool(b)) => VarV::Bool(a && b),
            _ => panic!("Type mismatch"),
        }
    }
}