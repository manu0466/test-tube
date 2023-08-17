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
    fn_execute! { pub eemove_post_attachment: MsgRemovePostAttachment => MsgRemovePostAttachmentResponse }
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
