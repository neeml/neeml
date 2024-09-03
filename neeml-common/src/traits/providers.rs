/* 'Remotes' providers */

use core::ops

pub type Provider = dyn BaseProvider;
pub type Remote = dyn BaseRemoteProvider;

pub trait BaseProvider {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_version(&self) -> String;
    fn get_author(&self) -> String;
}

pub trait BaseRemoteProvider : BaseProvider {
    fn connect(&self) -> Result<(), String> {
        Err(String::from("Not supported.")) /* Maildirs/Mbox don't work this way, for example */
    }
    fn is_connected(&self) -> bool {
        false
    }
    fn is_disconnected(&self) -> bool {
        !self.is_connected()
    }
    fn disconnect(&self) -> Result<(), String> {
        Err(String::from("Not supported.")) /* Maildirs/Mbox don't work this way, for example */
    }
}
