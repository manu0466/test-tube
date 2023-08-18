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
        pub query_subspaces ["/desmos.subspaces.v3.Query/Subspaces"]: QuerySubspacesRequest => QuerySubspacesResponse
    }
    fn_query! {
        pub query_subspace ["/desmos.subspaces.v3.Query/Subspace"]: QuerySubspaceRequest => QuerySubspaceResponse
    }
    fn_query! {
        pub query_sections ["/desmos.subspaces.v3.Query/Sections"]: QuerySectionsRequest => QuerySectionsResponse
    }
    fn_query! {
        pub query_section ["/desmos.subspaces.v3.Query/Section"]: QuerySectionRequest => QuerySectionResponse
    }
    fn_query! {
        pub query_user_groups ["/desmos.subspaces.v3.Query/UserGroups"]: QueryUserGroupsRequest => QueryUserGroupsResponse
    }
    fn_query! {
        pub query_user_group ["/desmos.subspaces.v3.Query/UserGroup"]: QueryUserGroupRequest => QueryUserGroupResponse
    }
    fn_query! {
        pub query_user_group_members ["/desmos.subspaces.v3.Query/UserGroupMembers"]: QueryUserGroupMembersRequest => QueryUserGroupMembersResponse
    }
    fn_query! {
        pub query_user_permissions ["/desmos.subspaces.v3.Query/UserPermissions"]: QueryUserPermissionsRequest => QueryUserPermissionsResponse
    }
    fn_query! {
        pub query_user_allowances ["/desmos.subspaces.v3.Query/UserAllowances"]: QueryUserAllowancesRequest => QueryUserAllowancesResponse
    }
    fn_query! {
        pub query_group_allowances ["/desmos.subspaces.v3.Query/GroupAllowances"]: QueryGroupAllowancesRequest => QueryGroupAllowancesResponse
    }
}

#[cfg(test)]
mod tests {
    use crate::module::test_utils::*;
    use crate::module::Subspaces;
    use crate::runner::app::DesmosTestApp;
    use cosmwasm_std::{coin, Addr};
    use desmos_bindings::subspaces::msg::SubspacesMsg;
    use desmos_bindings::subspaces::types::{
        Permission, QuerySectionRequest, QuerySectionsRequest, QuerySubspaceRequest,
        QuerySubspacesRequest, QueryUserGroupMembersRequest, QueryUserGroupRequest,
        QueryUserGroupsRequest, QueryUserPermissionsRequest, Section, UserGroup,
    };
    use test_tube::{Account, Module};

