use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    db::db::Db,
    interface::command_strategy::CommandStrategy,
    interface::command_type::CommandType,
    session::session::Session,
    tools::resp::RespValue,
    RudisConfig,
};

pub struct HsetCommand {}

impl CommandStrategy for HsetCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        redis: &Arc<Mutex<Db>>,
        _rudis_config: &Arc<RudisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str,
    ) {
        let mut db_ref = redis.lock().unwrap();

        let db_index = {
            let sessions_ref = sessions.lock().unwrap();
            if let Some(session) = sessions_ref.get(session_id) {
                session.get_selected_database()
            } else {
                return;
            }
        };

        let key = fragments[4].to_string();
        let field = fragments[6].to_string();
        let value = fragments[8].to_string();
        
        db_ref.check_ttl(db_index, &key);
        
        match db_ref.hset(db_index, key.clone(), field, value) {
            Ok(result) => {
                if let Some(stream) = stream {
                    let response_bytes = &RespValue::Integer(result as i64).to_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {
                            // Response successful
                        },
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            }
            Err(err_msg) => {
                if let Some(stream) = stream {
                    let response_bytes = &RespValue::Error(err_msg.to_string()).to_bytes();
                    match stream.write(response_bytes) {
                        Ok(_bytes_written) => {
                            // Response successful
                        },
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                        },
                    };
                }
            }
        }
    }

    fn command_type(&self) -> CommandType {
        CommandType::Write
    }
}