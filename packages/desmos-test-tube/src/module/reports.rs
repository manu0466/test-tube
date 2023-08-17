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
    fn_execute! { pub ddd_reason: MsgAddReason => MsgAddReasonResponse }
    fn_execute! { pub remove_reason: MsgRemoveReason => MsgRemoveReasonResponse }
    fn_execute! { pub update_params: MsgUpdateParams => MsgUpdateParamsResponse }

    // ========== Queries ==========

    fn_query! { pub query_reports ["/desmos.reports.v1.Query/Reports"]: QueryReportsRequest => QueryReportsResponse }
    fn_query! { pub query_report ["/desmos.reports.v1.Query/Report"]: QueryReportRequest => QueryReportResponse }
    fn_query! { pub query_reasons ["/desmos.reports.v1.Query/Reasons"]: QueryReasonsRequest => QueryReasonsResponse }
    fn_query! { pub query_reason ["/desmos.reports.v1.Query/Reason"]: QueryReasonRequest => QueryReasonResponse }
    fn_query! { pub query_params ["/desmos.reports.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse }
}
