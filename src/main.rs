use serde_json::{Value, from_value};
use solana_transaction_status::UiConfirmedBlock;
use std::fs::File;
use std::io::BufReader;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("test").use_db("test").await?;

    // Open the JSON file
    let file = File::open("block.json")?;
    let reader = BufReader::new(file);
    let json_obj: Value = serde_json::from_reader(reader).unwrap();

    // TODO; Writes to the db fine but fails to return the Option<Value>
    // Error: Db(Serialization("invalid type: enum, expected any valid JSON value"))
    // let value: Option<Value> = db.create("json_data").content(json_obj.clone()).await?;

    // Deserialize into a block type
    let block_data: UiConfirmedBlock = from_value(json_obj).unwrap();

    // TODO; Write to the dbs fine but fails to return the Option<UiConfirmedBlock>
    // Error: Db(Serialization("failed to deserialize; expected an enum variant of Result, found {  }"))
    let block: Option<UiConfirmedBlock> = db.create("block_data").content(block_data).await?;

    Ok(())
}
