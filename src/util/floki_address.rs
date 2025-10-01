#![cfg(not(feature = "liquid"))]

use core::convert::TryFrom;
use core::str::FromStr;

use bitcoin::bech32;
use bitcoin::blockdata::script::witness_program::WitnessProgram;
use bitcoin::blockdata::script::witness_version::WitnessVersion;
use bitcoin::KnownHrp;

use crate::chain::address::{self, NetworkUnchecked};
use crate::chain::BNetwork;
use crate::chain::Script;
use crate::chain::{Address, Network};

/// Produce the script pubkey associated with a Flokicoin address.
///
/// This tries the upstream parser first and falls back to decoding
/// Flokicoin-specific bech32 human-readable parts.
pub fn script_pubkey_from_address(addr: &str, network: Network) -> Result<Script, String> {
    match Address::<NetworkUnchecked>::from_str(addr) {
        Ok(parsed) => parsed
            .require_network(BNetwork::from(network))
            .map(|checked| checked.script_pubkey())
            .map_err(|err| err.to_string()),
        Err(address::ParseError::UnknownHrp(_)) => parse_flokicoin_bech32(addr, network),
        Err(err) => Err(err.to_string()),
    }
}

fn parse_flokicoin_bech32(addr: &str, network: Network) -> Result<Script, String> {
    let (hrp, witness_version, program) =
        bech32::segwit::decode(addr).map_err(|e| e.to_string())?;

    let hrp_lower = hrp.to_lowercase();
    let expected_hrp = expected_hrp(network);
    if hrp_lower != expected_hrp {
        return Err(format!(
            "Address on invalid network: expected hrp '{}', got '{}'",
            expected_hrp, hrp_lower
        ));
    }

    let version = WitnessVersion::try_from(witness_version).map_err(|e| e.to_string())?;
    let program = WitnessProgram::new(version, &program).map_err(|e| e.to_string())?;

    let known_hrp = match hrp_lower.as_str() {
        "fc" => KnownHrp::Mainnet,
        "tf" => KnownHrp::Testnets,
        "fcrt" => KnownHrp::Regtest,
        other => return Err(format!("Unsupported bech32 hrp '{}'", other)),
    };

    let address = Address::from_witness_program(program, known_hrp);
    Ok(address.script_pubkey())
}

fn expected_hrp(network: Network) -> &'static str {
    match network {
        Network::Bitcoin => "fc",
        Network::Testnet | Network::Testnet4 | Network::Signet => "tf",
        Network::Regtest => "fcrt",
        #[allow(unreachable_patterns)]
        _ => "fc",
    }
}
