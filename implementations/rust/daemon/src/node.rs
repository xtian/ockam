use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use crate::cli;
use crate::config::{Config, Role};
use crate::worker::Worker;

use crate::cli::VaultKind;
use ockam_channel::*;
use ockam_kex::xx::{XXInitiator, XXNewKeyExchanger, XXResponder};
use ockam_message::message::AddressType;
use ockam_router::router::Router;
use ockam_system::commands::{OckamCommand, RouterCommand};
use ockam_transport::transport::UdpTransport;
use ockam_vault::software::DefaultVaultSecret;
use ockam_vault::types::*;
use ockam_vault::{file::FilesystemVault, DynVault, Secret};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "atecc608a")] {
        use ockam_vault::c::Atecc608aVaultBuilder;
        use ockam_vault::c::CVault;
    }
}

#[allow(dead_code)]
pub struct Node<'a> {
    config: &'a Config,
    chan_manager: ChannelManager<XXInitiator, XXResponder, XXNewKeyExchanger>,
    worker: Option<Worker>,
    router: Router,
    router_tx: Sender<OckamCommand>,
    transport: UdpTransport,
    transport_tx: Sender<OckamCommand>,
    pub channel_tx: Sender<OckamCommand>,
}

impl<'a> Node<'a> {
    #[cfg(feature = "atecc608a")]
    fn create_atecc_vault() -> CVault {
        let builder = Atecc608aVaultBuilder::default();
        builder.build().expect("failed to initialize atecc vault")
    }

    #[cfg(not(feature = "atecc608a"))]
    fn create_atecc_vault() -> FilesystemVault {
        panic!("No atecc support")
    }

    pub fn new(config: &'a Config) -> (Self, Sender<OckamCommand>) {
        // TODO: temporarily passed into the node, need to re-work
        let (router_tx, router_rx) = std::sync::mpsc::channel();
        let router = Router::new(router_rx);

        let (resp_key_ctx, vault): (Option<Arc<Box<dyn Secret>>>, Arc<Mutex<dyn DynVault>>) =
            match config.vault_kind() {
                // create the vault, using the FILESYSTEM implementation
                VaultKind::Filesystem => {
                    let mut v = FilesystemVault::new(config.vault_path())
                        .expect("failed to initialize filesystem vault");
                    // check for re-use of provided identity name from CLI args, if not in on-disk in vault
                    // generate a new one to be used
                    let resp_key_ctx = if !contains_key(&mut v, &config.identity_name()) {
                        // if responder, generate keypair and display static public key
                        if matches!(config.role(), Role::Responder) {
                            let attributes = SecretKeyAttributes {
                                xtype: config.cipher_suite().get_secret_key_type(),
                                purpose: SecretPurposeType::KeyAgreement,
                                persistence: SecretPersistenceType::Persistent,
                            };
                            Some(Arc::new(
                                v.secret_generate(attributes)
                                    .expect("failed to generate secret"),
                            ))
                        } else {
                            None
                        }
                    } else {
                        Some(Arc::new(
                            as_key_ctx(&config.identity_name())
                                .expect("invalid identity name provided"),
                        ))
                    };
                    (resp_key_ctx, Arc::new(Mutex::new(v)))
                }
                VaultKind::Atecc => {
                    let vault = Node::create_atecc_vault();

                    // TODO: Prepare identity key in ATECC
                    (
                        None,
                        Arc::new(Mutex::new(vault)),
                    )
                }
            };

        if matches!(config.role(), Role::Responder) && resp_key_ctx.is_some() {
            let mut vault = vault.lock().unwrap();
            if let Ok(resp_key) = vault.secret_public_key_get(resp_key_ctx.as_ref().unwrap()) {
                println!("Responder public key: {}", hex::encode(resp_key));
            }
        }

        // create the channel manager
        type XXChannelManager = ChannelManager<XXInitiator, XXResponder, XXNewKeyExchanger>;
        let (channel_tx, channel_rx) = mpsc::channel();
        let new_key_exchanger =
            XXNewKeyExchanger::new(config.cipher_suite(), vault.clone(), vault.clone());

        let chan_manager = XXChannelManager::new(
            channel_rx,
            channel_tx.clone(),
            router_tx.clone(),
            vault,
            new_key_exchanger,
            resp_key_ctx,
            None,
        )
        .unwrap();

        // create the transport, currently UDP-only
        let transport_router_tx = router_tx.clone();
        let (transport_tx, transport_rx) = mpsc::channel();
        let self_transport_tx = transport_tx.clone();
        let transport = UdpTransport::new(
            transport_rx,
            transport_tx,
            transport_router_tx,
            config.local_host().to_string().as_str(),
        )
        .expect("failed to create udp transport");

        let node_router_tx = router_tx.clone();
        (
            Self {
                config,
                worker: None,
                router,
                router_tx,
                chan_manager,
                transport_tx: self_transport_tx,
                transport,
                channel_tx,
            },
            node_router_tx,
        )
    }

    pub fn add_worker(&mut self, worker: Worker) {
        self.router_tx
            .send(OckamCommand::Router(RouterCommand::Register(
                AddressType::Worker,
                worker.sender(),
            )))
            .expect("failed to register worker with router");

        self.worker = Some(worker);
    }

    pub fn run(mut self) {
        match self.worker {
            Some(worker) => {
                while self.router.poll()
                    && self.transport.poll()
                    && worker.poll()
                    && self
                        .chan_manager
                        .poll()
                        .expect("channel manager poll failure")
                {
                    thread::sleep(time::Duration::from_millis(1));
                }
            }
            None => {
                while self.router.poll()
                    && self.transport.poll()
                    && self
                        .chan_manager
                        .poll()
                        .expect("channel manager poll failure")
                {
                    thread::sleep(time::Duration::from_millis(1));
                }
            }
        }
    }
}

fn as_key_ctx(key_name: &str) -> Result<Box<dyn Secret>, String> {
    if let Some(id) = key_name.strip_suffix(cli::FILENAME_KEY_SUFFIX) {
        return Ok(Box::new(DefaultVaultSecret(
            id.parse().map_err(|_| format!("bad key name"))?,
        )));
    }

    Err("invalid key name format".into())
}

fn contains_key(v: &mut FilesystemVault, key_name: &str) -> bool {
    if let Ok(ctx) = as_key_ctx(key_name) {
        return v.secret_export(&ctx).is_ok();
    }

    false
}
