//! Identidade oficial sem alterar o protocolo binário do Fig nem seus arquivos.
use crate::invite::Endpoint;
use nova_live::identity::{self, Profile};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock, PoisonError};
use tungstenite::client::IntoClientRequest;
use tungstenite::handshake::{client::Request, server::Request as ServerRequest};

pub fn request(endpoint: &Endpoint) -> Result<Request, String> {
    let mut request = endpoint.url().into_client_request().map_err(|_| "Endereço de colaboração inválido.".to_string())?;
    if endpoint.secure && matches!(endpoint.host.as_str(), "collab.agcunae.com" | "collab.agcunae.com:443") {
        if !identity::authorized("novafig") {
            return Err("Conecte uma conta com acesso ao Nova Fig.".into());
        }
        let token = identity::token().ok_or_else(|| "Conecte sua conta Nova Foods.".to_string())?;
        request.headers_mut().insert("Authorization", format!("Bearer {token}").parse().map_err(|_| "Conta inválida.".to_string())?);
    }
    Ok(request)
}

pub fn required() -> bool {
    std::env::var("NOVAFOODS_ACCOUNTS").is_ok_and(|v| v == "required")
}
pub fn bearer(request: &ServerRequest) -> Option<String> {
    let token = request.headers().get("authorization")?.to_str().ok()?.strip_prefix("Bearer ")?;
    (token.len() == 43 && token.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')).then(|| token.to_string())
}
pub fn validate(token: &str) -> Result<Profile, String> {
    let profile = identity::validate_token(token).map_err(|_| "Conecte novamente sua conta Nova Foods.".to_string())?;
    if !profile.allows("novafig") {
        return Err("Nova Fig não está liberado para os seus setores.".into());
    }
    Ok(profile)
}

struct Bindings {
    path: PathBuf,
    files: BTreeMap<String, String>,
}
static FILES: OnceLock<Mutex<Option<Bindings>>> = OnceLock::new();
/// Um arquivo legado só recebe empresa depois de seu segredo ser validado pelo backend.
/// O vínculo fica ao lado dos arquivos, persistente e privado; falhas de leitura/escrita negam acesso.
pub fn file(profile: &Profile, file: &str, claim: bool) -> Result<(), String> {
    if file.is_empty() {
        return Ok(());
    }
    let bindings = FILES.get_or_init(|| Mutex::new(None));
    let mut guard = bindings.lock().unwrap_or_else(PoisonError::into_inner);
    if guard.is_none() {
        let path = std::env::var_os("NOVAFOODS_FILE_ORGANIZATIONS")
            .map(PathBuf::from)
            .ok_or_else(|| "Isolamento de arquivos não configurado.".to_string())?;
        let files = match std::fs::read(&path) {
            Ok(bytes) if bytes.len() <= 16 << 20 => serde_json::from_slice(&bytes).map_err(|_| "Registro de arquivos indisponível.".to_string())?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
            _ => return Err("Registro de arquivos indisponível.".into()),
        };
        *guard = Some(Bindings { path, files });
    }
    let Some(bindings) = guard.as_mut() else { return Err("Registro de arquivos indisponível.".into()) };
    bindings.check(profile, file, claim)
}
impl Bindings {
    fn check(&mut self, profile: &Profile, file: &str, claim: bool) -> Result<(), String> {
        if let Some(org) = self.files.get(file) {
            if org != &profile.organization.id {
                return Err("Este arquivo pertence a outra organização.".into());
            }
        } else if claim {
            let mut files = self.files.clone();
            files.insert(file.to_string(), profile.organization.id.clone());
            let bytes = serde_json::to_vec(&files).map_err(|_| "Registro de arquivos indisponível.".to_string())?;
            let temp = self.path.with_extension("next");
            use std::io::Write;
            let mut options = std::fs::OpenOptions::new();
            options.write(true).create(true).truncate(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut output = options.open(&temp).map_err(|_| "Não foi possível registrar a empresa do arquivo.".to_string())?;
            output
                .write_all(&bytes)
                .and_then(|()| output.sync_all())
                .and_then(|()| std::fs::rename(&temp, &self.path))
                .map_err(|_| "Não foi possível registrar a empresa do arquivo.".to_string())?;
            self.files = files;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    #[test]
    fn official_credentials_are_not_sent_to_invite_servers() {
        let endpoint = Endpoint { host: "example.test:443".into(), path: "/".into(), secure: true };
        assert!(request(&endpoint).unwrap().headers().get("authorization").is_none());
        let official = Endpoint { host: "collab.agcunae.com".into(), ..endpoint };
        assert!(request(&official).is_err());
    }
    #[test]
    fn file_organization_is_durable_and_cannot_be_reassigned() {
        let path = std::env::temp_dir().join(format!("novafig-org-test-{}.json", std::process::id()));
        let mut bindings = Bindings { path: path.clone(), files: BTreeMap::new() };
        let mut profile: Profile = serde_json::from_value(serde_json::json!({"user":{"id":"person-1","name":"Ana Oliveira","avatarUrl":null},"organization":{"id":"org-1","name":"Empresa de teste"},"departments":[],"apps":["novafig"],"policyEnabled":true,"canManage":false})).unwrap();
        bindings.check(&profile, "stable-document-id", true).unwrap();
        bindings.files = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        bindings.check(&profile, "stable-document-id", false).unwrap();
        profile.organization.id = "org-2".into();
        assert!(bindings.check(&profile, "stable-document-id", true).is_err());
        assert_eq!(bindings.files["stable-document-id"], "org-1");
        std::fs::remove_file(path).unwrap();
    }
}
