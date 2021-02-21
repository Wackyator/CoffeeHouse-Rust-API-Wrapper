extern crate reqwest;

mod structs;
mod error;

use std::result;

pub type Result<T> = result::Result<T, crate::error::Error>;

pub mod lydia {
    use crate::structs::{
        SuccessResponse,
        ErrorResponse,
        ThoughtResponse,
    };
    
    use crate::error::Error;

    
    pub struct LydiaAI {
        api_endpoint: String,
        access_key: String,
        session_id: String,
        client: reqwest::Client,
    }

    impl LydiaAI {
        pub fn new(access_key: &str) -> Self {
            Self {
                api_endpoint: String::from("https://api.intellivoid.net/coffeehouse/v1/lydia"),
                access_key: String::from(access_key),
                client: reqwest::Client::new(),
                session_id: String::new(),
            }
        }
    }

    impl LydiaAI {
        pub fn construct_url(&self, route: &str) -> String {
            format!("{}/{}", self.api_endpoint, route)
        }

        pub async fn create_session(&mut self) -> crate::Result<SuccessResponse> {
            let res = self.client.get(&self.construct_url("session/create"))
                .query(&[("access_key", &self.access_key)])
                .header("user-agent", "Mozilla/5.0")
                .send().await?;

            match res.status().as_u16() {
                200 => {
                    let session = res.json::<SuccessResponse>().await?;
                    self.session_id = session.results.session_id.clone();
                    Ok(session)
                },
                _ => {
                    let err = res.json::<ErrorResponse>().await?;
                    Err(Error::from(err))
                },
            }
        }

        pub async fn think_thought(&self, input: String) -> crate::Result<ThoughtResponse> {
            let res = self.client.get(&self.construct_url("session/think"))
                .query(&[("access_key", &self.access_key), ("session_id", &self.session_id), ("input", &input)])
                .header("user-agent", "Mozilla/5.0")
                .send().await?;

            match res.status().as_u16() {
                200 => {
                    let thought = res.json::<ThoughtResponse>().await?;
                    Ok(thought)
                },
                _ => {
                    let err = res.json::<ErrorResponse>().await?;
                    Err(Error::from(err))
                },
            }
        }
    }
}
