pub use crate::smart_enum::*;

pub struct EnumVec<E,V> {
    data: Vec<V>,
    phantom: std::marker::PhantomData<E>,
}

impl<E,V> EnumVec<E,V> 
    where E: Copy + Into<usize> + SmartEnum<E>,
{
    pub fn fill(mut f: impl FnMut(E) -> V) -> Self {
        unsafe {
            let mut data = Vec::new();
            data.resize_with(E::LEN, || std::mem::MaybeUninit::uninit().assume_init());

            for (i,&e) in E::VALUES.iter().enumerate() {
                data[i] = f(e);
            }

            Self {
                data,
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl<E,V> std::ops::Index<E> for EnumVec<E,V>
    where E: Into<usize>
{
    type Output = V;

    fn index(&self, e: E) -> &Self::Output {
        &self.data[e.into()]
    }
}
    
#[macro_export]
macro_rules! enum_vec (
    ($k:ident -> $v: ident $match: tt) => {
        {
            let a: EnumVec<$k, $v> = EnumVec::fill(|e| match e $match);
            a
        }
    }
);
pub use enum_vec;
