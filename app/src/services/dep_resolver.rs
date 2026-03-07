use tauri::AppHandle;

pub struct DependencyResolver(AppHandle);

impl DependencyResolver {
    pub fn new(app_handle: AppHandle) -> Self {
        Self(app_handle)
    }
}