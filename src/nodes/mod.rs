pub mod trigger;
pub mod http;
pub mod function;

pub use trigger::TriggerFactory;
pub use http::HttpFactory;
pub use function::FunctionFactory;

// use crate::traits::NodeExecutorFactory;

// pub struct Plugins;

// impl Plugins {
    // pub fn get_available_plugins() -> Vec<Box<dyn NodeExecutorFactory>> {
    //     vec![
    //         Box::new(TriggerFactory),
    //         Box::new(HttpFactory),
    //         Box::new(FunctionFactory),
    //     ]
    // }
// }
