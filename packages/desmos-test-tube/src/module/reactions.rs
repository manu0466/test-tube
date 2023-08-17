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
        pub query_reactions ["/desmos.reactions.v1.Query/Reactions"]:  QueryReactionsRequest => QueryReactionsResponse
    }
    fn_query! {
        pub query_reaction ["/desmos.reactions.v1.Query/Reaction"]:  QueryReactionRequest => QueryReactionResponse
    }
    fn_query! {
        pub query_registered_reactions ["/desmos.reactions.v1.Query/RegisteredReactions"]:  QueryRegisteredReactionsRequest => QueryRegisteredReactionsResponse
    }
    fn_query! {
        pub query_registered_reaction ["/desmos.reactions.v1.Query/RegisteredReaction"]:  QueryRegisteredReactionRequest => QueryRegisteredReactionResponse
    }
    fn_query! {
        pub query_params ["/desmos.reactions.v1.Query/ReactionsParams"]:  QueryReactionsParamsRequest => QueryReactionsParamsResponse
    }
}

#[cfg(test)]
mod test {
    use crate::module::test_utils::*;
    use crate::module::Reactions;
    use crate::runner::app::DesmosTestApp;
    use cosmwasm_std::{coin, Addr};
    use desmos_bindings::reactions::msg::ReactionsMsg;
    use desmos_bindings::reactions::types::{
        FreeTextValue, QueryReactionRequest, QueryReactionsRequest, QueryRegisteredReactionRequest,
        QueryRegisteredReactionsRequest, ReactionValue,
    };
    use test_tube::{Account, Module};

    #[test]
    fn test_reaction_add_remove() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let reactions = Reactions::new(&app);

        // Prepare the environment
        create_test_profile(&app, &account);
        let subspace = create_test_subspace(&app, &account);
        let post = create_test_post(&app, &account, subspace.id);

        // Add a reaction to the created post.
        let created_reaction_id = reactions
            .add_reaction(
                ReactionsMsg::add_reaction(
                    subspace.id,
                    post.id,
                    ReactionValue::FreeText(FreeTextValue {
                        text: "Like".to_string(),
                    }),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .reaction_id;

        // Test reaction remove
        reactions
            .remove_reaction(
                ReactionsMsg::remove_reaction(
                    subspace.id,
                    post.id,
                    created_reaction_id,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_registered_reactions_add_edit_remove() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let reactions = Reactions::new(&app);

        // Prepare the environment
        let subspace = create_test_subspace(&app, &account);

        // Add a registered reaction
        let registered_reaction_id = reactions
            .add_registered_reaction(
                ReactionsMsg::add_registered_reaction(
                    subspace.id,
                    "like",
                    "+1",
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .registered_reaction_id;

        // Try to edit the previously created reaction
        reactions
            .edit_registered_reaction(
                ReactionsMsg::edit_registered_reaction(
                    subspace.id,
                    registered_reaction_id,
                    "dislike",
                    "-1",
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Remove the registered reaction
        reactions
            .remove_registered_reaction(
                ReactionsMsg::remove_registered_reaction(
                    subspace.id,
                    registered_reaction_id,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_query_reaction() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let reactions = Reactions::new(&app);

        // Prepare the environment
        create_test_profile(&app, &account);
        let subspace = create_test_subspace(&app, &account);
        let post = create_test_post(&app, &account, subspace.id);
        let reaction = create_test_reaction(&app, &account, subspace.id, post.id);

        // Test reaction query
        let queried_reaction = reactions
            .query_reaction(&QueryReactionRequest {
                subspace_id: subspace.id,
                post_id: post.id,
                reaction_id: reaction.id,
            })
            .unwrap()
            .reaction
            .unwrap();
        assert_eq!(reaction, queried_reaction);

        // Test reactions query
        let queried_reactions = reactions
            .query_reactions(&QueryReactionsRequest {
                subspace_id: subspace.id,
                post_id: post.id,
                user: account.address(),
                pagination: None,
            })
            .unwrap()
            .reactions;
        assert_eq!(vec![reaction], queried_reactions);
    }

    #[test]
    fn test_query_registered_reaction() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let reactions = Reactions::new(&app);

        // Prepare the environment
        let subspace = create_test_subspace(&app, &account);
        let registered_reaction = create_test_registered_reaction(&app, &account, subspace.id);

        // Test registered reaction query
        let queried_registered_reaction = reactions
            .query_registered_reaction(&QueryRegisteredReactionRequest {
                subspace_id: subspace.id,
                reaction_id: registered_reaction.id,
            })
            .unwrap()
            .registered_reaction
            .unwrap();
        assert_eq!(registered_reaction, queried_registered_reaction);

        // Test registered reactions query
        let queried_registered_reactions = reactions
            .query_registered_reactions(&QueryRegisteredReactionsRequest {
                subspace_id: subspace.id,
                pagination: None,
            })
            .unwrap()
            .registered_reactions;
        assert_eq!(vec![registered_reaction], queried_registered_reactions);
    }
}
