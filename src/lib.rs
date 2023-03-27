extern crate curl;
extern crate log;
extern crate samp;

use curl::easy::{Easy, List};
use samp::prelude::*;
use samp::{initialize_plugin, native};

struct Plugin {
    token: String,
}

const RAC_API_URL: &str = "https://rockac.stalker-rp.net/api";

impl SampPlugin for Plugin {
    fn on_load(&mut self) {
        curl::init();
        log::info!(" >> ROCK-AC: Plugin loaded!");
    }
}

impl Plugin {
    fn new() -> Self {
        return Self {
            token: String::new(),
        };
    }

    fn ac_headers_list(&mut self) -> List {
        let mut list = List::new();

        list.append(&*format!("API-KEY: {}", self.token)).unwrap();
        list.append(&*format!("User-Agent: {}", "STALKER/1.0"))
            .unwrap();

        return list;
    }

    #[native(name = "RAC_SetAccessToken")]
    fn set_access_token(&mut self, _: &Amx, token: AmxString) -> AmxResult<bool> {
        if !self.token.is_empty() {
            log::warn!(" >> ROCK-AC: Token has been already installed!");
            return Ok(false);
        }

        self.token = token.to_string();
        log::info!(" >> ROCK-AC: Installed token access ({})", self.token);

        return Ok(true);
    }

    #[native(name = "RAC_SetUserSessionStatus")]
    fn set_session_status(&mut self, _: &Amx, name: AmxString, status: bool) -> AmxResult<bool> {
        if self.token.is_empty() {
            log::error!(" >> ROCK-AC: API token isn't installed");
            return Ok(false);
        }

        let state: &str = match status {
            true => "connected",
            false => "disconnected",
        };

        let sz_url = &*String::from(format!(
            "{}/perform.php?state={}&name={}",
            RAC_API_URL, state, name
        ));

        let mut handle = Easy::new();
        handle.url(sz_url).unwrap();
        handle.http_headers(self.ac_headers_list()).unwrap();
        handle.get(true).unwrap();

        let mut content = String::new();
        {
            let mut transfer = handle.transfer();
            transfer
                .write_function(|data| {
                    content = String::from_utf8(Vec::from(data)).unwrap();
                    Ok(data.len())
                })
                .unwrap();

            transfer.perform().unwrap();
        }

        if handle.response_code() != Ok(200) {
            log::error!(
                " >> ROCK-AC: <RAC_SetUserSessionStatus> HTTP response code {}",
                handle.response_code().unwrap()
            );
            return Ok(false);
        }

        let response = json::parse(&*content).unwrap();
        if response["status"] == false {
            return Ok(false);
        }

        Ok(true)
    }

    #[native(name = "RAC_BanUser")]
    fn ban_user(&mut self, _: &Amx, name: AmxString, reason: AmxString) -> AmxResult<bool> {
        if self.token.is_empty() {
            log::error!(" >> ROCK-AC: API token isn't installed");
            return Ok(false);
        }

        let text: &str = &*reason.to_string();
        let sz_reason = urlencoding::encode(text);

        let mut handle = Easy::new();
        handle
            .url(&*format!(
                "{}/block.php?action=block&name={}&reason={}",
                RAC_API_URL,
                name,
                sz_reason.into_owned()
            ))
            .unwrap();
        handle.http_headers(self.ac_headers_list()).unwrap();
        handle.get(true).unwrap();

        let mut content = String::new();
        {
            let mut transfer = handle.transfer();
            transfer
                .write_function(|data| {
                    content = String::from_utf8(Vec::from(data)).unwrap();
                    Ok(data.len())
                })
                .unwrap();

            transfer.perform().unwrap();
        }

        if handle.response_code() != Ok(200) {
            log::error!(
                " >> ROCK-AC: <RAC_BanUser> HTTP response code {}",
                handle.response_code().unwrap()
            );
            return Ok(false);
        }

        let response = json::parse(&*content).unwrap();
        if response["status"] == false {
            return Ok(false);
        }

        Ok(true)
    }

    #[native(name = "RAC_UnbanUser")]
    fn unban_user(&mut self, _: &Amx, name: AmxString) -> AmxResult<bool> {
        if self.token.is_empty() {
            log::error!(" >> ROCK-AC: API token isn't installed");
            return Ok(false);
        }

        let mut handle = Easy::new();
        handle
            .url(&*format!(
                "{}/block.php?action=unblock&name={}",
                RAC_API_URL, name
            ))
            .unwrap();
        handle.http_headers(self.ac_headers_list()).unwrap();
        handle.get(true).unwrap();

        let mut content = String::new();
        {
            let mut transfer = handle.transfer();
            transfer
                .write_function(|data| {
                    content = String::from_utf8(Vec::from(data)).unwrap();
                    Ok(data.len())
                })
                .unwrap();

            transfer.perform().unwrap();
        }

        if handle.response_code() != Ok(200) {
            log::error!(
                " >> ROCK-AC: <RAC_UnbanUser> HTTP response code {}",
                handle.response_code().unwrap()
            );
            return Ok(false);
        }

        let response = json::parse(&*content).unwrap();
        if response["status"] == false {
            return Ok(false);
        }

        Ok(true)
    }

    #[native(name = "RAC_ClearSessions")]
    fn clear_session(&mut self, _: &Amx) -> AmxResult<bool> {
        Ok(true)
    }
}

initialize_plugin! {
    natives: [
        Plugin::set_access_token,
        Plugin::set_session_status,
        Plugin::ban_user,
        Plugin::unban_user,
        Plugin::clear_session
    ],
    {
        return Plugin::new();
    }
}
