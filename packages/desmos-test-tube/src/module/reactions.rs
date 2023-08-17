use desmos_bindings::reactions::types::{
    MsgAddReaction, MsgAddReactionResponse, MsgAddRegisteredReaction,
    MsgAddRegisteredReactionResponse, MsgEditRegisteredReaction, MsgEditRegisteredReactionResponse,
    MsgRemoveReaction, MsgRemoveReactionResponse, MsgRemoveRegisteredReaction,
    MsgRemoveRegisteredReactionResponse, MsgSetReactionsParams, MsgSetReactionsParamsResponse,
    QueryReactionRequest, QueryReactionResponse, QueryReactionsParamsRequest,
    QueryReactionsParamsResponse, QueryReactionsRequest, QueryReactionsResponse,
    QueryRegisteredReactionRequest, QueryRegisteredReactionResponse,
    QueryRegisteredReactionsRequest, QueryRegisteredReactionsResponse,
};
use test_tube::{fn_execute, fn_query, Module, Runner};

/// Reactions module.
pub struct Reactions<'a, R: Runner<'a>> {
    #[allow(dead_code)]
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Reactions<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Reactions<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    fn_execute! { pub add_reaction : MsgAddReaction => MsgAddReactionResponse }
    fn_execute! { pub remove_reaction : MsgRemoveReaction => MsgRemoveReactionResponse }
    fn_execute! { pub add_registered_reaction : MsgAddRegisteredReaction => MsgAddRegisteredReactionResponse }
    fn_execute! { pub edit_registered_reaction : MsgEditRegisteredReaction => MsgEditRegisteredReactionResponse }
    fn_execute! { pub remove_registered_reaction : MsgRemoveRegisteredReaction => MsgRemoveRegisteredReactionResponse }
    fn_execute! { pub update_params : MsgSetReactionsParams => MsgSetReactionsParamsResponse }

    // ========== Queries ==========

    fn_query! {
        query_reactions ["/desmos.reactions.v1.Query/Reactions"]:  QueryReactionsRequest => QueryReactionsResponse
    }
    fn_query! {
        query_reaction ["/desmos.reactions.v1.Query/Reaction"]:  QueryReactionRequest => QueryReactionResponse
    }
    fn_query! {
        query_registered_reactions ["/desmos.reactions.v1.Query/RegisteredReactions"]:  QueryRegisteredReactionsRequest => QueryRegisteredReactionsResponse
    }
    fn_query! {
        query_registered_reaction ["/desmos.reactions.v1.Query/RegisteredReaction"]:  QueryRegisteredReactionRequest => QueryRegisteredReactionResponse
    }
    fn_query! {
        query_params ["/desmos.reactions.v1.Query/ReactionsParams"]:  QueryReactionsParamsRequest => QueryReactionsParamsResponse
    }
}
