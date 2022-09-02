mod test;

#[ic_cdk_macros::import(canister = "imported_canister")]
struct ImportedCanister;

#[ic_cdk_macros::update]
#[candid::candid_method(update)]
async fn call_imported_func() -> String {
    ImportedCanister::greet("Alice".to_string()).await.0
}

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    candid::export_service!();
    __export_service()
}
