use desmos_bindings::posts::types::{
    MsgAddPostAttachment, MsgAddPostAttachmentResponse, MsgAnswerPoll, MsgAnswerPollResponse,
    MsgCreatePost, MsgCreatePostResponse, MsgDeletePost, MsgDeletePostResponse, MsgEditPost,
    MsgEditPostResponse, MsgRemovePostAttachment, MsgRemovePostAttachmentResponse, MsgUpdateParams,
    MsgUpdateParamsResponse, QueryParamsRequest, QueryParamsResponse, QueryPollAnswersRequest,
    QueryPollAnswersResponse, QueryPostAttachmentsRequest, QueryPostAttachmentsResponse,
    QueryPostRequest, QueryPostResponse, QuerySectionPostsRequest, QuerySectionPostsResponse,
    QuerySubspacePostsRequest, QuerySubspacePostsResponse,
};
use test_tube::{fn_execute, fn_query, Module, Runner};

/// Posts module.
pub struct Posts<'a, R: Runner<'a>> {
    #[allow(dead_code)]
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Posts<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Posts<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    fn_execute! { pub create_post: MsgCreatePost => MsgCreatePostResponse }
    fn_execute! { pub edit_post: MsgEditPost => MsgEditPostResponse }
    fn_execute! { pub delete_post: MsgDeletePost => MsgDeletePostResponse }
    fn_execute! { pub add_post_attachment: MsgAddPostAttachment => MsgAddPostAttachmentResponse }
    fn_execute! { pub remove_post_attachment: MsgRemovePostAttachment => MsgRemovePostAttachmentResponse }
    fn_execute! { pub answer_poll: MsgAnswerPoll => MsgAnswerPollResponse }
    fn_execute! { pub update_params: MsgUpdateParams => MsgUpdateParamsResponse }

    // ========== Queries ==========

