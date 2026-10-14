mod model; mod service; mod routes; mod error;

use summer::{auto_config, App};
use summer_sqlx::SqlxPlugin;
use summer_web::{WebConfigurator, WebPlugin};

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(SqlxPlugin)
        .add_plugin(WebPlugin)
        .run()
        .await
}