    #[test]
    fn test_subspace_management() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Test subspace creation
        let created_subspace = subspaces
            .create_subspace(
                SubspacesMsg::create_subspace(
                    "test",
                    "test",
                    Addr::unchecked(account.address()),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .subspace_id;

        // Test subspace edit
        subspaces
            .edit_subspace(
                SubspacesMsg::edit_subspace(
                    created_subspace,
                    "New name",
                    "New description",
                    Addr::unchecked(account.address()),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Test subspace delete
        subspaces
            .delete_subspace(
                SubspacesMsg::delete_subspace(created_subspace, Addr::unchecked(account.address())),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_section_management() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let subspace = create_test_subspace(&app, &account);

        // Test create section
        let section_id = subspaces
            .create_section(
                SubspacesMsg::create_section(
                    subspace.id,
                    "section",
                    "test section",
                    0,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .section_id;

        // Test section edit
        subspaces
            .edit_section(
                SubspacesMsg::edit_section(
                    subspace.id,
                    section_id,
                    None,
                    Some("new section description"),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Create a new section
        let test_section = create_test_section(&app, &account, subspace.id);

        // Test move section
        subspaces
            .move_section(
                SubspacesMsg::move_section(
                    subspace.id,
                    section_id,
                    test_section.id,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Test delete section
        subspaces
            .delete_section(
                SubspacesMsg::delete_section(
                    subspace.id,
                    section_id,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_group_management() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let subspace = create_test_subspace(&app, &account);
        let test_section = create_test_section(&app, &account, subspace.id);

        // Test group creation
        let group_id = subspaces
            .create_user_group(
                SubspacesMsg::create_user_group(
                    subspace.id,
                    0,
                    "Test group",
                    "Test user group",
                    vec![],
                    vec![],
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .group_id;

        // Test group edit
        subspaces
            .edit_user_group(
                SubspacesMsg::edit_user_group(
                    subspace.id,
                    group_id,
                    Some("new name"),
                    None,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Test move group
        subspaces
            .move_user_group(
                SubspacesMsg::move_user_group(
                    subspace.id,
                    group_id,
                    test_section.id,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Test set group permissions
        subspaces
            .set_user_group_permissions(
                SubspacesMsg::set_user_group_permissions(
                    subspace.id,
                    group_id,
                    vec![Permission::Write, Permission::EditOwnContent],
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Test group delete
        subspaces
            .delete_user_group(
                SubspacesMsg::delete_user_group(
                    subspace.id,
                    group_id,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_add_remove_user_from_group() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let group_member = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let test_subspace = create_test_subspace(&app, &account);
        let test_user_group = create_test_user_group(&app, &account, test_subspace.id, 0);

        // Test add user to group
        subspaces
            .add_user_to_user_group(
                SubspacesMsg::add_user_to_user_group(
                    test_subspace.id,
                    test_user_group.id,
                    Addr::unchecked(group_member.address()),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Test remove user to group
        subspaces
            .remove_user_from_user_group(
                SubspacesMsg::remove_user_from_user_group(
                    test_subspace.id,
                    test_user_group.id,
                    Addr::unchecked(group_member.address()),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_set_user_permissions() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let test_user = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let test_subspace = create_test_subspace(&app, &account);

        // Test set user permissions
        subspaces
            .set_user_permissions(
                SubspacesMsg::set_user_permissions(
                    test_subspace.id,
                    0,
                    Addr::unchecked(test_user.address()),
                    vec![Permission::Everything],
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_subspace_queries() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let test_subspace = create_test_subspace(&app, &account);

        // Test query subspace
        let queried_subspace = subspaces
            .query_subspace(&QuerySubspaceRequest {
                subspace_id: test_subspace.id,
            })
            .unwrap()
            .subspace
            .unwrap();
        assert_eq!(test_subspace, queried_subspace);

        // Test query subspaces
        let queried_subspaces = subspaces
            .query_subspaces(&QuerySubspacesRequest { pagination: None })
            .unwrap()
            .subspaces;
        assert_eq!(vec![queried_subspace], queried_subspaces);
    }

    #[test]
    fn test_section_queries() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let test_subspace = create_test_subspace(&app, &account);
        let test_section = create_test_section(&app, &account, test_subspace.id);

        // Test query section
        let queried_section = subspaces
            .query_section(&QuerySectionRequest {
                subspace_id: test_subspace.id,
                section_id: test_section.id,
            })
            .unwrap()
            .section
            .unwrap();
        assert_eq!(test_section, queried_section);

        // Test query sections
        let queried_sections = subspaces
            .query_sections(&QuerySectionsRequest {
                subspace_id: test_subspace.id,
                pagination: None,
            })
            .unwrap()
            .sections;
        assert_eq!(
            vec![
                Section {
                    subspace_id: 1,
                    id: 0,
                    parent_id: 0,
                    name: "Default section".to_string(),
                    description: "This is the default subspace section".to_string()
                },
                queried_section
            ],
            queried_sections
        );
    }

    #[test]
    fn test_user_group_queries() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let test_subspace = create_test_subspace(&app, &account);
        let test_group = create_test_user_group(&app, &account, test_subspace.id, 0);

        // Test query user group
        let queried_user_group = subspaces
            .query_user_group(&QueryUserGroupRequest {
                subspace_id: test_subspace.id,
                group_id: test_group.id,
            })
            .unwrap()
            .group
            .unwrap();
        assert_eq!(test_group, queried_user_group);

        // Test query user groups
        let queried_user_groups = subspaces
            .query_user_groups(&QueryUserGroupsRequest {
                subspace_id: test_subspace.id,
                section_id: 0,
                pagination: None,
            })
            .unwrap()
            .groups;
        assert_eq!(
            vec![
                UserGroup {
                    subspace_id: 1,
                    section_id: 0,
                    id: 0,
                    name: "Default".to_string(),
                    description:
                        "This is a default user group which all users are automatically part of"
                            .to_string(),
                    permissions: vec![]
                },
                test_group
            ],
            queried_user_groups
        );
    }

    #[test]
    fn test_query_group_members() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let test_account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let test_subspace = create_test_subspace(&app, &account);
        let test_group = create_test_user_group(&app, &account, test_subspace.id, 0);
        subspaces
            .add_user_to_user_group(
                SubspacesMsg::add_user_to_user_group(
                    test_subspace.id,
                    test_group.id,
                    Addr::unchecked(test_account.address()),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Test query group members
        let members = subspaces
            .query_user_group_members(&QueryUserGroupMembersRequest {
                subspace_id: test_subspace.id,
                group_id: test_group.id,
                pagination: None,
            })
            .unwrap()
            .members;
        assert_eq!(vec![test_account.address()], members);
    }

    #[test]
    fn test_query_user_permissions() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let test_account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let subspaces = Subspaces::new(&app);

        // Environment setup
        let test_subspace = create_test_subspace(&app, &account);
        subspaces
            .set_user_permissions(
                SubspacesMsg::set_user_permissions(
                    test_subspace.id,
                    0,
                    Addr::unchecked(test_account.address()),
                    vec![Permission::EditSubspace],
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Test query group members
        let permissions = subspaces
            .query_user_permissions(&QueryUserPermissionsRequest {
                subspace_id: test_subspace.id,
                section_id: 0,
                user: test_account.address(),
            })
            .unwrap()
            .permissions;
        assert_eq!(1, permissions.len());
        let permission_name: String = Permission::EditSubspace.into();
        let queried_permission_name: String = permissions[0].clone().into();
        assert_eq!(permission_name, queried_permission_name);
    }
}
