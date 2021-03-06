// SPEC LINES 8 + 2 + 2 + 2 + 5 + 2 + 4 + 5 + 8 + 3 + 4 + 3
use crate::prelude::*;

pub struct Vec<T>(std::vec::Vec<T>);

#[cfg(feature = "contracts")]
impl<T> Model for Vec<T> {
    type ModelTy = Seq<T>;
    #[logic]
    #[trusted]
    #[ensures(result.len() <= @usize::MAX)]
    fn model(self) -> Self::ModelTy {
        std::process::abort()
    }
}

impl<T> Vec<T> {
    #[trusted]
    #[ensures((@result).len() === 0)]
    pub fn new() -> Self {
        Vec(std::vec::Vec::new())
    }

    #[trusted]
    #[ensures((@result).len() === 0)]
    pub fn with_capacity(capacity: usize) -> Vec<T> {
        Vec(std::vec::Vec::with_capacity(capacity))
    }

    #[trusted]
    #[ensures(result.into() === (@self).len())]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[trusted]
    #[ensures(match result {
        Some(t) => *t === (@*self)[ix.into()],
        None => (@*self).len() <= ix.into(),
    })]
    pub fn get(&self, ix: usize) -> Option<&T> {
        self.0.get(ix)
    }

    #[trusted]
    #[ensures(@^self === (@self).push(v))]
    pub fn push(&mut self, v: T) {
        self.0.push(v)
    }

    #[trusted]
    #[requires(@i < (@self).len())]
    #[requires(@j < (@self).len())]
    #[ensures((@^self).exchange(@*self, @i, @j))]
    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j)
    }

    #[trusted]
    #[ensures(match result {
        Some(t) => (@self) === (@^self).push(t),
        None => (@self).len() === (@^self).len() && (@self).len() === 0
    })]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    #[trusted]
    #[ensures((@*self).len() === (@result).len() && (@*self).len() === (@^self).len())]
    #[ensures(forall<i : Int> 0 <= i && i <= (@*self).len() ==> (@*self)[i] === *(@result)[i])]
    #[ensures(forall<i : Int> 0 <= i && i <= (@^self).len() ==> (@^self)[i] === ^(@result)[i])]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.0.iter_mut())
    }
}

#[trusted]
#[ensures((@result).len() === @n)]
#[ensures(forall<i : Int> 0 <= i && i < @n ==> (@result)[i] === elem)]
pub fn from_elem<T: ::std::clone::Clone>(elem: T, n: usize) -> Vec<T> {
    Vec(::std::vec::from_elem(elem, n))
}

impl<T> ::std::ops::IndexMut<usize> for Vec<T> {
    #[trusted]
    #[requires(@ix < (@*self).len())]
    #[ensures(*result === (@self)[@ix])]
    #[ensures(^result === (@^self)[@ix])]
    #[ensures(forall<j : Int> 0 <= j && j < (@^self).len() ==>
        !(j === @ix) ==>
        (@^self)[j] === (@*self)[j])]
    #[ensures((@*self).len() === (@^self).len())]
    fn index_mut(&mut self, ix: usize) -> &mut T {
        self.0.index_mut(ix)
    }
}

impl<T> ::std::ops::Index<usize> for Vec<T> {
    type Output = T;

    #[trusted]
    #[requires(@ix < (@self).len())]
    #[ensures(*result === (@self)[@ix])]
    fn index(&self, ix: usize) -> &T {
        self.0.index(ix)
    }
}
