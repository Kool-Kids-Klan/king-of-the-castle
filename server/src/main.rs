use kotc_actix::start_actix_server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    start_actix_server().await?;

    Ok(())
}
