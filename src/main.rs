use serde_json::Value;
use serde_json::from_value;
use solana_transaction_status::UiConfirmedBlock;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
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

    loop {
        // Open the JSON file
        let file = File::open("block.json")?;
        let reader = BufReader::new(file);
        let json_obj: Value = serde_json::from_reader(reader).unwrap();

        let start = Instant::now();
        // Save the json data into surrealdb
        let _json_data: Option<Value> = match db.create("json_data").content(json_obj.clone()).await
        {
            Ok(result) => result,
            Err(e) => {
                // eprintln!("Error: {}", e);
                None
            }
        };
        let duration = start.elapsed();
        println!("0000: Time elapsed: {:?}", duration);

        // Deserialize into a block type
        let block_data: UiConfirmedBlock = from_value(json_obj).unwrap();

        let start = Instant::now();
        // Save the block data into surrealdb
        let _block: Option<UiConfirmedBlock> =
            match db.create("block_data").content(block_data).await {
                Ok(result) => result,
                Err(e) => {
                    // eprintln!("Error: {}", e);
                    None
                }
            };
        let duration = start.elapsed();
        println!("1111: Time elapsed: {:?}", duration);
    }

    Ok(())
}
