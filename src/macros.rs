#[cfg(any(feature = "rust", feature = "gmp"))]
macro_rules! binops_impl {
    ($ops:ident, $func:ident, $ops_assign:ident, $func_assign:ident, $opr:tt, $opr_assign:tt) => {
        impl<'a, 'b> $ops<&'b Bn> for &'a Bn {
            type Output = Bn;

            fn $func(self, rhs: &'b Self::Output) -> Self::Output {
                Bn(self.0.clone() $opr &rhs.0.clone())
            }
        }

        impl<'b> $ops_assign<&'b Bn> for Bn {
            fn $func_assign(&mut self, rhs: &'b Bn) {
                self.0 $opr_assign rhs.0.clone();
            }
        }

        ops_impl!($ops, $func, $ops_assign, $func_assign, $opr);
    };
}

macro_rules! ops_impl {
    ($ops:ident, $func:ident, $ops_assign:ident, $func_assign:ident, $opr:tt) => {
        impl<'b> $ops<&'b Bn> for Bn {
            type Output = Bn;

            fn $func(self, rhs: &'b Self::Output) -> Self::Output {
                &self $opr rhs
            }
        }

        impl<'a> $ops<Bn> for &'a Bn {
            type Output = Bn;

            fn $func(self, rhs: Self::Output) -> Self::Output {
                self $opr &rhs
            }
        }

        impl $ops for Bn {
            type Output = Bn;

            fn $func(self, rhs: Self::Output) -> Self::Output {
                &self $opr &rhs
            }
        }

        impl $ops_assign for Bn {
            fn $func_assign(&mut self, rhs: Bn) {
                *self = &*self $opr &rhs;
            }
        }
    };
}

macro_rules! neg_impl {
    ($ops:expr) => {
        impl<'a> Neg for &'a Bn {
            type Output = Bn;

            fn neg(self) -> Self::Output {
                $ops(&self.0)
            }
        }

        impl Neg for Bn {
            type Output = Bn;

            fn neg(self) -> Self::Output {
                $ops(&self.0)
            }
        }
    };
}

macro_rules! shift_impl {
    (@ref $ops:ident, $func:ident, $opr:expr, $($rhs:ty),+) => {$(
        impl<'a> $ops<$rhs> for &'a Bn {
            type Output = Bn;

            fn $func(self, rhs: $rhs) -> Self::Output {
                $opr(&self.0, rhs)
            }
        }

        impl $ops<$rhs> for Bn {
            type Output = Bn;

            fn $func(self, rhs: $rhs) -> Self::Output {
                $opr(&self.0, rhs)
            }
        }
    )*};
    ($ops:ident, $func:ident, $opr:expr) => {
        shift_impl!(@ref $ops, $func, $opr, u8, u16, u32, u64, usize);
        shift_impl!(@ref $ops, $func, $opr, i8, i16, i32, i64, isize);
    };
}

macro_rules! display_impl {
    () => {
        impl Display for Bn {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Debug for Bn {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}

macro_rules! zeroize_impl {
    ($opr:expr) => {
        impl Zeroize for Bn {
            fn zeroize(&mut self) {
                $opr(self)
            }
        }
    };
}

macro_rules! default_impl {
    ($opr:expr) => {
        impl Default for Bn {
            fn default() -> Self {
                Self($opr())
            }
        }
    };
}

macro_rules! clone_impl {
    ($opr:expr) => {
        impl Clone for Bn {
            fn clone(&self) -> Self {
                Self($opr(self))
            }
        }
    };
}

macro_rules! serdes_impl {
    ($ser:expr, $des:expr) => {
        impl Serialize for Bn {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let str = $ser(self);
                serializer.serialize_str(&str)
            }
        }

        impl<'de> Deserialize<'de> for Bn {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct BnVisitor;

                impl<'de> Visitor<'de> for BnVisitor {
                    type Value = Bn;

                    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        write!(f, "a hex encoded string")
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Bn, E>
                    where
                        E: DError,
                    {
                        let b = $des(s)
                            .map_err(|_| DError::invalid_value(Unexpected::Str(s), &self))?;
                        Ok(Bn(b))
                    }
                }

                deserializer.deserialize_str(BnVisitor)
            }
        }
    };
}

macro_rules! eq_impl {
    () => {
        impl Eq for Bn {}

        impl PartialEq for Bn {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
    };
}

macro_rules! ord_impl {
    () => {
        impl Ord for Bn {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl PartialOrd for Bn {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
    };
}

macro_rules! from_impl {
    ($opr:expr) => {
        impl From<usize> for Bn {
            fn from(d: usize) -> Self {
                Self($opr(d))
            }
        }
    };
}
