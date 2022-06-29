use {
    actix_web::HttpResponse,
    actix_web::web::Json,

    crate::miner::*,
    crate::util::*,
};

// list all miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    // get all MinerDAO objects from DB and convert to Miner object
    let miners: Vec<Miner> = vec![];
    ResponseType::Ok(miners).get_response()
}

// get a miner by id
#[get("miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    // get the MinerDAO from DB where id = requested id and convert to Miner object
    let miner: Option<Miner> = None;
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Miner not found.".to_string())
        ).get_response(),
    }
}

// create a new miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner(miner_request: Json<NewMinerRequest>) -> HttpResponse {
    // create a new MinerDAO object from requested inputs and write to DB
    let miner: Vec<Miner> = vec![];
    ResponseType::Created(miner).get_response()
}