use crate::Params;
use crate::collect_into::utils::split_vec_reserve;
use crate::generic_values::TransformableValues;
use crate::generic_values::runner_results::Infallible;
use crate::runner::ParallelRunner;
use crate::using::collect_into::collect::{map_collect_into, xap_collect_into};
use crate::using::collect_into::u_par_collect_into::UParCollectIntoCore;
use orx_concurrent_iter::ConcurrentIter;
use orx_split_vec::{GrowthWithConstantTimeAccess, PseudoDefault, SplitVec};

impl<O, G> UParCollectIntoCore<O> for SplitVec<O, G>
where
    O: Send + Sync,
    G: GrowthWithConstantTimeAccess,
    Self: PseudoDefault,
{
    fn u_m_collect_into<U, R, I, M1>(
        mut self,
        using: U,
        orchestrator: R,
        params: Params,
        iter: I,
        map1: M1,
    ) -> Self
    where
        U: crate::using::using_variants::Using,
        R: ParallelRunner,
        I: ConcurrentIter,
        M1: Fn(&mut U::Item, I::Item) -> O + Sync,
    {
        split_vec_reserve(&mut self, params.is_sequential(), iter.try_get_len());
        let (_, pinned_vec) = map_collect_into(using, orchestrator, params, iter, map1, self);
        pinned_vec
    }

    fn u_x_collect_into<U, R, I, Vo, X1>(
        mut self,
        using: U,
        orchestrator: R,
        params: Params,
        iter: I,
        xap1: X1,
    ) -> Self
    where
        U: crate::using::using_variants::Using,
        R: ParallelRunner,
        I: ConcurrentIter,
        Vo: TransformableValues<Item = O, Fallibility = Infallible>,
        X1: Fn(&mut U::Item, I::Item) -> Vo + Sync,
    {
        split_vec_reserve(&mut self, params.is_sequential(), iter.try_get_len());
        let (_num_spawned, pinned_vec) =
            xap_collect_into(using, orchestrator, params, iter, xap1, self);
        pinned_vec
    }
}
