pub trait ImagePlugin: Send + Sync {
    fn supported_extensions(&self) -> Vec<String>;
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "Rust" fn new_plugin() -> Box<dyn ImagePlugin + Send + Sync> {
            Box::new($constructor())
        }
    };
}