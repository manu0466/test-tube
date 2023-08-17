use desmos_bindings::profiles::types::{
    MsgAcceptDTagTransferRequest, MsgAcceptDTagTransferRequestResponse,
    MsgCancelDTagTransferRequest, MsgCancelDTagTransferRequestResponse, MsgDeleteProfile,
    MsgDeleteProfileResponse, MsgLinkApplication, MsgLinkApplicationResponse, MsgLinkChainAccount,
    MsgLinkChainAccountResponse, MsgRefuseDTagTransferRequest,
    MsgRefuseDTagTransferRequestResponse, MsgRequestDTagTransfer, MsgRequestDTagTransferResponse,
    MsgSaveProfile, MsgSaveProfileResponse, MsgSetDefaultExternalAddress,
    MsgSetDefaultExternalAddressResponse, MsgUnlinkApplication, MsgUnlinkApplicationResponse,
    MsgUnlinkChainAccount, MsgUnlinkChainAccountResponse, MsgUpdateParams, MsgUpdateParamsResponse,
    QueryApplicationLinkByClientIdRequest, QueryApplicationLinkByClientIdResponse,
    QueryApplicationLinkOwnersRequest, QueryApplicationLinkOwnersResponse,
    QueryApplicationLinksRequest, QueryApplicationLinksResponse, QueryChainLinkOwnersRequest,
    QueryChainLinkOwnersResponse, QueryChainLinksRequest, QueryChainLinksResponse,
    QueryDefaultExternalAddressesRequest, QueryDefaultExternalAddressesResponse,
    QueryIncomingDTagTransferRequestsRequest, QueryIncomingDTagTransferRequestsResponse,
    QueryProfileRequest, QueryProfileResponse,
};
use test_tube::{fn_execute, fn_query, Module, Runner};

/// Profiles module.
pub struct Profiles<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Profiles<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Profiles<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    fn_execute! { pub save_profile: MsgSaveProfile => MsgSaveProfileResponse }
    fn_execute! { pub delete_profile: MsgDeleteProfile => MsgDeleteProfileResponse }
    fn_execute! { pub request_dtag_transfer: MsgRequestDTagTransfer => MsgRequestDTagTransferResponse }
    fn_execute! { pub cancel_dtag_transfer_request: MsgCancelDTagTransferRequest => MsgCancelDTagTransferRequestResponse }
    fn_execute! { pub accept_dtag_transfer_request: MsgAcceptDTagTransferRequest => MsgAcceptDTagTransferRequestResponse }
    fn_execute! { pub refuse_dtag_transfer_request: MsgRefuseDTagTransferRequest => MsgRefuseDTagTransferRequestResponse }
    fn_execute! { pub link_chain_account: MsgLinkChainAccount => MsgLinkChainAccountResponse }
    fn_execute! { pub unlink_chain_account: MsgUnlinkChainAccount => MsgUnlinkChainAccountResponse }
    fn_execute! { pub set_default_external_address: MsgSetDefaultExternalAddress => MsgSetDefaultExternalAddressResponse }
    fn_execute! { pub link_application: MsgLinkApplication => MsgLinkApplicationResponse }
    fn_execute! { pub unlink_application: MsgUnlinkApplication => MsgUnlinkApplicationResponse }
    fn_execute! { pub update_params: MsgUpdateParams => MsgUpdateParamsResponse }

    // ========== Queries ==========

    fn_query! {
        query_profile ["/desmos.profiles.v3.Query/Profile"]: QueryProfileRequest => QueryProfileResponse
    }
    fn_query! {
        query_incoming_dtag_transfer_requests ["/desmos.profiles.v3.Query/IncomingDTagTransferRequests"]: QueryIncomingDTagTransferRequestsRequest => QueryIncomingDTagTransferRequestsResponse
    }
    fn_query! {
        query_chain_links ["/desmos.profiles.v3.Query/ChainLinks"]: QueryChainLinksRequest => QueryChainLinksResponse
    }
    fn_query! {
        query_chain_link_owners ["/desmos.profiles.v3.Query/ChainLinkOwners"]: QueryChainLinkOwnersRequest => QueryChainLinkOwnersResponse
    }
    fn_query! {
        query_default_external_addresses ["/desmos.profiles.v3.Query/DefaultExternalAddresses"]: QueryDefaultExternalAddressesRequest => QueryDefaultExternalAddressesResponse
    }
    fn_query! {
        query_application_links ["/desmos.profiles.v3.Query/ApplicationLinks"]: QueryApplicationLinksRequest => QueryApplicationLinksResponse
    }
    fn_query! {
        query_application_link_by_client_id ["/desmos.profiles.v3.Query/ApplicationLinkByClientID"]: QueryApplicationLinkByClientIdRequest => QueryApplicationLinkByClientIdResponse
    }
    fn_query! {
        query_application_link_owners ["/desmos.profiles.v3.Query/ApplicationLinkOwners"]: QueryApplicationLinkOwnersRequest => QueryApplicationLinkOwnersResponse
    }
}

