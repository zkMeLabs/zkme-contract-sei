use cosmwasm_schema::write_api;
use zkme_sbt::msg::{ExecuteMsg, InitMsg, QueryMsg};

fn main() {
    write_api!(instantiate: InitMsg, execute: ExecuteMsg, query: QueryMsg,)
}
