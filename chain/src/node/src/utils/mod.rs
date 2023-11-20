use crate::env::ENV;
use utils::commandline::print_banner;

pub fn display_banner_on_startup(node_id: &str, env: &ENV) {
    print_banner(Some(vec![
        vec!["Node Version", env!("CARGO_PKG_VERSION")],
        vec!["Node ID", node_id],
        vec!["Log Level", &env.rust_log],
    ]))
}