#[cfg(test)]
mod tests {
    use crate::module::Profiles;
    use crate::runner::app::DesmosTestApp;
    use cosmwasm_std::{coins, Addr};
    use desmos_bindings::profiles::msg::ProfilesMsg;
    use desmos_bindings::profiles::types::{
        MsgAcceptDTagTransferRequest, MsgCancelDTagTransferRequest, MsgRefuseDTagTransferRequest,
        MsgRequestDTagTransfer,
    };
    use test_tube::{Account, Module};

    #[test]
    fn test_profile_managment() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&coins(100_000_000_000, "udsm")).unwrap();

        let profiles = Profiles::new(&app);
        let _response = profiles
            .save_profile(
                ProfilesMsg::save_profile(
                    Some("test"),
                    None,
                    None,
                    None,
                    None,
                    Addr::unchecked(&account.address()),
                ),
                &account,
            )
            .unwrap();

        let _response = profiles
            .delete_profile(
                ProfilesMsg::delete_profile(Addr::unchecked(&account.address())),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_successful_dtag_transfer() {
        let app = DesmosTestApp::new();
        let profiles = Profiles::new(&app);
        let accounts = app
            .init_accounts(&coins(100_000_000_000, "udsm"), 2)
            .unwrap();

        // Create the profile for each account
        accounts.iter().for_each(|account| {
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
        });

        // Test DTag transfer from account 1 to account 0
        profiles
            .request_dtag_transfer(
                MsgRequestDTagTransfer {
                    sender: (&accounts[0]).address(),
                    receiver: (&accounts[1]).address(),
                },
                &accounts[0],
            )
            .unwrap();

        // Accept the transfer
        profiles
            .accept_dtag_transfer_request(
                MsgAcceptDTagTransferRequest {
                    new_dtag: "test".to_string(),
                    sender: (&accounts[0]).address(),
                    receiver: (&accounts[1]).address(),
                },
                &accounts[1],
            )
            .unwrap();
    }

    #[test]
    fn test_cancel_dtag_transfer() {
        let app = DesmosTestApp::new();
        let profiles = Profiles::new(&app);
        let accounts = app
            .init_accounts(&coins(100_000_000_000, "udsm"), 2)
            .unwrap();

        // Create the profile for each account
        accounts.iter().for_each(|account| {
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
        });

        // Test DTag transfer from account 1 to account 0
        profiles
            .request_dtag_transfer(
                MsgRequestDTagTransfer {
                    sender: (&accounts[0]).address(),
                    receiver: (&accounts[1]).address(),
                },
                &accounts[0],
            )
            .unwrap();

        // Cancel the transfer
        profiles
            .cancel_dtag_transfer_request(
                MsgCancelDTagTransferRequest {
                    sender: (&accounts[0]).address(),
                    receiver: (&accounts[1]).address(),
                },
                &accounts[0],
            )
            .unwrap();
    }

    #[test]
    fn test_refuse_dtag_transfer() {
        let app = DesmosTestApp::new();
        let profiles = Profiles::new(&app);
        let accounts = app
            .init_accounts(&coins(100_000_000_000, "udsm"), 2)
            .unwrap();

        // Create the profile for each account
        accounts.iter().for_each(|account| {
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
        });

        // Test DTag transfer from account 1 to account 0
        profiles
            .request_dtag_transfer(
                MsgRequestDTagTransfer {
                    sender: (&accounts[0]).address(),
                    receiver: (&accounts[1]).address(),
                },
                &accounts[0],
            )
            .unwrap();

        // Refuse the transfer
        profiles
            .refuse_dtag_transfer_request(
                MsgRefuseDTagTransferRequest {
                    sender: (&accounts[0]).address(),
                    receiver: (&accounts[1]).address(),
                },
                &accounts[1],
            )
            .unwrap();
    }
}
