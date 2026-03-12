use sequoia_openpgp::cert::CertParser;
use sequoia_openpgp::parse::Parse;
use std::process::Command;
use sequoia_openpgp::Cert;

pub fn get_certs() -> Result<Vec<Cert>, String> {
    let mut command = Command::new("gpg");
    command.arg("--export").arg("-a");
    let output = command.output().expect("failed to excaute command");
    let armored_output = String::from_utf8_lossy(&output.stdout);
    let mut certs = vec![];
    for cert in CertParser::from_reader(armored_output.as_bytes()).map_err(|e| e.to_string())? {
        match cert {
            Ok(cert) => certs.push(cert),
            Err(e) => eprintln!("Skipping malformed cert: {}", e),
        }
    }
    Ok(certs)
}