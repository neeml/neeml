/* 'Remotes' providers */

pub mod remotes {
    pub trait BaseRemoteProvider {
        fn connect(&self) -> Result<(), String>;
        fn is_connected(&self) -> bool;
        fn is_disconnected(&self) -> bool {
            !self.is_connected()
        }
        fn disconnect(&self) -> Result<(), String>;
    }

    pub mod mail {
        use std::collections::HashMap;

        use super::BaseRemoteProvider;

        pub trait MailRemoteProvider: BaseRemoteProvider {
            fn get_credentials(&self) -> Option<HashMap<String, String>>;
            fn fetch(&self) -> Result<(), String>;
            fn refresh(&self) -> Result<(), String> {
                self.fetch()
            }
        }

        pub trait ImapRemoteProvider: MailRemoteProvider {}

        pub trait JmapRemoteProvider: MailRemoteProvider {}

        pub trait OutgoingMailRemoteProvider: BaseRemoteProvider {
            fn send_mail(&self, mail: &str) -> Result<(), String>;
        }
    }

    pub mod news {
        use super::BaseRemoteProvider;

        pub trait NewsRemoteProvider: BaseRemoteProvider {
            fn fetch(&self) -> Result<(), String>;
            fn refresh(&self) -> Result<(), String> {
                self.fetch()
            }
            fn fetch(&self) -> Result<(), String>;
            fn refresh(&self) -> Result<(), String> {
                self.fetch()
            }
        }

        pub trait NntpRemoteProvider: NewsRemoteProvider {
            fn configure(&self);
            fn send_article(&self, article: &str) -> Result<(), String>;
        }

        pub trait RssRemoteProvider: NewsRemoteProvider {}
    }
}
