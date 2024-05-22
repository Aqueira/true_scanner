use std::time::Duration;
use log::{error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::custom_errors::Errors;
use crate::ports::Ports;
use crate::TIME_OUT_PROGRAMS;

pub enum SSHFeatures{
    SSHVersion(String)
}
pub trait SSHVersion{
    async fn ssh_version(target: &str) -> Result<Vec<SSHFeatures>, Errors>;
}
impl SSHVersion for Ports{
    async fn ssh_version(target: &str) -> Result<Vec<SSHFeatures>, Errors> {
        let mut features_list = vec![];
        let ssh_port = 22;
        let url = format!("{}:{}",target, &ssh_port);
        let timeout_duration = Duration::from_secs(TIME_OUT_PROGRAMS);

        let mut tcp_stream = match timeout (timeout_duration, TcpStream::connect(&url)).await{
            Ok(Ok(stream)) => stream,
            Err(e) => {
                error!("Ошибка подключения к порту - {}", e);
                return Err(Errors::Error);
            }
            Ok(Err(e)) => {
                error!("Ошибка пермишена - {}", e);
                return Err(Errors::Error)
            },
        };

        let request = b"SSH-2.0\n";
        tcp_stream.write_all(request).await.map_err(|e|{
            error!("Ошибка отправки! - {}", e);
            Errors::Error
        })?;

        let mut buffer = [0; 1024];
        tcp_stream.read(&mut buffer).await.map_err(|e|{
            error!("Ошибка записи! - {}", e);
            Errors::Error
        })?;

        if buffer.is_empty(){
            error!("Пустой буффер!");
            return Err(Errors::Error)
        };

        let version_brute = String::from_utf8_lossy(&mut buffer);
        if let Some(version) = version_brute.lines().next(){
            features_list.push(SSHFeatures::SSHVersion(version.to_string()));
        }
        Ok(features_list)
    }
}