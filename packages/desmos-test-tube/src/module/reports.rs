use desmos_bindings::reports::types::{
    MsgAddReason, MsgAddReasonResponse, MsgCreateReport, MsgCreateReportResponse, MsgDeleteReport,
    MsgDeleteReportResponse, MsgRemoveReason, MsgRemoveReasonResponse, MsgSupportStandardReason,
    MsgSupportStandardReasonResponse, MsgUpdateParams, MsgUpdateParamsResponse, QueryParamsRequest,
    QueryParamsResponse, QueryReasonRequest, QueryReasonResponse, QueryReasonsRequest,
    QueryReasonsResponse, QueryReportRequest, QueryReportResponse, QueryReportsRequest,
    QueryReportsResponse,
};
use test_tube::{fn_execute, fn_query, Module, Runner};

/// Reports module.
pub struct Reports<'a, R: Runner<'a>> {
    #[allow(dead_code)]
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Reports<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Reports<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    fn_execute! { pub create_report: MsgCreateReport => MsgCreateReportResponse }
    fn_execute! { pub delete_report: MsgDeleteReport => MsgDeleteReportResponse }
    fn_execute! { pub support_standard_reason: MsgSupportStandardReason => MsgSupportStandardReasonResponse }
    fn_execute! { pub add_reason: MsgAddReason => MsgAddReasonResponse }
    fn_execute! { pub remove_reason: MsgRemoveReason => MsgRemoveReasonResponse }
    fn_execute! { pub update_params: MsgUpdateParams => MsgUpdateParamsResponse }

    // ========== Queries ==========

