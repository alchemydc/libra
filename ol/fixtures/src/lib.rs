use std::{fs, path::{Path, PathBuf}};

use ol_types::{block::VDFProof, config::{AppCfg, parse_toml}};

pub fn get_persona_mnem(persona: &str) -> String {
  let path= env!("CARGO_MANIFEST_DIR");
  let buf = Path::new(path).join("mnemonic").join(format!("{}.mnem", persona));
  fs::read_to_string(&buf).expect("could not find mnemonic file")
}

pub fn get_persona_account_json(persona: &str) -> (String, PathBuf) {
  let path= env!("CARGO_MANIFEST_DIR");
  let buf = Path::new(path).join("account").join(format!("{}.account.json", persona));
  (
    fs::read_to_string(&buf).expect("could not account file"),
    buf
  )
}

pub fn get_persona_autopay_json(persona: &str) -> (String, PathBuf) {
  let path= env!("CARGO_MANIFEST_DIR");
  let buf = Path::new(path).join("autopay").join(format!("{}.autopay_batch.json", persona));
  (
    fs::read_to_string(&buf).expect("could not find autopay file"),
    buf
  )
}

pub fn get_demo_autopay_json() -> (String, PathBuf) {
  let path= env!("CARGO_MANIFEST_DIR");
  let buf = Path::new(path).join("autopay").join("all.autopay_batch.json");
  (
    fs::read_to_string(&buf).expect("could not find autopay file"),
    buf
  )
}

pub fn get_test_genesis_blob() -> PathBuf {
  let path= env!("CARGO_MANIFEST_DIR");
  Path::new(path).join("genesis").join("swarm_genesis.blob")
}

pub fn get_persona_toml_configs(persona: &str) -> AppCfg {
  let path= env!("CARGO_MANIFEST_DIR");
  let buf = Path::new(path).join("configs").join(format!("{}.toml", persona));
  parse_toml(buf.to_str().unwrap().to_owned()).expect("could not parse app config from file")
}

// TODO: duplicated with ol/types/genesis proof
pub fn get_persona_block_zero(persona: &str, env: &str) -> VDFProof {
  let path= env!("CARGO_MANIFEST_DIR");
  let buf = Path::new(path).join(format!("blocks/{}/{}/block_0.json", env, persona));
  let s = fs::read_to_string(&buf).expect("could not find block file");
  serde_json::from_str(&s).expect(&format!("could not parse block from file: {:?}", &buf))

}
