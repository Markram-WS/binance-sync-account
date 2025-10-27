
use sqlx;
use sqlx::postgres::PgPool;

#[derive(Debug)]
pub struct Params<'a> {
    balances_free :  &'a f64,
    sqlx:&'a sqlx::Pool<sqlx::Postgres>

}
impl<'a> Params<'a> {
    #[allow(dead_code)]
    pub fn new(balances_free:  &'a f64 ,order_id :  &'a str ) -> Self {

        
    }
}

pub async fn insert<'a>(payload: Params<'a>){

}


