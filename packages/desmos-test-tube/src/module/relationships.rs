use desmos_bindings::relationships::types::{
    MsgBlockUser, MsgBlockUserResponse, MsgCreateRelationship, MsgCreateRelationshipResponse,
    MsgDeleteRelationship, MsgDeleteRelationshipResponse, MsgUnblockUser, MsgUnblockUserResponse,
    QueryBlocksRequest, QueryBlocksResponse, QueryRelationshipsRequest, QueryRelationshipsResponse,
};
use test_tube::{fn_execute, fn_query, Module, Runner};

/// Relationships module.
pub struct Relationships<'a, R: Runner<'a>> {
    #[allow(dead_code)]
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Relationships<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Relationships<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    fn_execute! { pub create_relationship: MsgCreateRelationship => MsgCreateRelationshipResponse }
    fn_execute! { pub delete_relationship: MsgDeleteRelationship => MsgDeleteRelationshipResponse }
    fn_execute! { pub block_user: MsgBlockUser => MsgBlockUserResponse }
    fn_execute! { pub unblock_user: MsgUnblockUser => MsgUnblockUserResponse }

    // ========== Queries ==========

    fn_query! {
        pub query_relationships ["/desmos.relationships.v1.Query/Relationships"]: QueryRelationshipsRequest => QueryRelationshipsResponse
    }
    fn_query! {
        pub query_blocks ["/desmos.relationships.v1.Query/Blocks"]: QueryBlocksRequest => QueryBlocksResponse
    }
}
