use crate::prelude::*;

pub trait FakeFnOnce {
    type Return;
    #[predicate]
    fn precondition(self) -> bool;

    #[predicate]
    fn postcondition(self, _: Self::Return) -> bool;

    #[requires(self.precondition())]
    #[ensures(self.postcondition(result))]
    fn call(self) -> Self::Return;
}
