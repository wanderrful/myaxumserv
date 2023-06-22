use std::sync::Arc;

use axum::extract::FromRef;

use crate::modules::api::ApiModule;

#[derive(Clone)]
pub(crate) struct ApiState {
    pub(crate) module: Arc<ApiModule>
}

impl FromRef<ApiState> for Arc<ApiModule> {
    fn from_ref(api_state: &ApiState) -> Arc<ApiModule> {
        api_state.module.clone()
    }
}
