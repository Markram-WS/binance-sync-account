
use sqlx::postgres::PgPool;

use crate::utils::{get_env, get_env_decode};
#[allow(unused_imports)]
use sqlx::Row ;


pub async fn create()  -> Result<sqlx::Pool<sqlx::Postgres>,sqlx::Error> {
    let db_host = get_env("DB_HOST");
    let db_user_decode = get_env_decode("POSTGRES_USER");
    let db_password_decode = get_env_decode("POSTGRES_PASSWORD");
        
    let db_port = get_env("DB_PORT");
    let db_stage = get_env("STAGE");
    
    let db_url = format!("postgresql://{}:{}@{}:{}/{}",&db_user_decode,&db_password_decode,&db_host,&db_port,&db_stage);
    
    //------
    let db_app_id =  get_env("APP_ID") ;
    let db_suffix_schema = get_env("SUFFIX") ;
    let asset = get_env("SYMBOL");
        
    let schema = format!("{}{}_{}",&asset,&db_suffix_schema,&db_app_id[..5]);
    println!( "schema : {}",&schema );
    println!( "url : {}",&db_url );
    let pool: sqlx::Pool<sqlx::Postgres> = PgPool::connect(&db_url).await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    //get_env
    #[tokio::test]
    async  fn test_postgres_create(){
        unsafe {
            env::set_var("DB_HOST", "localhost");
            env::set_var("POSTGRES_USER", "dGVzdA==");
            env::set_var("POSTGRES_PASSWORD", "dGVzdA==");
            env::set_var("DB_PORT", "10776");
            env::set_var("STAGE", "dev");
            env::set_var("APP_ID", "PuxFgnIYYZcpjyzD");
            env::set_var("SUFFIX", "");
            env::set_var("SYMBOL", "usd");
        };

        let pool = create().await.unwrap(); 
        // Dynamic query
        let result = sqlx::query("SELECT 1 = 1 as result")
        .fetch_one(&pool) 
        .await;
        match result {
            Ok(row) => {
                let val: bool = row.get("result"); 
                println!("val: {}", &val);
                assert!(&val,"True")
            }
            Err(e) => {
                eprintln!("Query failed: {}", e);
            }
        }
    }
}

