use crate::module::{Posts, Profiles, Reactions, Subspaces};
use crate::runner::app::DesmosTestApp;
use cosmwasm_std::Addr;
use desmos_bindings::posts::msg::PostsMsg;
use desmos_bindings::posts::types::{Post, QueryPostRequest, ReplySetting};
use desmos_bindings::profiles::msg::ProfilesMsg;
use desmos_bindings::profiles::types::{Profile, QueryProfileRequest};
use desmos_bindings::reactions::msg::ReactionsMsg;
use desmos_bindings::reactions::types::{
    FreeTextValue, QueryReactionRequest, QueryRegisteredReactionRequest, Reaction, ReactionValue,
    RegisteredReaction,
};
use desmos_bindings::subspaces::msg::SubspacesMsg;
use desmos_bindings::subspaces::types::{QuerySubspaceRequest, Subspace};
use test_tube::{Account, Module, SigningAccount};

/// Create a profile for the specified account.
/// - `app`: Environment in which the profile will be created.
/// - `account`: Account for which the profile will be created.
pub fn create_test_profile(app: &DesmosTestApp, account: &SigningAccount) -> Profile {
    let profiles = Profiles::new(app);

    let mut dtag = account.address();
    dtag.truncate(30);
    profiles
        .save_profile(
            ProfilesMsg::save_profile(
                Some(&dtag),
                None,
                None,
                None,
                None,
                Addr::unchecked(account.address()),
            ),
            account,
        )
        .unwrap();

    Profile::try_from(
        profiles
            .query_profile(&QueryProfileRequest {
                user: account.address(),
            })
            .unwrap()
            .profile
            .unwrap(),
    )
    .unwrap()
}

/// Function to create a test subspace that can be used during the tests.
/// * `app` - Environment where will be created the subspace.
/// * `account` - Account used to create the subspace.
pub fn create_test_subspace(app: &DesmosTestApp, account: &SigningAccount) -> Subspace {
    let subspaces = Subspaces::new(app);

    let created_subspace_id = subspaces
        .create_subspace(
            SubspacesMsg::create_subspace(
                "test",
                "",
                Addr::unchecked(account.address()),
                Addr::unchecked(account.address()),
            ),
            account,
        )
        .unwrap()
        .data
        .subspace_id;

    subspaces
        .query_subspace(&QuerySubspaceRequest {
            subspace_id: created_subspace_id,
        })
        .unwrap()
        .subspace
        .unwrap()
}

/// Create a test post in the subspace with the given `subspace_id`.
/// - `app`: Environment in which the post will be created.
/// - `account`: Account used to create the post.
/// - `subspace_id`: ID of the subspace where the post will be created.
pub fn create_test_post(app: &DesmosTestApp, account: &SigningAccount, subspace_id: u64) -> Post {
    let posts = Posts::new(app);

    let created_post = posts
        .create_post(
            PostsMsg::create_post(
                subspace_id,
                0,
                None,
                "test post",
                None,
                vec![],
                vec![],
                Addr::unchecked(account.address()),
                None,
                ReplySetting::Everyone,
                vec![],
            ),
            &account,
        )
        .unwrap()
        .data
        .post_id;

    posts
        .query_post(&QueryPostRequest {
            subspace_id,
            post_id: created_post,
        })
        .unwrap()
        .post
        .unwrap()
}

/// Add a test reaction to a post in a subspace.
/// - `app`: Environment in which the post exists.
/// - `account`: Account used to create the reaction.
/// - `subspace_id`: ID of the subspace where the post is located.
/// - `post_id`: ID of the post to which the reaction will be added.
pub fn create_test_reaction(
    app: &DesmosTestApp,
    account: &SigningAccount,
    subspace_id: u64,
    post_id: u64,
) -> Reaction {
    let reactions = Reactions::new(app);

    let created_reaction_id = reactions
        .add_reaction(
            ReactionsMsg::add_reaction(
                subspace_id,
                post_id,
                ReactionValue::FreeText(FreeTextValue {
                    text: "like".to_string(),
                }),
                Addr::unchecked(account.address()),
            ),
            account,
        )
        .unwrap()
        .data
        .reaction_id;

    reactions
        .query_reaction(&QueryReactionRequest {
            subspace_id,
            post_id,
            reaction_id: created_reaction_id,
        })
        .unwrap()
        .reaction
        .unwrap()
}

/// Create a test [RegisteredReaction] in the subspace with the provided subspace ID.
/// - `app`: Environment in which the reaction will be created.
/// - `account`: Account used to create the reaction.
/// - `subspace_id`: ID of the subspace where the [RegisteredReaction] will be created.
pub fn create_test_registered_reaction(
    app: &DesmosTestApp,
    account: &SigningAccount,
    subspace_id: u64,
) -> RegisteredReaction {
    let reactions = Reactions::new(app);

    let registered_reaction_id = reactions
        .add_registered_reaction(
            ReactionsMsg::add_registered_reaction(
                subspace_id,
                "like",
                "+1",
                Addr::unchecked(account.address()),
            ),
            &account,
        )
        .unwrap()
        .data
        .registered_reaction_id;

    reactions
        .query_registered_reaction(&QueryRegisteredReactionRequest {
            subspace_id,
            reaction_id: registered_reaction_id,
        })
        .unwrap()
        .registered_reaction
        .unwrap()
}
