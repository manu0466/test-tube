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

#[cfg(test)]
mod tests {
    use crate::module::test_utils::*;
    use crate::module::Relationships;
    use crate::runner::app::DesmosTestApp;
    use cosmwasm_std::coin;
    use desmos_bindings::relationships::types::{
        MsgBlockUser, MsgCreateRelationship, MsgDeleteRelationship, MsgUnblockUser,
        QueryBlocksRequest, QueryRelationshipsRequest,
    };
    use test_tube::{Account, Module};

    #[test]
    fn test_create_delete_relationship() {
        let app = DesmosTestApp::new();
        let accounts = app
            .init_accounts(&[coin(100_000_000_000, "udsm")], 2)
            .unwrap();
        let relationships = Relationships::new(&app);
        let account_0 = accounts.get(0).unwrap();
        let account_1 = accounts.get(1).unwrap();

        // Setup test environment
        let subspace = create_test_subspace(&app, account_0);
        create_test_profile(&app, account_0);
        create_test_profile(&app, account_1);

        // Create a relationship between the user 0 and 1
        relationships
            .create_relationship(
                MsgCreateRelationship {
                    subspace_id: subspace.id,
                    counterparty: account_1.address(),
                    signer: account_0.address(),
                },
                account_0,
            )
            .unwrap();

        // Delete the previously created relationship
        relationships
            .delete_relationship(
                MsgDeleteRelationship {
                    subspace_id: subspace.id,
                    counterparty: account_1.address(),
                    signer: account_0.address(),
                },
                account_0,
            )
            .unwrap();
    }

    #[test]
    fn test_block_unblock_user() {
        let app = DesmosTestApp::new();
        let accounts = app
            .init_accounts(&[coin(100_000_000_000, "udsm")], 2)
            .unwrap();
        let relationships = Relationships::new(&app);
        let account_0 = accounts.get(0).unwrap();
        let account_1 = accounts.get(1).unwrap();

        // Setup test environment
        let subspace = create_test_subspace(&app, account_0);
        create_test_profile(&app, account_0);
        create_test_profile(&app, account_1);

        // Test user0 block user1
        relationships
            .block_user(
                MsgBlockUser {
                    subspace_id: subspace.id,
                    blocked: account_1.address(),
                    reason: "test".to_string(),
                    blocker: account_0.address(),
                },
                account_0,
            )
            .unwrap();

        // Test unblock
        relationships
            .unblock_user(
                MsgUnblockUser {
                    subspace_id: subspace.id,
                    blocked: account_1.address(),
                    blocker: account_0.address(),
                },
                account_0,
            )
            .unwrap();
    }

    #[test]
    fn test_query_relationship() {
        let app = DesmosTestApp::new();
        let accounts = app
            .init_accounts(&[coin(100_000_000_000, "udsm")], 2)
            .unwrap();
        let relationships = Relationships::new(&app);
        let account_0 = accounts.get(0).unwrap();
        let account_1 = accounts.get(1).unwrap();

        // Setup test environment
        let subspace = create_test_subspace(&app, account_0);
        create_test_profile(&app, account_0);
        create_test_profile(&app, account_1);

        // Create a relationship between the user 0 and 1
        relationships
            .create_relationship(
                MsgCreateRelationship {
                    subspace_id: subspace.id,
                    counterparty: account_1.address(),
                    signer: account_0.address(),
                },
                account_0,
            )
            .unwrap();

        let relationships = relationships
            .query_relationships(&QueryRelationshipsRequest {
                subspace_id: subspace.id,
                counterparty: account_1.address(),
                user: account_0.address(),
                pagination: None,
            })
            .unwrap()
            .relationships;

        assert_eq!(1, relationships.len());
        let relationship = relationships.get(0).unwrap();
        assert_eq!(subspace.id, relationship.subspace_id);
        assert_eq!(account_0.address(), relationship.creator);
        assert_eq!(account_1.address(), relationship.counterparty);
    }

    #[test]
    fn test_query_blocks() {
        let app = DesmosTestApp::new();
        let accounts = app
            .init_accounts(&[coin(100_000_000_000, "udsm")], 2)
            .unwrap();
        let relationships = Relationships::new(&app);
        let account_0 = accounts.get(0).unwrap();
        let account_1 = accounts.get(1).unwrap();

        // Setup test environment
        let subspace = create_test_subspace(&app, account_0);
        create_test_profile(&app, account_0);
        create_test_profile(&app, account_1);

        // Test user0 block user1
        relationships
            .block_user(
                MsgBlockUser {
                    subspace_id: subspace.id,
                    blocked: account_1.address(),
                    reason: "test".to_string(),
                    blocker: account_0.address(),
                },
                account_0,
            )
            .unwrap();

        // Test unblock
        let blocks = relationships
            .query_blocks(&QueryBlocksRequest {
                subspace_id: subspace.id,
                blocked: account_1.address(),
                blocker: account_0.address(),
                pagination: None,
            })
            .unwrap()
            .blocks;

        assert_eq!(1, blocks.len());
        let block = blocks.get(0).unwrap();
        assert_eq!(subspace.id, block.subspace_id);
        assert_eq!(account_0.address(), block.blocker);
        assert_eq!(account_1.address(), block.blocked);
        assert_eq!("test", block.reason);
    }
}
