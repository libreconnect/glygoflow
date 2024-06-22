use lapin::{auth::Credentials, Channel, Connection, ConnectionProperties};

pub struct LapinClient {
    pub conn: Connection,
    pub channel: Channel,
}

impl LapinClient {
    pub async fn new(host: String, port: u16, creds: Credentials) -> lapin::Result<Self> {
        let uri = format!(
            "amqp://{}:{}@{}:{}/%2f",
            creds.username(),
            creds.password(),
            host,
            port
        );

        println!("{}", uri);

        let conn = Connection::connect(&uri, ConnectionProperties::default()).await?;

        let channel = conn.create_channel().await?;

        Ok(Self { conn, channel })
    }
}
