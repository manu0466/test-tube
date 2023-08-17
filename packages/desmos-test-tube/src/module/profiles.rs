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
    use cosmwasm_std::{coin, coins, Addr};
    use desmos_bindings::cosmos_types::Any;
    use desmos_bindings::profiles::msg::ProfilesMsg;
    use desmos_bindings::profiles::types::{
        AddressData, Bech32Address, ChainConfig, MsgAcceptDTagTransferRequest,
        MsgCancelDTagTransferRequest, MsgRefuseDTagTransferRequest, MsgRequestDTagTransfer,
        Profile, Proof, QueryChainLinkOwnersRequest, QueryChainLinksRequest,
        QueryIncomingDTagTransferRequestsRequest, QueryProfileRequest, SignatureValueType,
        SingleSignature,
    };
    use prost::Message;
    use test_tube::{Account, Module, SigningAccount};

    /// Create a profile for the provided [SigningAccount].
    fn create_profile_for_user(app: &DesmosTestApp, account: &SigningAccount) {
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
    }

    /// Creates a new [SigningAccount] and performs a chain link it to connect the newly
    /// generated [SigningAccount] to the provided [SigningAccount].
    fn link_account(app: &DesmosTestApp, account: &SigningAccount) -> SigningAccount {
        let profiles = Profiles::new(app);

        // Create a new account to generate the chain link.
        let new_account = app.init_account(&[]).unwrap();
        let new_account_pub_key = new_account.public_key().to_any().unwrap();
        // Sign the account address to generate the proof.
        let signature = new_account
            .signing_key()
            .sign(account.address().as_bytes())
            .unwrap();
        // Encode the signature object.
        let mut encoded_signature = Vec::<u8>::new();
        SingleSignature {
            value_type: SignatureValueType::Raw.into(),
            signature: signature.to_vec(),
        }
        .encode(&mut encoded_signature)
        .unwrap();

        // Perform the chain link.
        profiles
            .link_chain_account(
                ProfilesMsg::link_chain_account(
                    AddressData::Bech32Address(Bech32Address {
                        value: new_account.address(),
                        prefix: new_account.prefix().to_string(),
                    }),
                    Proof {
                        pub_key: Some(new_account_pub_key.into()),
                        plain_text: hex::encode(account.address()),
                        signature: Some(Any {
                            type_url: "/desmos.profiles.v3.SingleSignature".to_string(),
                            value: encoded_signature,
                        }),
                    },
                    ChainConfig {
                        name: "desmos".to_string(),
                    },
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        return new_account;
    }

    #[test]
    fn test_profile_management() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&coins(100_000_000_000, "udsm")).unwrap();

        let profiles = Profiles::new(&app);
        profiles
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

        // Test profile query.
        let profile = profiles
            .query_profile(&QueryProfileRequest {
                user: account.address(),
            })
            .unwrap()
            .profile
            .unwrap();
        let desmos_profile = Profile::try_from(profile).unwrap();
        assert_eq!("test", desmos_profile.dtag);

        profiles
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
            create_profile_for_user(&app, account);
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

        // Query the transfer request
        let requests = profiles
            .query_incoming_dtag_transfer_requests(&QueryIncomingDTagTransferRequestsRequest {
                receiver: accounts[1].address(),
                pagination: None,
            })
            .unwrap()
            .requests;
        let request = requests.first().unwrap();
        assert_eq!(accounts[0].address(), request.sender);
        let mut dtag_to_trade = accounts[1].address();
        dtag_to_trade.truncate(30);
        assert_eq!(dtag_to_trade, request.dtag_to_trade);

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

    #[test]
    fn test_chain_link_manipulation() {
        let app = DesmosTestApp::new();
        let profiles = Profiles::new(&app);
        let account = app.init_account(&coins(100_000_000_000, "udsm")).unwrap();

        // Create a profile to perform the chain link.
        profiles
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

        // Link the osmosis account.
        let linked_account = link_account(&app, &account);

        // Set the default address.
        profiles
            .set_default_external_address(
                ProfilesMsg::set_default_external_address(
                    "desmos",
                    linked_account.address().as_str(),
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();

        // Unlink the previously linked account.
        profiles
            .unlink_chain_account(
                ProfilesMsg::unlink_chain_account(
                    Addr::unchecked(account.address()),
                    "desmos",
                    linked_account.address().as_str(),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_query_chain_link() {
        let app = DesmosTestApp::new();
        let profiles = Profiles::new(&app);
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();

        create_profile_for_user(&app, &account);
        let linked_account = link_account(&app, &account);

        let links = profiles
            .query_chain_links(&QueryChainLinksRequest {
                user: account.address(),
                chain_name: "desmos".to_string(),
                target: "".to_string(),
                pagination: None,
            })
            .unwrap()
            .links;

        assert_eq!(1, links.len());
        let any_address = links.first().unwrap().address.clone().unwrap();
        let linked_address = Bech32Address::try_from(any_address).unwrap();
        assert_eq!(linked_account.address(), linked_address.value);

        // Try the reverse query
        let owners = profiles
            .query_chain_link_owners(&QueryChainLinkOwnersRequest {
                chain_name: "desmos".to_string(),
                target: linked_account.address(),
                pagination: None,
            })
            .unwrap()
            .owners;

        assert_eq!(1, owners.len());
        assert_eq!(account.address(), owners.first().unwrap().user);
    }
}
