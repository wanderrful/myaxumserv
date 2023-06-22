use shaku::module;

use crate::accessors::user::UserAccessorImpl;
use crate::clients::local_db::LocalDbClientImpl;
use crate::resources::user::UserResourceImpl;
use crate::managers::user::UserManagerImpl;

module! {
    pub(crate) ApiModule {
        components = [ LocalDbClientImpl, UserAccessorImpl, UserManagerImpl, UserResourceImpl],
        providers = [],
    }
}
