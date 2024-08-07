use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};

use crate::{db::db_config::RedisConfig, session::session::Session};
use crate::db::db::Redis;

use super::command_type::CommandType;

/*
 * 命令策略接口
 *
 * @param stream 通讯流
 * @param fragments 消息内容
 * @param redis 数据库实例
 * @param redis_config 数据库配置
 * @param sessions 会话列表
 */
pub trait CommandStrategy {

    // 命令逻辑
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        fragments: &[&str],
        redis: &Arc<Mutex<Redis>>,
        redis_config: &Arc<RedisConfig>,
        sessions: &Arc<Mutex<HashMap<String, Session>>>,
        session_id: &str,
    );

    // 命令类型
    fn command_type(&self) -> CommandType; 
}