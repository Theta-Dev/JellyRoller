use reqwest::blocking::Client;
use reqwest::StatusCode;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct ResetPass {
    username: String,
    newpw: String,
    server_url: String,
    api_key: String
}

impl ResetPass {
    pub fn new(username: String, newpw: String, server_url: String, api_key: String) -> ResetPass{
        ResetPass{
            username: username.clone(),
            newpw,
            server_url: format!("{}/Users/{}/Password", server_url, username),
            api_key
        }
    }
    
    pub fn reset(self)  -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .post(self.server_url.to_string())
            .header(CONTENT_TYPE, "application/json")
            .header("X-Emby-Token", self.api_key.to_string())
            .body(serde_json::to_string_pretty(&self).unwrap())
            .send()?;
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Password updated successfully.");
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserAdd {
    name: String,
    password: String,
    server_url: String,
    api_key: String
}

impl UserAdd {
    pub fn new(username: String, password: String, server_url: String, api_key: String) -> UserAdd{
        UserAdd{
            name: username,
            password,
            server_url: format!("{}/Users/New",server_url),
            api_key
        }
    }

    pub fn create(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .post(self.server_url.to_string())
            .header("X-Emby-Token", &self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string_pretty(&self).unwrap())
            .send()?;
        match response.status() {
            StatusCode::OK => {
                println!("User \"{}\" successfully created.", &self.name);
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }
        
        Ok(())
    }
}

pub struct UserDel {
    server_url: String,
    api_key: String,
    username: String
}

impl UserDel {
    pub fn new(username: String, server_url: String, api_key: String) -> UserDel{
        UserDel{
            server_url: format!("{}/Users/{}",server_url,&username),
            api_key,
            username
        }
    }

    pub fn remove(self) -> Result<(), reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .delete(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .send()?;
            match response.status() {
            StatusCode::NO_CONTENT => {
                // let body: Value = response.json()?;
                // println!("{:#}", body);
                println!("User \"{}\" successfully removed.", &self.username);
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(())
        
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ServerId")]
    serverid: String,
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Policy")]
    policy: Policy
}

#[derive(Serialize, Deserialize)]
pub struct Policy {
    #[serde(rename = "AuthenticationProviderId")]
    auth_provider_id: String,
    #[serde(rename = "PasswordResetProviderId")]
    pass_reset_provider_id: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuthJson {
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "ServerId")]
    pub server_id: String,
}

pub type UserInfoVec = Vec<User>;

#[derive(Serialize, Deserialize)]
pub struct UserAuth {
    server_url: String,
    username: String,
    pw: String
}

impl UserAuth {
    pub fn new(server_url: String, username: String, password: String) -> UserAuth{
        UserAuth{ 
            server_url: format!("{}/Users/authenticatebyname",server_url),
            username, 
            pw: password
        }
    }
    
    pub fn auth_user(self) -> Result<String, reqwest::Error> {  
        let client = Client::new();
        let response = client
            .post(self.server_url.to_string())
            .header(CONTENT_TYPE, "application/json")
            .header("X-Emby-Authorization", "MediaBrowser Client=\"JellyRoller\", Device=\"jellyroller\", DeviceId=\"1\", Version=\"0.0.1\"")
            .body(serde_json::to_string_pretty(&self).unwrap())
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let result = response.json::<UserAuthJson>().unwrap();
                println!("User authenticated successfully.");
                return Ok(result.access_token)
            } _ => {
                // Panic since the application requires an authenticated user
                panic!("[ERROR] Unable to authenticate user.  Please assure your configuration information is correct.\n");
            }
        }
    }
}

#[derive(Clone)]
pub struct UserList {
    server_url: String,
    api_key: String
}

impl UserList {
    pub fn new(endpoint: String, server_url: String, api_key: String) -> UserList{
        UserList{
            server_url: format!("{}{}",server_url, endpoint),
            api_key
        }
    }

    pub fn list_users(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .send()?;
        let users = response.json::<UserInfoVec>().unwrap();
        println!("Current users:");
        for user in users {
            println!("\t{}", user.name);
        }
        Ok(())
    }

    pub fn get_user_id(self, username: &String) -> String {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .send()
            .unwrap();
        let users = response.json::<UserInfoVec>().unwrap();
        for user in users {
            if user.name == *username {
                return user.id;
            }
        }

        // Supplied username could not be found.  Panic.
        panic!("Could not find user {}.", username);
    }

    pub fn update_user_config_bool(self, id: String, config_key: String, config_value: bool, username: String, req_json_keys: Vec<String>, req_json_values: Vec<String>) -> Result<(), reqwest::Error> {
        let body = json!({
            &config_key:config_value, 
            &req_json_keys[0]:&req_json_values[0],
            &req_json_keys[1]:&req_json_values[1]
        });
        let client = Client::new();
        let response = client
            .post(self.server_url.replace("{userId}", &id))
            .header(CONTENT_TYPE, "application/json")
            .header("X-Emby-Authorization", "MediaBrowser Client=\"JellyRoller\", Device=\"jellyroller-cli\", DeviceId=\"1\", Version=\"0.0.1\"")
            .header("X-Emby-Token", self.api_key)
            .body(serde_json::to_string_pretty(&body).unwrap())
            .send()?;
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("User {} configuration value of {} successfully set to {}", username, config_key, config_value);
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }
        Ok(())
    }

    pub fn get_user_providers_vec(self, id: String) -> Result<Vec<String>, reqwest::Error> {
        let client = Client::new();
        let response = client
            .get(self.server_url.replace("{userId}", &id))
            .header("X-Emby-Token", self.api_key.to_string())
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let json = response.text().unwrap();
                let providers = serde_json::from_str::<User>(&json).unwrap();
                // Auth provider will always be at position 0 and PassReset provider will be at 1
                let json_vec = vec![providers.policy.auth_provider_id, providers.policy.pass_reset_provider_id];
                Ok(json_vec)
                    
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
                std::process::exit(0);
            } _ => {
                println!("Status Code: {}", response.status());
                std::process::exit(0);
            }
        }
    }
}