// SPEC LINES 9 + 3
use crate::prelude::*;
pub struct IterMut<'a, T>(pub ::std::slice::IterMut<'a, T>);

#[cfg(feature = "contracts")]
impl<'a, T> Model for IterMut<'a, T> {
    type ModelTy = Seq<&'a mut T>;

    #[trusted]
    #[logic]
    fn model(self) -> Self::ModelTy {
        std::process::abort()
    }
}

impl<'a, T> IterMut<'a, T> {
    #[trusted]
    #[ensures(result === (@*self).get(0))]
    #[ensures(@^self === (@*self).tail())]
    pub fn next(&mut self) -> Option<&'a mut T> {
        self.0.next()
    }
}
