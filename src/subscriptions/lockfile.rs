use std::{env, sync::Arc};

use tokio::{fs, sync::Mutex};

use crate::Message;

#[derive(Clone, Debug)]
pub struct File {
    pub name: String,
    pub pid: String,
    pub port: String,
    pub password: String,
    pub protocol: String,
}

#[derive(Clone, Debug, Default)]
pub struct LockFile {
    data: Arc<Mutex<Option<File>>>,
}

impl LockFile {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn name(&self) {
        let _data = self.data.lock().await;
    }

    pub async fn get_file(&self) -> Option<File> {
        let data = self.data.lock().await;
        (*data).clone()
    }

    pub async fn set_file_async(&self, file: Option<File>) {
        let mut data = self.data.lock().await;
        *data = file;
    }

    pub fn set_file(&self, file: Option<File>) {
        loop {
            match self.data.try_lock() {
                Ok(mut data) => {
                    *data = file;
                    break;
                }
                Err(_) => (),
            };
        }
    }
}

pub fn get_lockfile() -> iced::Subscription<crate::Message> {
    iced::subscription::unfold("get_lockfile", (), |_| async {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let string = fs::read_to_string(format!(
            "{}\\Riot Games\\Riot Client\\Config\\lockfile",
            env::var("LocalAppData").unwrap()
        ))
        .await
        .ok();
        (Some(string), ())
    })
    .map(Message::File)
}
