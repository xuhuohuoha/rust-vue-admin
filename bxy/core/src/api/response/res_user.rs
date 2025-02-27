use sea_orm::FromQueryResult;

pub struct UserRes {}

#[derive(FromQueryResult)]
pub struct PartialUserAuth {
    pub id: String,
    pub mcode: String,
}
