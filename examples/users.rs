extern crate dropbox;

use dropbox::models::users::*;
use dropbox::Dropbox;
use hyper::rt::Future;

static KEY: &str = "this_is_my_auth_key";
static ACCOUNT_ID = "dbid:this_is_my_accout_id";

fn main() {
    let dbx = Dropbox::new(KEY.to_owned()).unwrap();
    
    
    println!("Users get_account:");
    let res1 = dbx
        .users
        .get_account(GetAccountArg {
            account_id: String::from(ACCOUNT_ID),
        }).unwrap()
        .map(|su| {
            println!("{:#?}", su);
        }).map_err(|e| {
            eprintln!("error: {:#?}", e);
        });
    hyper::rt::run(res1);

    println!("Users get_account_batch:");
    let res2 = dbx
        .users
        .get_account_batch(GetAccountBatchArg {
            account_ids: vec![String::from(ACCOUNT_ID)],
        }).unwrap()
        .map(|su| {
            println!("{:#?}", su);
        }).map_err(|e| {
            eprintln!("error: {:#?}", e);
        });
    hyper::rt::run(res2);

    println!("Users get_current_account:");
    let res3 = dbx
        .users
        .get_current_account()
        .unwrap()
        .map(|su| {
            println!("{:#?}", su);
        }).map_err(|e| {
            eprintln!("error: {:#?}", e);
        });
    hyper::rt::run(res3);

    println!("Users get_space_usage:");
    let res4 = dbx
        .users
        .get_space_usage()
        .unwrap()
        .map(|su| {
            println!("{:#?}", su);
        }).map_err(|e| {
            eprintln!("error: {:#?}", e);
        });
    hyper::rt::run(res4);
}