    fn_query! { pub query_reports ["/desmos.reports.v1.Query/Reports"]: QueryReportsRequest => QueryReportsResponse }
    fn_query! { pub query_report ["/desmos.reports.v1.Query/Report"]: QueryReportRequest => QueryReportResponse }
    fn_query! { pub query_reasons ["/desmos.reports.v1.Query/Reasons"]: QueryReasonsRequest => QueryReasonsResponse }
    fn_query! { pub query_reason ["/desmos.reports.v1.Query/Reason"]: QueryReasonRequest => QueryReasonResponse }
    fn_query! { pub query_params ["/desmos.reports.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse }
}

#[cfg(test)]
mod tests {
    use crate::module::test_utils::*;
    use crate::module::Reports;
    use crate::runner::app::DesmosTestApp;
    use cosmwasm_std::{coin, Addr};
    use desmos_bindings::reports::msg::ReportsMsg;
    use desmos_bindings::reports::types::{
        QueryReasonRequest, QueryReasonsRequest, QueryReportRequest, QueryReportsRequest,
        ReportTarget, UserTarget,
    };
    use test_tube::{Account, Module};

    #[test]
    fn test_create_delete_report() {
        let app = DesmosTestApp::new();
        let accounts = app
            .init_accounts(&[coin(100_000_000_000, "udsm")], 2)
            .unwrap();
        let account_0 = accounts.get(0).unwrap();
        let account_1 = accounts.get(1).unwrap();
        let reports = Reports::new(&app);

        // Setup the test environment
        let subspace = create_test_subspace(&app, account_0);
        create_test_profile(&app, account_0);

        // Create a new reason
        let reason_id = reports
            .add_reason(
                ReportsMsg::add_reason(
                    subspace.id,
                    "Test reason",
                    "Test reason",
                    Addr::unchecked(account_0.address()),
                ),
                account_0,
            )
            .unwrap()
            .data
            .reason_id;

        // Create a user report
        let report_id = reports
            .create_report(
                ReportsMsg::create_report(
                    subspace.id,
                    vec![reason_id],
                    "Test",
                    Addr::unchecked(account_0.address()),
                    ReportTarget::User(UserTarget {
                        user: account_1.address(),
                    }),
                ),
                account_0,
            )
            .unwrap()
            .data
            .report_id;

        // Delete the report
        reports
            .delete_report(
                ReportsMsg::delete_report(
                    subspace.id,
                    report_id,
                    Addr::unchecked(account_0.address()),
                ),
                account_0,
            )
            .unwrap();
    }

    #[test]
    fn test_support_standard_reason() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let reports = Reports::new(&app);

        // Setup the test environment
        let subspace = create_test_subspace(&app, &account);

        // Test support standards reason
        reports
            .support_standard_reason(
                ReportsMsg::support_standard_reason(
                    subspace.id,
                    1,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_add_remove_reason() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let reports = Reports::new(&app);

        // Setup the test environment
        let subspace = create_test_subspace(&app, &account);

        // Create a new reason
        let reason_id = reports
            .add_reason(
                ReportsMsg::add_reason(
                    subspace.id,
                    "Test reason",
                    "Test reason",
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .reason_id;

        // Test the remove reason
        reports
            .remove_reason(
                ReportsMsg::remove_reason(
                    subspace.id,
                    reason_id,
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap();
    }

    #[test]
    fn test_report_queries() {
        let app = DesmosTestApp::new();
        let accounts = app
            .init_accounts(&[coin(100_000_000_000, "udsm")], 2)
            .unwrap();
        let account_0 = accounts.get(0).unwrap();
        let account_1 = accounts.get(1).unwrap();
        let reports = Reports::new(&app);

        // Setup the test environment
        let subspace = create_test_subspace(&app, account_0);
        create_test_profile(&app, account_0);

        // Create a new reason
        let reason_id = reports
            .add_reason(
                ReportsMsg::add_reason(
                    subspace.id,
                    "Test reason",
                    "Test reason",
                    Addr::unchecked(account_0.address()),
                ),
                account_0,
            )
            .unwrap()
            .data
            .reason_id;

        // Create a user report
        let report_id = reports
            .create_report(
                ReportsMsg::create_report(
                    subspace.id,
                    vec![reason_id],
                    "Test",
                    Addr::unchecked(account_0.address()),
                    ReportTarget::User(UserTarget {
                        user: account_1.address(),
                    }),
                ),
                account_0,
            )
            .unwrap()
            .data
            .report_id;

        // Test query report
        let created_report = reports
            .query_report(&QueryReportRequest {
                subspace_id: subspace.id,
                report_id,
            })
            .unwrap()
            .report
            .unwrap();
        assert_eq!(report_id, created_report.id);
        assert_eq!(vec![reason_id], created_report.reasons_ids);
        assert_eq!("Test", created_report.message);

        // Test query reports
        let queried_reports = reports
            .query_reports(&QueryReportsRequest {
                subspace_id: subspace.id,
                target: None,
                reporter: "".to_string(),
                pagination: None,
            })
            .unwrap()
            .reports;
        assert_eq!(vec![created_report], queried_reports);
    }

    #[test]
    fn test_reason_queries() {
        let app = DesmosTestApp::new();
        let account = app.init_account(&[coin(100_000_000_000, "udsm")]).unwrap();
        let reports = Reports::new(&app);

        // Setup the test environment
        let subspace = create_test_subspace(&app, &account);

        // Create a new reason
        let reason_id = reports
            .add_reason(
                ReportsMsg::add_reason(
                    subspace.id,
                    "Test reason",
                    "Test reason",
                    Addr::unchecked(account.address()),
                ),
                &account,
            )
            .unwrap()
            .data
            .reason_id;

        // Test query reason
        let queried_reason = reports
            .query_reason(&QueryReasonRequest {
                subspace_id: subspace.id,
                reason_id,
            })
            .unwrap()
            .reason
            .unwrap();
        assert_eq!(reason_id, queried_reason.id);
        assert_eq!(subspace.id, queried_reason.subspace_id);
        assert_eq!("Test reason", queried_reason.title);
        assert_eq!("Test reason", queried_reason.description);

        // Test query reasons
        let queried_reasons = reports
            .query_reasons(&QueryReasonsRequest {
                subspace_id: subspace.id,
                pagination: None,
            })
            .unwrap()
            .reasons;
        assert_eq!(vec![queried_reason], queried_reasons);
    }
}
