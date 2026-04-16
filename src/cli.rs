use crate::Error;
use clap::{ArgGroup, Parser};
use log::LevelFilter;
use tonic::transport::Uri;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
#[clap(group(ArgGroup::new("required")))]
pub struct Opt {
    #[clap(short, long, display_order = 3)]
    /// Enable debug logging level
    pub debug: bool,
    #[clap(short = 'a', long = "mining-address", display_order = 0)]
    /// The Kaspa address for the miner reward
    pub mining_address: String,
    #[clap(short = 's', long = "kaspad-address", default_value = "127.0.0.1", display_order = 1)]
    /// The IP of the kaspad instance
    pub kaspad_address: String,

    #[clap(long = "devfund", display_order = 6)]
    /// Mine a percentage of the blocks to the Kaspa devfund [default: Off]
    pub devfund_address: Option<String>,

    #[clap(long = "devfund-percent", default_value = "1", display_order = 7, value_parser = parse_devfund_percent)]
    /// The percentage of blocks to send to the devfund
    pub devfund_percent: u16,

    #[clap(short, long, display_order = 2)]
    /// Kaspad port [default: Mainnet = 16110, Testnet = 16210]
    port: Option<u16>,

    #[clap(long, display_order = 4)]
    /// Use testnet instead of mainnet [default: false]
    testnet: bool,
    #[clap(short = 't', long = "threads", display_order = 5)]
    /// Amount of miner threads to launch [default: number of logical cpus]
    pub num_threads: Option<u16>,
    #[clap(long = "mine-when-not-synced", display_order = 8)]
    /// Mine even when kaspad says it is not synced, only useful when passing `--allow-submit-block-when-not-synced` to kaspad  [default: false]
    pub mine_when_not_synced: bool,
    #[clap(long = "throttle", display_order = 9)]
    /// Throttle (milliseconds) between each pow hash generation (used for development testing)
    pub throttle: Option<u64>,
    #[clap(long, display_order = 10)]
    /// Output logs in alternative format (same as kaspad)
    pub altlogs: bool,
    #[clap(long = "user-agent-suffix", display_order = 11)]
    /// Custom user agent suffix (max 20 characters)
    pub user_agent_suffix: Option<String>,
}

fn parse_devfund_percent(s: &str) -> Result<u16, &'static str> {
    let err = "devfund-percent should be --devfund-percent=XX.YY up to 2 numbers after the dot";
    let mut splited = s.split('.');
    let prefix = splited.next().ok_or(err)?;
    // if there's no postfix then it's 0.
    let postfix = splited.next().ok_or(err).unwrap_or("0");
    // error if there's more than a single dot
    if splited.next().is_some() {
        return Err(err);
    };
    // error if there are more than 2 numbers before or after the dot
    if prefix.len() > 2 || postfix.len() > 2 {
        return Err(err);
    }
    let postfix: u16 = postfix.parse().map_err(|_| err)?;
    let prefix: u16 = prefix.parse().map_err(|_| err)?;
    // can't be more than 99.99%,
    if prefix >= 100 || postfix >= 100 {
        return Err(err);
    }
    Ok(prefix * 100 + postfix)
}

impl Opt {
    pub fn process(&mut self) -> Result<(), Error> {
        if self.kaspad_address.is_empty() {
            self.kaspad_address = "127.0.0.1".to_string();
        }

        let has_scheme = has_supported_scheme(&self.kaspad_address);
        let port_provided_as_arg = self.port.is_some();
        let address = if has_scheme { self.kaspad_address.clone() } else { format!("http://{}", self.kaspad_address) };

        let uri = Uri::try_from(address.as_str())?;
        let authority = uri
            .authority()
            .ok_or_else(|| format!("invalid kaspad address `{}`: missing authority (host)", self.kaspad_address))?;

        let scheme = uri.scheme_str().unwrap_or("http");
        let path_and_query = uri.path_and_query().map(|value| value.as_str()).unwrap_or("");

        if authority.port().is_none() && (!has_scheme || port_provided_as_arg) {
            let port = self.port();
            self.kaspad_address = format!("{scheme}://{authority}:{port}{path_and_query}");
        } else {
            self.kaspad_address = format!("{scheme}://{authority}{path_and_query}");
        }

        if let Some(suffix) = &self.user_agent_suffix {
            if suffix.contains('/') {
                return Err("--user-agent-suffix cannot contain '/' characters".into());
            }
            if suffix.chars().count() > 20 {
                return Err("--user-agent-suffix must be at most 20 characters".into());
            }
        }

        Ok(())
    }

    fn port(&mut self) -> u16 {
        *self.port.get_or_insert(if self.testnet { 16210 } else { 16110 })
    }

    pub fn log_level(&self) -> LevelFilter {
        if self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    }
}

fn has_supported_scheme(address: &str) -> bool {
    return address.starts_with("http://") || address.starts_with("https://");
}
