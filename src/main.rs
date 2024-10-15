pub mod redlib {
    pub mod client;
}

pub mod settings {
    pub mod params;
}

use redlib::client::RedisClient;
use settings::params;

fn main() {
    let cli_obj = RedisClient::new();
    let client_set: RedisClient;

    match cli_obj {
        Ok(cli_res) => client_set = cli_res,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    }

    let res = client_set.set("key", "val");
    match res {
        Ok(_) => println!("key saved"),
        Err(e) => eprintln!("Error: {}", e),
    }

    let cli_obj = RedisClient::new();
    let mut client_get: RedisClient;

    match cli_obj {
        Ok(c) => client_get = c,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    }

    let out = client_get.get("key");
    match out {
        Ok(v) => println!("Value: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
