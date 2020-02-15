pub trait SmartEnum<E:'static+Copy>: Sized+'static {
    type Type;
    const LEN: usize;
    #[warn(clippy::declare_interior_mutable_const)] // Remove when const generics improve
    const VALUES: &'static [E];
}

#[macro_export]
macro_rules! smart_enum{
    (@count $i:ident) => {
        1
    };
    
    (@count $i:ident, $($is:ident),+$(,)?) => {
        1 + smart_enum!(@count $($is),+) 
    };

    ($name: ident: $prim: ident = $($i: ident),+$(,)?) => {
        #[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Debug,std::hash::Hash)]
        #[repr($prim)]
        pub enum $name {
            $($i,)+
        }

        impl Into<usize> for $name {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl SmartEnum<$name> for $name {
            type Type = $name;
            const LEN: usize = {smart_enum!(@count $($i),+)};
            const VALUES: &'static [$name] = &[$(Self::$i,)+];
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    $( Self::$i => write!(f, stringify!($i)), )+
                }
            }
        }
    };
}
pub use smart_enum;
