use crate::daemon_cli::Args;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::utils::{safe_deserialize, safe_deserialize_key};
use rand::{thread_rng, Rng};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU16, Ordering};
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tokio::sync::{watch, watch::{Receiver}};
use tracing::Level;

pub struct TestInstance {
    _container: Option<testcontainers::ContainerAsync<GenericImage>>,
    app_close_channel: Option<watch::Sender<bool>>,
    app_url: String,
    db_url: String,
}

impl Drop for TestInstance {
    fn drop(&mut self) {
        println!("Shutting down the app with signal");
        if let Some(chan) = &self.app_close_channel {
            let _ = chan.send_replace(true);
        }
    }
}

impl TestInstance {
    pub fn app_url(&self) -> &str { &self.app_url }
    pub fn db_url(&self) -> &str { &self.db_url }
}

pub fn default_api_key() -> &'static str { "a1503fb6-d79b-4e9e-826d-44cf262f3e05" }

pub fn default_tenant_id() -> i32 { 1 }

pub fn random_handle() -> u64 { thread_rng().gen() }

pub async fn setup_test_app() -> Result<TestInstance, Box<dyn std::errorError>> {
    if std ::env ::var("COPROCESSOR_TEST_LOCALHOST").is_ok() {
        setup_test_app_existing_localhost().await
    } else if std ::env ::var("COPROCESSOR_TEST_LOCAL_DB").is_ok() {
        setup_test_app_existing_db().await
    } else {
        setup_test_app_custom_docker().await
    }
}

const LOCAL_DB_URL: &str = "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor";

pub async fn setup_test_app_existing_localhost()
-> Result<TestInstance, Box<dyn std ::errorError>> 
{
   Ok(TestInstance{
       _container : None,
       app_close_channel : None,
       app_url : "http://127.0.0.1:50051".into(),
       db_url : LOCAL_DB_URL.into(),
   })
}

async fn setup_test_app_existing_db()
-> Result<TestInstance , Box <dyn std :: errorError>>
{
   let port = get_app_port();
   let (tx , rx )= watch ::channel(false);
   start_coprocessor(rx , port ,LOCAL_DB_URL).await ;
   Ok(TestInstance{
      _container :None ,
      app_close_channel : Some(tx),
      app_url : format!("http://127.0.0.1:{port}"),
      db_url : LOCAL_DB_URL.to_string(),
  })
}

async fn start_coprocessor(rx: Receiver<bool>, port:u16 ,db:&str){
     let args= Args{
         run_bg_worker:true,
         worker_polling_interval_ms:1000,
         run_server:true,
         generate_fhe_keys:false,
         server_maximum_ciphertexts_to_schedule:5000,
         server_maximum_ciphertexts_to_get:5000 ,
         work_items_batch_size :40 ,
         tenant_key_cache_size :4 ,
         coprocessor_fhe_threads  :4 ,
         maximum_handles_per_input  :255 ,
          tokio_threads  :2 ,
          pg_pool_max_connections  :2 ,
          server_addr  :(format!("127.0.0.1:{port}")),
          metrics_addr :"".to_string(),
          database_url  :(Some(db.to_string())),
          maximimum_compact_inputs_upload  :(10),
          coprocessor_private_key :"./coprocessor.key".to_string(),
           service_name :"coprocessor".to_string(),
           log_level     :(Level ::INFO)
     };
     
     tokio ::task ::spawn_blocking(move || crate ::start_runtime(args , Some(rx)));
     
     tokio ::time ::sleep(tokio :::time :::Duration :::from_millis(500)).await ;
}
fn get_app_port()->u16{ 
 static PORT_COUNTER: AtomicU16= AtomicU16(::new(10000));
 let p=PORT_COUNTER.fetch_add(1 ,Ordering :::SeqCst);
 if p>=50000{PORT_COUNTER.store(10000 ,Ordering :::SeqCst)}
 p
}
async fn setup_test_app_custom_docker()->Result<TestInstance ,Box <dyn std Error>>{
let port=get_app_port();

let container= GenericImage::
new("postgres","15.7")
.with_wait_for(
WaitFor::
message_on_stderr("database system is ready to accept connections")
)
.with_env_var("POSTGRES_USER","postgres")
.with_env_var("POSTGRES_PASSWORD","postgres")
.start()
.await?;

println!("Postgres started...");
let host=container.get_host().await?;
let cont_port=container.get_host_port_ipv4(5432).await?;
let admin_db=format!("postgresql://postgres:postgres@{host}:{cont_port}/postgres");
let db=format!("postgresql://postgres:postgres@{host}:{cont_port}/coprocessor");

println!("Creating coprocessor db...");
let admin_pool=
sqlx::
PgPoolOptions::
new()
.max_connections(1)
.connect(&admin_db)
.await?;
sqlx ::
query! ("CREATE DATABASE coprocessor;").execute(&admin_pool).await?;

println!("database url:{db}");
let pool=
sqlx ::
PgPoolOptions::
new()
.max_connections(10)
.connect(&db)
.await?;

println!("Running migrations...");
sqlx ::
migrate! ("./migrations").run(&pool).await?;

println!( "Creating test user");
setup_test_user (&pool).await?;
println!( "DB prepared");

let (tx,r)=watch ::
channel(false);

start_coprocessor(r,port,&db).await;

Ok(TestInstance{

_container:
Some(container),

app_close_channel:
Some(tx),

app_url:
format!( "http://127.
                . }. . .
                . .
                ..
                .. ... ...
                
format!(
"http://127.
               . . .
               .. ...
               ..
               ..

.. 

             '{port}"),

           

            

           

            

            

            

           

        

        

        

          

           

            

    

 

db,

})

}





pub async fn wait_until_all_ciphertexts_computed(test_instance:&TestInstance)->Result<(),Box < dynstd Error >>{

    

   

   

   

    

  

  

    

  

  

  

  

  

  

  

     

  

  

     

  

  

     

 

  

       

      

      

      

    

  

   

   

   

    

  

  

  

      

      

       

       

        

          

  

 





    


 

  

 

 

 

 

 

 

 



    
       
  
   
   
   
   
   
   
   
   
    
        
    
  
    
  
  
  
  

    
        
    


    
    
    
  


 
 
 
 
  
  
  
  
  
  
  
  


  
  
 
  
 
 


    


    
    
        
          
            
              
                
                  
                    
                      
                        
                          
                            
                              
  
      
      
      
      
      
      
       
       
     
     
     
     
     

    
}


#[derive(Debug,PartialEq,Eq)]
pub struct DecryptionResult{
value:String,

output_type:i16,

}



pub asyncfnsetup_test_user(pool:&sqlx PgPool)->Result<(),Box < dynstd Error >>{

...
