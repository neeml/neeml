/* 'Remotes' providers */

pub trait BaseRemoteProvider {
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

#[cfg(feature = "providers-mail")]
pub mod mail {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use super::BaseRemoteProvider;

    #[cfg(feature = "providers-mail")]
    pub trait MailRemoteProvider: BaseRemoteProvider {
        fn get_credentials(&self) -> Option<HashMap<String, String>> {
            None /* Maildirs/Mbox don't need authentication, for example */
        }
        fn authenticate(&self, _credentials: HashMap<String, String>) -> Result<(), String> {
            Err(String::from("Not supported.")) /* Maildirs/Mbox don't need authentication, for example */
        }
        fn authenticate_with_oauth2(&self, _token: &str) -> Result<(), String> {
            Err(String::from("Not supported.")) /* Maildirs/Mbox don't need authentication, for example */
        }
        fn refresh_token(&self) -> Result<(), String> {
            Err(String::from("Not supported.")) /* Maildirs/Mbox don't need authentication, for example */
        }
        fn authenticate_anonymous(&self) -> Result<(), String> {
            Err(String::from("Not supported.")) /* Maildirs/Mbox don't need authentication, for example */
        }
        fn fetch(&self) -> Result<(), String> {
            Ok(())
        }
        fn refresh(&self) -> Result<(), String> {
            self.fetch()
        }
    }

    #[cfg(feature = "providers-mail-imap")]
    pub trait ImapRemoteProvider: MailRemoteProvider {
        fn get_imap_server(&self) -> Option<String>;
    }

    #[cfg(feature = "providers-mail-jmap")]
    pub trait JmapRemoteProvider: MailRemoteProvider {
        fn get_jmap_server(&self) -> Option<String>;
    }

    #[cfg(feature = "providers-mail-maildir")]
    pub trait MaildirRemoteProvider: MailRemoteProvider {
        fn get_maildir_path(&self) -> Option<PathBuf>;
    }

    #[cfg(feature = "providers-mail-mbox")]
    pub trait MboxRemoteProvider: MailRemoteProvider {
        fn get_mbox_path(&self) -> Option<PathBuf>;
    }

    #[cfg(feature = "providers-mail")]
    pub trait OutgoingMailRemoteProvider: BaseRemoteProvider {
        fn send_mail(&self, mail: &str) -> Result<(), String>;
    }

    #[cfg(feature = "providers-mail-sendmail")]
    pub trait SendmailOutgoingMailRemoteProvider: OutgoingMailRemoteProvider {}

    #[cfg(feature = "providers-mail-smtp")]
    pub trait SmtpOutgoingMailRemoteProvider: OutgoingMailRemoteProvider {}

    #[cfg(feature = "providers-mail-exchange")]
    pub trait ExchangeMailRemoteProvider: MailRemoteProvider {
        fn get_exchange_server(&self) -> Option<String>;
        fn get_ews_url(&self) -> Option<String>;
    }
}

#[cfg(feature = "providers-completion")]
pub mod completion {
    use super::BaseRemoteProvider;
    pub trait CompletionRemoteProvider: BaseRemoteProvider {}

    #[cfg(feature = "providers-contexts")]
    pub mod contexts {
        use super::BaseRemoteProvider;

        #[cfg(feature = "providers-contexts")]
        pub trait ContextsRemoteProvider: BaseRemoteProvider {
            fn fetch(&self) -> Result<(), String>;
            fn refresh(&self) -> Result<(), String> {
                self.fetch()
            }
        }

        #[cfg(feature = "providers-contexts-exchange")]
        pub trait ExchangeContextsRemoteProvider: ContextsRemoteProvider {
            fn get_exchange_server(&self) -> Option<String>;
            fn get_ews_url(&self) -> Option<String>;
        }

        #[cfg(feature = "providers-contexts-gmail")]
        pub trait GmailContextsRemoteProvider: ContextsRemoteProvider {
            fn get_gmail_server(&self) -> Option<String>;
        }

        #[cfg(feature = "providers-contexts-notmuch")]
        pub trait NotmuchContextsRemoteProvider: ContextsRemoteProvider {
            fn get_notmuch_path(&self) -> Option<String>;
        }

        #[cfg(feature = "providers-contexts-dav")]
        pub trait DavRemoteProvider: ContextsRemoteProvider {
            fn get_dav_url(&self) -> Option<String>;
        }

        #[cfg(all(
            feature = "providers-contexts-caldav",
            feature = "providers-contexts-dav"
        ))]
        pub trait CaldavRemoteProvider: DavRemoteProvider {
            fn get_caldav_url(&self) -> Option<String>;
        }

        #[cfg(all(
            feature = "providers-contexts-carddav",
            feature = "providers-contexts-dav"
        ))]
        pub trait CarddavRemoteProvider: DavRemoteProvider {
            fn get_carddav_url(&self) -> Option<String>;
        }
    }

    #[cfg(feature = "providers-completion-llm")]
    pub mod llm {
        use crate::traits::providers::completion::CompletionRemoteProvider;

        #[cfg(feature = "providers-completion-llm")]
        pub trait LlmCompletionRemoteProvider: CompletionRemoteProvider {
            fn get_llm_url(&self) -> Option<String>;
            fn query_llm_prompt(&self) -> Result<String, String>;
        }

        #[cfg(feature = "providers-completion-llm-openai")]
        pub trait OpenAiCompletionRemoteProvider: LlmCompletionRemoteProvider {}

        #[cfg(feature = "providers-completion-llm-ollama")]
        pub trait OllamaCompletionRemoteProvider: LlmCompletionRemoteProvider {}
    }
}

#[cfg(feature = "providers-news")]
pub mod news {
    use super::BaseRemoteProvider;

    #[cfg(feature = "providers-news")]
    pub trait NewsRemoteProvider: BaseRemoteProvider {
        fn fetch(&self) -> Result<(), String>;
        fn refresh(&self) -> Result<(), String> {
            self.fetch()
        }
    }

    #[cfg(feature = "providers-news-nntp")]
    pub trait NntpRemoteProvider: NewsRemoteProvider {
        fn configure(&self);
        fn send_article(&self, article: &str) -> Result<(), String>;
    }

    #[cfg(feature = "providers-news-rss")]
    pub trait RssRemoteProvider: NewsRemoteProvider {
        fn get_rss_url(&self) -> Option<String>;
        fn parse_rss(&self) -> Result<(), String>;
    }
}

#[cfg(feature = "providers-storage")]
pub mod storage {
    use super::BaseRemoteProvider;

    #[cfg(feature = "providers-storage")]
    pub trait StorageRemoteProvider: BaseRemoteProvider {
        fn fetch(&self) -> Result<(), String>;
        fn refresh(&self) -> Result<(), String> {
            self.fetch()
        }
        fn sync(&self) -> Result<(), String>;
    }

    #[cfg(feature = "providers-storage-rsync")]
    pub trait RsyncRemoteProvider: StorageRemoteProvider {
        fn snapshot(&self) -> Result<(), String>;
        fn get_rsync_path(&self) -> Option<String>;
    }

    #[cfg(feature = "providers-storage-sqlite")]
    pub trait SqliteRemoteProvider: StorageRemoteProvider {
        fn query(&self, query: &str) -> Result<(), String>;
        fn get_db_path(&self) -> Option<String>;
    }
}