    fn_query! {
        query_subspace_posts ["/desmos.posts.v3.Query/SubspacePosts"]: QuerySubspacePostsRequest => QuerySubspacePostsResponse
    }
    fn_query! {
        query_section_posts ["/desmos.posts.v3.Query/SectionPosts"]: QuerySectionPostsRequest => QuerySectionPostsResponse
    }
    fn_query! {
        query_post ["/desmos.posts.v3.Query/Post"]: QueryPostRequest => QueryPostResponse
    }
    fn_query! {
        query_post_attachments ["/desmos.posts.v3.Query/PostAttachments"]: QueryPostAttachmentsRequest => QueryPostAttachmentsResponse
    }
    fn_query! {
        query_poll_answers ["/desmos.posts.v3.Query/PollAnswers"]: QueryPollAnswersRequest => QueryPollAnswersResponse
    }
    fn_query! {
        query_params ["/desmos.posts.v3.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }
}

#[cfg(test)]
mod test {
    use crate::module::{Posts, Profiles, Subspaces};
    use crate::runner::app::DesmosTestApp;
    use cosmwasm_std::{coin, Addr};
    use desmos_bindings::cosmos_types::Timestamp;
    use desmos_bindings::posts::msg::PostsMsg;
    use desmos_bindings::posts::types::poll::ProvidedAnswer;
    use desmos_bindings::posts::types::{
        AttachmentContent, Media, Poll, QueryPollAnswersRequest, QueryPostAttachmentsRequest,
        QueryPostRequest, QuerySectionPostsRequest, QuerySubspacePostsRequest, ReplySetting,
    };
    use desmos_bindings::profiles::msg::ProfilesMsg;
    use desmos_bindings::subspaces::types::MsgCreateSubspace;
    use prost::Message;
    use test_tube::{Account, Module, SigningAccount};

    /// Setups the test environment creating a new subspace
    /// owned by the provided [SigningAccount], creating a profile
    /// for the provided [SigningAccount] and creating a post in the newly
    /// created subspace.
    fn setup_test_environment(runner: &DesmosTestApp, account: &SigningAccount) -> (u64, u64) {
        let profiles = Profiles::new(runner);
        let subspaces = Subspaces::new(runner);
        let posts = Posts::new(runner);

        // Create a test subspace
        let created_subspace = subspaces
            .create_subspace(
                MsgCreateSubspace {
                    name: "test".to_string(),
                    description: "test subspace".to_string(),
                    owner: account.address(),
                    creator: account.address(),
                },
                &account,
            )
            .unwrap()
            .data
            .subspace_id;

        // Create a profile for the user
        profiles
            .save_profile(
                ProfilesMsg::save_profile(
                    Some("test"),
                    None,
                    None,
                    None,
                    None,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Create a new post in the created subspace
        let created_post = posts
            .create_post(
                PostsMsg::create_post(
                    created_subspace,
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

        return (created_subspace, created_post);
    }

    #[test]
    fn test_post_manipulation() {
        let app = DesmosTestApp::new();
        let posts = Posts::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let (subspace_id, created_post) = setup_test_environment(&app, &account);

        // Edit the post
        posts
            .edit_post(
                PostsMsg::edit_post(
                    subspace_id,
                    created_post,
                    Some("new content"),
                    None,
                    vec![],
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Delete the created post
        posts
            .delete_post(
                PostsMsg::delete_post(
                    subspace_id,
                    created_post,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_post_attachment_manipulation() {
        let app = DesmosTestApp::new();
        let posts = Posts::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let (subspace_id, created_post) = setup_test_environment(&app, &account);

        // Test attachment creation
        let created_attachment = posts
            .add_post_attachment(
                PostsMsg::add_post_attachment(
                    subspace_id,
                    created_post,
                    AttachmentContent::Media(Media {
                        uri: "https://desmos.network/image.png".to_string(),
                        mime_type: "image/png".to_string(),
                    }),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .attachment_id;

        // Test attachment remove
        posts
            .remove_post_attachment(
                PostsMsg::remove_post_attachment(
                    subspace_id,
                    created_post,
                    created_attachment,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_poll_answer() {
        let app = DesmosTestApp::new();
        let posts = Posts::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let (subspace_id, created_post) = setup_test_environment(&app, &account);

        // Test poll creation
        let current_time_sec = app.get_block_time_seconds();
        let created_attachment = posts
            .add_post_attachment(
                PostsMsg::add_post_attachment(
                    subspace_id,
                    created_post,
                    AttachmentContent::Poll(Poll {
                        question: "42?".to_string(),
                        provided_answers: vec![
                            ProvidedAnswer {
                                text: "Yes".to_string(),
                                attachments: vec![],
                            },
                            ProvidedAnswer {
                                text: "No".to_string(),
                                attachments: vec![],
                            },
                        ],
                        end_date: Some(Timestamp {
                            seconds: current_time_sec + 3600,
                            nanos: 0,
                        }),
                        allows_multiple_answers: false,
                        allows_answer_edits: false,
                        final_tally_results: None,
                    }),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .attachment_id;

        // Test poll answer
        posts
            .answer_poll(
                PostsMsg::answer_poll(
                    subspace_id,
                    created_post,
                    created_attachment,
                    vec![0],
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_query_subspace_posts() {
        let app = DesmosTestApp::new();
        let posts = Posts::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let (subspace_id, created_post_id) = setup_test_environment(&app, &account);

        let posts = posts
            .query_subspace_posts(&QuerySubspacePostsRequest {
                subspace_id,
                pagination: None,
            })
            .unwrap()
            .posts;

        assert_eq!(1, posts.len());
        assert_eq!(created_post_id, posts[0].id);
        assert_eq!("test post", posts[0].text);
    }

    #[test]
    fn test_query_section_posts() {
        let app = DesmosTestApp::new();
        let posts = Posts::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let (subspace_id, created_post_id) = setup_test_environment(&app, &account);

        let posts = posts
            .query_section_posts(&QuerySectionPostsRequest {
                subspace_id,
                section_id: 0,
                pagination: None,
            })
            .unwrap()
            .posts;

        assert_eq!(1, posts.len());
        assert_eq!(created_post_id, posts[0].id);
        assert_eq!("test post", posts[0].text);
    }

    #[test]
    fn test_query_post() {
        let app = DesmosTestApp::new();
        let posts = Posts::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let (subspace_id, created_post_id) = setup_test_environment(&app, &account);

        let post = posts
            .query_post(&QueryPostRequest {
                subspace_id,
                post_id: created_post_id,
            })
            .unwrap()
            .post
            .unwrap();

        assert_eq!(created_post_id, post.id);
        assert_eq!("test post", post.text);
    }

    #[test]
    fn test_query_post_attachment() {
        let app = DesmosTestApp::new();
        let posts = Posts::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let (subspace_id, created_post_id) = setup_test_environment(&app, &account);

        // Create a test attachment
        let media_attachment = Media {
            uri: "https://desmos.network/image.png".to_string(),
            mime_type: "image/png".to_string(),
        };
        posts
            .add_post_attachment(
                PostsMsg::add_post_attachment(
                    subspace_id,
                    created_post_id,
                    AttachmentContent::Media(media_attachment.clone()),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        let attachments = posts
            .query_post_attachments(&QueryPostAttachmentsRequest {
                subspace_id,
                post_id: created_post_id,
                pagination: None,
            })
            .unwrap()
            .attachments;

        assert_eq!(1, attachments.len());
        let attachment = attachments.get(0).unwrap();
        let queried_media =
            Media::decode(attachment.content.as_ref().unwrap().value.as_slice()).unwrap();
        assert_eq!(media_attachment, queried_media);
    }

    #[test]
    fn test_query_poll_answers() {
        let app = DesmosTestApp::new();
        let posts = Posts::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let (subspace_id, created_post_id) = setup_test_environment(&app, &account);

        // Create a test poll
        let current_time_sec = app.get_block_time_seconds();
        let created_poll = posts
            .add_post_attachment(
                PostsMsg::add_post_attachment(
                    subspace_id,
                    created_post_id,
                    AttachmentContent::Poll(Poll {
                        question: "42?".to_string(),
                        provided_answers: vec![
                            ProvidedAnswer {
                                text: "Yes".to_string(),
                                attachments: vec![],
                            },
                            ProvidedAnswer {
                                text: "No".to_string(),
                                attachments: vec![],
                            },
                        ],
                        end_date: Some(Timestamp {
                            seconds: current_time_sec + 3600,
                            nanos: 0,
                        }),
                        allows_multiple_answers: false,
                        allows_answer_edits: false,
                        final_tally_results: None,
                    }),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .attachment_id;

        // Answer the created poll
        posts
            .answer_poll(
                PostsMsg::answer_poll(
                    subspace_id,
                    created_post_id,
                    created_poll,
                    vec![0],
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Query the poll answers
        let answers = posts
            .query_poll_answers(&QueryPollAnswersRequest {
                subspace_id,
                post_id: created_post_id,
                poll_id: created_poll,
                user: account.address(),
                pagination: None,
            })
            .unwrap()
            .answers;

        assert_eq!(1, answers.len());
        let answer = answers.get(0).unwrap();
        assert_eq!(subspace_id, answer.subspace_id);
        assert_eq!(created_post_id, answer.post_id);
        assert_eq!(created_poll, answer.poll_id);
        assert_eq!(vec![0], answer.answers_indexes);
        assert_eq!(account.address(), answer.user);
    }
}
