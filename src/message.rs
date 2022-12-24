//! Schemes of all messages used by server

/// Messages used by data server
pub mod data_server {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Request {
        Load(request::Load),
        Store(request::Store),
    }

    pub mod request {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Load {
            pub key: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Store {
            pub key: String,
            pub hash: Vec<u8>,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Response {
        Load(response::Load),
        Store(response::Store),
    }

    pub mod response {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Load {
            pub hash: Option<Vec<u8>>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Store {
            pub success: bool,
        }
    }
}
