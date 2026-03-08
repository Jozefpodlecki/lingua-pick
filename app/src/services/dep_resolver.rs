use tauri::{AppHandle, Manager};

pub struct StateGuard<'r, T: Send + Sync + 'static>(tauri::State<'r, T>);

impl<T: Send + Sync + 'static> std::ops::Deref for StateGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Clone)]
pub struct DependencyResolver(AppHandle);

impl DependencyResolver {
    pub fn new(app_handle: AppHandle) -> Self {
        Self(app_handle)
    }
    
    pub fn get<T>(&self) -> StateGuard<'_, T> where T: Send + Sync + 'static {
        StateGuard(self.0.state::<T>())
    }
}