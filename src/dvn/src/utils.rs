pub fn guard_caller_is_controller() -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    if !ic_cdk::api::is_controller(&caller) {
        Err("Caller not a controller, unauthorized".to_string())
    } else {
        Ok(())
    }
}