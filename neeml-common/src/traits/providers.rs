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
        use std::path::PathBuf;

        use super::BaseRemoteProvider;

        pub trait MailRemoteProvider: BaseRemoteProvider {
            fn get_credentials(&self) -> Option<HashMap<String, String>>;
            fn authenticate(&self, credentials: HashMap<String, String>) -> Result<(), String>;
            fn authenticate_with_oauth2(&self, token: &str) -> Result<(), String>;
            fn refresh_token(&self) -> Result<(), String>;
            fn authenticate_anonymous(&self) -> Result<(), String>;
            fn fetch(&self) -> Result<(), String>;
            fn refresh(&self) -> Result<(), String> {
                self.fetch()
            }
        }

        #[cfg(feature = "providers-imap")]
        pub trait ImapRemoteProvider: MailRemoteProvider {
            fn get_imap_server(&self) -> Option<String>;
        }

        #[cfg(feature = "providers-jmap")]
        pub trait JmapRemoteProvider: MailRemoteProvider {
            fn get_jmap_server(&self) -> Option<String>;
        }

        #[cfg(feature = "providers-maildir")]
        pub trait MaildirRemoteProvider: MailRemoteProvider {
            fn get_maildir_path(&self) -> Option<PathBuf>;
        }

        #[cfg(feature = "providers-mbox")]
        pub trait MboxRemoteProvider: MailRemoteProvider {
            fn get_mbox_path(&self) -> Option<PathBuf>;
        }

        pub trait OutgoingMailRemoteProvider: BaseRemoteProvider {
            fn send_mail(&self, mail: &str) -> Result<(), String>;
        }

        #[cfg(feature = "providers-sendmail")]
        pub trait SendmailOutgoingMailRemoteProvider: OutgoingMailRemoteProvider {}

        #[cfg(feature = "providers-smtp")]
        pub trait SmtpOutgoingMailRemoteProvider: OutgoingMailRemoteProvider {}

        #[cfg(feature = "providers-contexts-exchange")]
        pub trait ExchangeMailRemoteProvider: MailRemoteProvider {
            fn get_exchange_server(&self) -> Option<String>;
            fn get_ews_url(&self) -> Option<String>;
        }
    }

    pub mod contexts {
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

        #[cfg(feature = "providers-contexts-caldav")]
        pub trait CaldavRemoteProvider: DavRemoteProvider {
            fn get_caldav_url(&self) -> Option<String>;
        }

        #[cfg(feature = "providers-contexts-carddav")]
        pub trait CarddavRemoteProvider: DavRemoteProvider {
            fn get_carddav_url(&self) -> Option<String>;
        }
    }

    pub mod completion {
        use super::contexts::ContextsRemoteProvider;
        pub trait CompletionRemoteProvider: BaseRemoteProvider {
            fn fetch(&self) -> Result<(), String>;
            fn refresh(&self) -> Result<(), String> {
                self.fetch()
            }
        }

        pub trait ContextsCompletionRemoteProvider:
            CompletionRemoteProvider + ContextsRemoteProvider
        {
            fn get_contexts(&self) -> Result<(), String>;
        }

        #[cfg(feature = "providers-contexts-llm")]
        pub trait LlmCompletionRemoteProvider: CompletionRemoteProvider {
            fn get_llm_url(&self) -> Option<String>;
            fn query_llm_prompt(&self) -> Result<String, String>;
        }

        #[cfg(feature = "providers-contexts-openai")]
        pub trait OpenAiCompletionRemoteProvider: LlmCompletionRemoteProvider {}

        #[cfg(feature = "providers-contexts-ollama")]
        pub trait OllamaCompletionRemoteProvider: LlmCompletionRemoteProvider {}
    }

    pub mod news {
        use super::BaseRemoteProvider;

        pub trait NewsRemoteProvider: BaseRemoteProvider {
            fn fetch(&self) -> Result<(), String>;
            fn refresh(&self) -> Result<(), String> {
                self.fetch()
            }
        }

        #[cfg(feature = "providers-nntp")]
        pub trait NntpRemoteProvider: NewsRemoteProvider {
            fn configure(&self);
            fn send_article(&self, article: &str) -> Result<(), String>;
        }

        #[cfg(feature = "providers-rss")]
        pub trait RssRemoteProvider: NewsRemoteProvider {
            fn get_rss_url(&self) -> Option<String>;
            fn parse_rss(&self) -> Result<(), String>;
        }
    }

    pub mod storage {
        use super::BaseRemoteProvider;
        pub trait StorageRemoteProvider: BaseRemoteProvider {
            fn fetch(&self) -> Result<(), String>;
            fn refresh(&self) -> Result<(), String> {
                self.fetch()
            }
            fn sync(&self) -> Result<(), String>;
        }

        #[cfg(feature = "providers-rsync")]
        pub trait RsyncRemoteProvider: StorageRemoteProvider {
            fn snapshot(&self) -> Result<(), String>;
            fn get_rsync_path(&self) -> Option<String>;
        }

        #[cfg(feature = "providers-sqlite")]
        pub trait SqliteRemoteProvider: StorageRemoteProvider {
            fn query(&self, query: &str) -> Result<(), String>;
            fn get_db_path(&self) -> Option<String>;
        }
    }
}
