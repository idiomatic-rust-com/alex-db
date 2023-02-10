use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Connection {
    pub address: String,
    pub api_key: Option<Uuid>,
    pub is_default: bool,
}

impl Connection {
    pub fn new(address: String, api_key: Option<Uuid>, is_default: bool) -> Self {
        Self {
            address,
            api_key,
            is_default,
        }
    }
}

#[derive(Debug, Default)]
pub struct Context {
    pub connections: Vec<Connection>,
}

impl Context {
    pub fn get_default_connection(&self) -> Option<Connection> {
        for connection in self.connections.iter() {
            if connection.is_default {
                return Some(connection.clone());
            }
        }

        None
    }
}
