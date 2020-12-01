////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use super::*;

impl Not for Position {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            x: self.x.not(),
            y: self.y.not(),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl Mul for Position {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl MulAssign for Position {
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl Div for Position {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl DivAssign for Position {
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl Rem for Position {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl RemAssign for Position {
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl Shl for Position {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl ShlAssign for Position {
    fn shl_assign(&mut self, rhs: Self) {
        self.x.shl_assign(rhs.x);
        self.y.shl_assign(rhs.y);
    }
}

impl Shr for Position {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl ShrAssign for Position {
    fn shr_assign(&mut self, rhs: Self) {
        self.x.shr_assign(rhs.x);
        self.y.shr_assign(rhs.y);
    }
}

impl BitAnd for Position {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
        }
    }
}

impl BitAndAssign for Position {
    fn bitand_assign(&mut self, rhs: Self) {
        self.x.bitand_assign(rhs.x);
        self.y.bitand_assign(rhs.y);
    }
}

impl BitOr for Position {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
        }
    }
}

impl BitOrAssign for Position {
    fn bitor_assign(&mut self, rhs: Self) {
        self.x.bitor_assign(rhs.x);
        self.y.bitor_assign(rhs.y);
    }
}

impl BitXor for Position {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
        }
    }
}

impl BitXorAssign for Position {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.x.bitxor_assign(rhs.x);
        self.y.bitxor_assign(rhs.y);
    }
}
