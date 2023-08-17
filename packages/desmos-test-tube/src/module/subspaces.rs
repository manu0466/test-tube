use desmos_bindings::subspaces::types::{
    MsgAddUserToUserGroup, MsgAddUserToUserGroupResponse, MsgCreateSection,
    MsgCreateSectionResponse, MsgCreateSubspace, MsgCreateSubspaceResponse, MsgCreateUserGroup,
    MsgCreateUserGroupResponse, MsgDeleteSection, MsgDeleteSectionResponse, MsgDeleteSubspace,
    MsgDeleteSubspaceResponse, MsgDeleteUserGroup, MsgDeleteUserGroupResponse, MsgEditSection,
    MsgEditSectionResponse, MsgEditSubspace, MsgEditSubspaceResponse, MsgEditUserGroup,
    MsgEditUserGroupResponse, MsgGrantAllowance, MsgGrantAllowanceResponse,
    MsgGrantTreasuryAuthorization, MsgGrantTreasuryAuthorizationResponse, MsgMoveSection,
    MsgMoveSectionResponse, MsgMoveUserGroup, MsgMoveUserGroupResponse, MsgRemoveUserFromUserGroup,
    MsgRemoveUserFromUserGroupResponse, MsgRevokeAllowance, MsgRevokeAllowanceResponse,
    MsgRevokeTreasuryAuthorization, MsgRevokeTreasuryAuthorizationResponse,
    MsgSetUserGroupPermissions, MsgSetUserGroupPermissionsResponse, MsgSetUserPermissions,
    MsgSetUserPermissionsResponse, QueryGroupAllowancesRequest, QueryGroupAllowancesResponse,
    QuerySectionRequest, QuerySectionResponse, QuerySectionsRequest, QuerySectionsResponse,
    QuerySubspaceRequest, QuerySubspaceResponse, QuerySubspacesRequest, QuerySubspacesResponse,
    QueryUserAllowancesRequest, QueryUserAllowancesResponse, QueryUserGroupMembersRequest,
    QueryUserGroupMembersResponse, QueryUserGroupRequest, QueryUserGroupResponse,
    QueryUserGroupsRequest, QueryUserGroupsResponse, QueryUserPermissionsRequest,
    QueryUserPermissionsResponse,
};
use test_tube::{fn_execute, fn_query, Module, Runner};

/// Subspaces module.
pub struct Subspaces<'a, R: Runner<'a>> {
    #[allow(dead_code)]
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Subspaces<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Subspaces<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    fn_execute! { pub grant_treasury_authorization: MsgGrantTreasuryAuthorization => MsgGrantTreasuryAuthorizationResponse }
    fn_execute! { pub revoke_treasury_authorization: MsgRevokeTreasuryAuthorization => MsgRevokeTreasuryAuthorizationResponse }
    fn_execute! { pub grant_allowance: MsgGrantAllowance => MsgGrantAllowanceResponse }
    fn_execute! { pub revoke_allowance: MsgRevokeAllowance => MsgRevokeAllowanceResponse }
    fn_execute! { pub create_subspace: MsgCreateSubspace => MsgCreateSubspaceResponse }
    fn_execute! { pub edit_subspace: MsgEditSubspace => MsgEditSubspaceResponse }
    fn_execute! { pub delete_subspace: MsgDeleteSubspace => MsgDeleteSubspaceResponse }
    fn_execute! { pub create_section: MsgCreateSection => MsgCreateSectionResponse }
    fn_execute! { pub edit_section: MsgEditSection => MsgEditSectionResponse }
    fn_execute! { pub move_section: MsgMoveSection => MsgMoveSectionResponse }
    fn_execute! { pub delete_section: MsgDeleteSection => MsgDeleteSectionResponse }
    fn_execute! { pub create_user_group: MsgCreateUserGroup => MsgCreateUserGroupResponse }
    fn_execute! { pub edit_user_group: MsgEditUserGroup => MsgEditUserGroupResponse }
    fn_execute! { pub move_user_group: MsgMoveUserGroup => MsgMoveUserGroupResponse }
    fn_execute! { pub set_user_group_permissions: MsgSetUserGroupPermissions => MsgSetUserGroupPermissionsResponse }
    fn_execute! { pub delete_user_group: MsgDeleteUserGroup => MsgDeleteUserGroupResponse }
    fn_execute! { pub add_user_to_user_group: MsgAddUserToUserGroup => MsgAddUserToUserGroupResponse }
    fn_execute! { pub remove_user_from_user_group: MsgRemoveUserFromUserGroup => MsgRemoveUserFromUserGroupResponse }
    fn_execute! { pub set_user_permissions: MsgSetUserPermissions => MsgSetUserPermissionsResponse }

    // ========== Queries ==========

    fn_query! {
        query_subspaces ["/desmos.subspaces.v3.Query/Subspaces"]: QuerySubspacesRequest => QuerySubspacesResponse
    }
    fn_query! {
        query_subspace ["/desmos.subspaces.v3.Query/Subspace"]: QuerySubspaceRequest => QuerySubspaceResponse
    }
    fn_query! {
        query_sections ["/desmos.subspaces.v3.Query/Sections"]: QuerySectionsRequest => QuerySectionsResponse
    }
    fn_query! {
        query_section ["/desmos.subspaces.v3.Query/Section"]: QuerySectionRequest => QuerySectionResponse
    }
    fn_query! {
        query_user_groups ["/desmos.subspaces.v3.Query/UserGroups"]: QueryUserGroupsRequest => QueryUserGroupsResponse
    }
    fn_query! {
        query_user_group ["/desmos.subspaces.v3.Query/UserGroup"]: QueryUserGroupRequest => QueryUserGroupResponse
    }
    fn_query! {
        query_user_group_members ["/desmos.subspaces.v3.Query/UserGroupMembers"]: QueryUserGroupMembersRequest => QueryUserGroupMembersResponse
    }
    fn_query! {
        query_user_permissions ["/desmos.subspaces.v3.Query/UserPermissions"]: QueryUserPermissionsRequest => QueryUserPermissionsResponse
    }
    fn_query! {
        query_uer_allowances ["/desmos.subspaces.v3.Query/UserAllowances"]: QueryUserAllowancesRequest => QueryUserAllowancesResponse
    }
    fn_query! {
        query_group_allowances ["/desmos.subspaces.v3.Query/GroupAllowances"]: QueryGroupAllowancesRequest => QueryGroupAllowancesResponse
    }
}
