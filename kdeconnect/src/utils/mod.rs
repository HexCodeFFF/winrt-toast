use winrt_toast::{Text, Toast, ToastManager};

pub mod clipboard;

lazy_static::lazy_static! {
    pub static ref TOAST_MANAGER: ToastManager = {
        ToastManager::new(crate::AUM_ID)
    };
}

pub fn unix_ts_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn log_if_error<R, E: std::fmt::Debug>(text: &str, res: Result<R, E>) {
    if let Err(e) = res {
        log::error!("{}: {:?}", text, e);
    }
}

pub async fn simple_toast(title: &str, content: Option<&str>, attribution: Option<&str>) {
    let mut toast = Toast::new();
    toast.text1(title);

    if let Some(c) = content {
        toast.text2(c);
    }

    if let Some(attr) = attribution {
        toast.text3(Text::new(attr).as_attribution());
    }

    let res = tokio::task::spawn_blocking(move || TOAST_MANAGER.show(&toast)).await;
    match res {
        Ok(Ok(_)) => {}
        Ok(Err(e)) => {
            log::error!("Failed to show toast: {:?}", e);
        }
        Err(e) => {
            log::error!("Failed to show toast: {:?}", e);
        }
    }
}