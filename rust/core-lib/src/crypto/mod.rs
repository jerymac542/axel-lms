//! Cryptography
//!
//! Encryption, signing, key management, JWT/OAuth2 utilities for Apexos services.

use crate::config::{Argon2Config, Argon2Variant, HashAlgorithm, JwtAlgorithm, JwtConfig};
use crate::domain::Timestamp;
use crate::errors::{ErrorCode, Result};
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit, OsRng}};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, Algorithm, Version, Params};
use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey, Nonce as ChaChaNonce, aead::{Aead, KeyInit, OsRng}};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use jsonwebtoken::{decode, encode, Algorithm as JwtAlg, DecodingKey, EncodingKey, Header, Validation, TokenData};
use p256::ecdsa::{SigningKey as EcdsaSigningKey, VerifyingKey as EcdsaVerifyingKey, Signature as EcdsaSignature, signature::Signer as EcdsaSigner, signature::Verifier as EcdsaVerifier};
use rand::rngs::OsRng;
use ring::hmac;
use secrecy::{ExposeSecret, SecretString, SecretVec};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{TOTP, Algorithm as TotpAlgorithm};
use uuid::Uuid;
use x25519_dalek::{PublicKey as X25519Public, StaticSecret as X25519Secret};

/// Encryption algorithms supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
}

/// Encryption key wrapper
#[derive(Debug, Clone)]
pub struct EncryptionKey(pub SecretVec<u8>);

impl EncryptionKey {
    pub fn generate(algorithm: EncryptionAlgorithm) -> Self {
        let key_size = match algorithm {
            EncryptionAlgorithm::Aes256Gcm => 32,
            EncryptionAlgorithm::ChaCha20Poly1305 => 32,
        };
        let mut key = vec![0u8; key_size];
        OsRng.fill(&mut key);
        Self(SecretVec::new(key))
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(SecretVec::new(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.expose_secret()
    }
}

/// Encryptor for symmetric encryption
pub struct Encryptor {
    algorithm: EncryptionAlgorithm,
    key: EncryptionKey,
}

impl Encryptor {
    pub fn new(algorithm: EncryptionAlgorithm, key: EncryptionKey) -> Self {
        Self { algorithm, key }
    }

    pub fn encrypt(&self, plaintext: &[u8], associated_data: Option<&[u8]>) -> Result<Vec<u8>> {
        match self.algorithm {
            EncryptionAlgorithm::Aes256Gcm => {
                let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(self.key.as_bytes()));
                let mut nonce_bytes = [0u8; 12];
                OsRng.fill(&mut nonce_bytes);
                let nonce = Nonce::from_slice(&nonce_bytes);
                
                let ciphertext = cipher.encrypt(nonce, aes_gcm::aead::Payload {
                    msg: plaintext,
                    aad: associated_data.unwrap_or_default(),
                })?;

                // Prepend nonce to ciphertext
                let mut result = nonce_bytes.to_vec();
                result.extend_from_slice(&ciphertext);
                Ok(result)
            }
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(self.key.as_bytes()));
                let mut nonce_bytes = [0u8; 12];
                OsRng.fill(&mut nonce_bytes);
                let nonce = ChaChaNonce::from_slice(&nonce_bytes);
                
                let ciphertext = cipher.encrypt(nonce, chacha20poly1305::aead::Payload {
                    msg: plaintext,
                    aad: associated_data.unwrap_or_default(),
                })?;

                let mut result = nonce_bytes.to_vec();
                result.extend_from_slice(&ciphertext);
                Ok(result)
            }
        }
    }

    pub fn decrypt(&self, ciphertext: &[u8], associated_data: Option<&[u8]>) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Ciphertext too short"));
        }

        let (nonce_bytes, ciphertext) = ciphertext.split_at(12);

        match self.algorithm {
            EncryptionAlgorithm::Aes256Gcm => {
                let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(self.key.as_bytes()));
                let nonce = Nonce::from_slice(nonce_bytes);
                
                cipher.decrypt(nonce, aes_gcm::aead::Payload {
                    msg: ciphertext,
                    aad: associated_data.unwrap_or_default(),
                }).map_err(|_| crate::errors::ApexosError::new(ErrorCode::InternalError, "Decryption failed"))
            }
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(self.key.as_bytes()));
                let nonce = ChaChaNonce::from_slice(nonce_bytes);
                
                cipher.decrypt(nonce, chacha20poly1305::aead::Payload {
                    msg: ciphertext,
                    aad: associated_data.unwrap_or_default(),
                }).map_err(|_| crate::errors::ApexosError::new(ErrorCode::InternalError, "Decryption failed"))
            }
        }
    }

    pub fn encrypt_string(&self, plaintext: &str) -> Result<String> {
        let encrypted = self.encrypt(plaintext.as_bytes(), None)?;
        Ok(general_purpose::STANDARD.encode(encrypted))
    }

    pub fn decrypt_string(&self, ciphertext: &str) -> Result<String> {
        let decoded = general_purpose::STANDARD.decode(ciphertext)?;
        let decrypted = self.decrypt(&decoded, None)?;
        Ok(String::from_utf8(decrypted)?)
    }
}

/// Signer for asymmetric signatures
pub struct Signer {
    algorithm: SigningAlgorithm,
    signing_key: SigningKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SigningAlgorithm {
    Ed25519,
    ES256,
    ES384,
    ES512,
    RS256,
    RS384,
    RS512,
}

impl Signer {
    pub fn new_ed25519() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self {
            algorithm: SigningAlgorithm::Ed25519,
            signing_key,
        }
    }

    pub fn from_ed25519_bytes(bytes: &[u8]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(bytes.try_into().map_err(|_| crate::errors::ApexosError::new(ErrorCode::InternalError, "Invalid key length"))?);
        Ok(Self {
            algorithm: SigningAlgorithm::Ed25519,
            signing_key,
        })
    }

    pub fn from_pkcs8(pem: &str) -> Result<Self> {
        // Parse PKCS#8 private key
        let der = pem::parse(pem).map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;
        let pkcs8 = pkcs8::PrivateKeyInfo::try_from(der.contents()).map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;
        
        // Try Ed25519 first
        if let Ok(key) = ed25519_dalek::SigningKey::try_from(pkcs8) {
            return Ok(Self {
                algorithm: SigningAlgorithm::Ed25519,
                signing_key: key,
            });
        }

        // Try ECDSA P-256
        if let Ok(key) = EcdsaSigningKey::try_from(pkcs8) {
            return Ok(Self {
                algorithm: SigningAlgorithm::ES256,
                signing_key: SigningKey::from_bytes(&key.to_bytes()),
            });
        }

        Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Unsupported key format"))
    }

    pub fn algorithm(&self) -> SigningAlgorithm {
        self.algorithm
    }

    pub fn public_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            SigningAlgorithm::Ed25519 => {
                let signature = self.signing_key.sign(message);
                Ok(signature.to_bytes().to_vec())
            }
            _ => Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Algorithm not implemented")),
        }
    }

    pub fn sign_detached(&self, message: &[u8]) -> Result<Signature> {
        match self.algorithm {
            SigningAlgorithm::Ed25519 => {
                Ok(self.signing_key.sign(message))
            }
            _ => Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Algorithm not implemented")),
        }
    }
}

/// Verifier for asymmetric signatures
pub struct Verifier {
    algorithm: SigningAlgorithm,
    verifying_key: VerifyingKey,
}

impl Verifier {
    pub fn new_ed25519(verifying_key: VerifyingKey) -> Self {
        Self {
            algorithm: SigningAlgorithm::Ed25519,
            verifying_key,
        }
    }

    pub fn from_ed25519_bytes(bytes: &[u8]) -> Result<Self> {
        let verifying_key = VerifyingKey::from_bytes(bytes.try_into().map_err(|_| crate::errors::ApexosError::new(ErrorCode::InternalError, "Invalid key length"))?);
        Ok(Self {
            algorithm: SigningAlgorithm::Ed25519,
            verifying_key,
        })
    }

    pub fn from_pem(pem: &str) -> Result<Self> {
        let der = pem::parse(pem).map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;
        let spki = spki::SubjectPublicKeyInfo::try_from(der.contents()).map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;
        
        if let Ok(key) = VerifyingKey::try_from(&spki) {
            return Ok(Self::new_ed25519(key));
        }

        Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Unsupported key format"))
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
        match self.algorithm {
            SigningAlgorithm::Ed25519 => {
                let sig = Signature::from_bytes(signature.try_into().map_err(|_| crate::errors::ApexosError::new(ErrorCode::InternalError, "Invalid signature length"))?);
                Ok(self.verifying_key.verify(message, &sig).is_ok())
            }
            _ => Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Algorithm not implemented")),
        }
    }
}

/// Key manager for encryption and signing keys
pub struct KeyManager {
    encryption_keys: dashmap::DashMap<String, EncryptionKey>,
    signing_keys: dashmap::DashMap<String, Signer>,
    current_encryption_key: String,
    current_signing_key: String,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            encryption_keys: dashmap::DashMap::new(),
            signing_keys: dashmap::DashMap::new(),
            current_encryption_key: "default".to_string(),
            current_signing_key: "default".to_string(),
        }
    }

    pub fn add_encryption_key(&self, id: String, key: EncryptionKey) {
        self.encryption_keys.insert(id, key);
    }

    pub fn add_signing_key(&self, id: String, key: Signer) {
        self.signing_keys.insert(id, key);
    }

    pub fn set_current_encryption_key(&mut self, id: String) -> Result<()> {
        if self.encryption_keys.contains_key(&id) {
            self.current_encryption_key = id;
            Ok(())
        } else {
            Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Encryption key not found"))
        }
    }

    pub fn set_current_signing_key(&mut self, id: String) -> Result<()> {
        if self.signing_keys.contains_key(&id) {
            self.current_signing_key = id;
            Ok(())
        } else {
            Err(crate::errors::ApexosError::new(ErrorCode::InternalError, "Signing key not found"))
        }
    }

    pub fn get_encryption_key(&self, id: Option<&str>) -> Option<EncryptionKey> {
        let id = id.unwrap_or(&self.current_encryption_key);
        self.encryption_keys.get(id).map(|v| v.clone())
    }

    pub fn get_signing_key(&self, id: Option<&str>) -> Option<Signer> {
        let id = id.unwrap_or(&self.current_signing_key);
        self.signing_keys.get(id).map(|v| v.clone())
    }

    pub fn get_verifying_key(&self, id: Option<&str>) -> Option<VerifyingKey> {
        self.get_signing_key(id).map(|s| s.public_key())
    }

    pub fn encryptor(&self, key_id: Option<&str>, algorithm: EncryptionAlgorithm) -> Result<Encryptor> {
        let key = self.get_encryption_key(key_id).ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "Encryption key not found"))?;
        Ok(Encryptor::new(algorithm, key))
    }

    pub fn signer(&self, key_id: Option<&str>) -> Result<Signer> {
        self.get_signing_key(key_id).ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "Signing key not found"))
    }

    pub fn verifier(&self, key_id: Option<&str>) -> Result<Verifier> {
        let key = self.get_verifying_key(key_id).ok_or_else(|| crate::errors::ApexosError::new(ErrorCode::InternalError, "Verifying key not found"))?;
        Ok(Verifier::new_ed25519(key))
    }

    pub fn rotate_encryption_key(&self, new_key: EncryptionKey) -> String {
        let id = format!("key-{}", Uuid::now_v7());
        self.encryption_keys.insert(id.clone(), new_key);
        id
    }

    pub fn rotate_signing_key(&self) -> String {
        let signer = Signer::new_ed25519();
        let id = format!("signing-key-{}", Uuid::now_v7());
        self.signing_keys.insert(id.clone(), signer);
        id
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.add_encryption_key("default".to_string(), EncryptionKey::generate(EncryptionAlgorithm::Aes256Gcm));
        manager.add_signing_key("default".to_string(), Signer::new_ed25519());
        manager
    }
}

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iss: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub exp: i64,
    pub iat: i64,
    pub nbf: Option<i64>,
    pub jti: String,
    pub scope: Vec<String>,
    pub permissions: Vec<String>,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub roles: Vec<String>,
    pub session_id: Option<String>,
    pub mfa_verified: bool,
    #[serde(flatten)]
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}

impl JwtClaims {
    pub fn new(
        issuer: &str,
        subject: &str,
        audience: Vec<String>,
        ttl: std::time::Duration,
        scope: Vec<String>,
    ) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        Self {
            iss: issuer.to_string(),
            sub: subject.to_string(),
            aud: audience,
            exp: now + ttl.as_secs() as i64,
            iat: now,
            nbf: Some(now),
            jti: Uuid::now_v7().to_string(),
            scope,
            permissions: Vec::new(),
            tenant_id: None,
            organization_id: None,
            roles: Vec::new(),
            session_id: None,
            mfa_verified: false,
            custom: std::collections::HashMap::new(),
        }
    }

    pub fn with_tenant(mut self, tenant_id: &str) -> Self {
        self.tenant_id = Some(tenant_id.to_string());
        self
    }

    pub fn with_organization(mut self, org_id: &str) -> Self {
        self.organization_id = Some(org_id.to_string());
        self
    }

    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.roles = roles;
        self
    }

    pub fn with_permissions(mut self, permissions: Vec<String>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_session(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }

    pub fn with_mfa(mut self, verified: bool) -> Self {
        self.mfa_verified = verified;
        self
    }

    pub fn with_custom(mut self, key: &str, value: serde_json::Value) -> Self {
        self.custom.insert(key.to_string(), value);
        self
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        now >= self.exp
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        self.scope.iter().any(|s| s == scope || s == "*")
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission || p == "*")
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}

/// JWT token verifier
pub struct TokenVerifier {
    config: JwtConfig,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl TokenVerifier {
    pub fn new(config: &JwtConfig) -> Result<Self> {
        let decoding_key = match config.algorithm {
            JwtAlgorithm::HS256 | JwtAlgorithm::HS384 | JwtAlgorithm::HS512 => {
                DecodingKey::from_secret(config.signing_key.expose_secret().as_bytes())
            }
            JwtAlgorithm::RS256 | JwtAlgorithm::RS384 | JwtAlgorithm::RS512 => {
                DecodingKey::from_rsa_pem(config.signing_key.expose_secret().as_bytes())?
            }
            JwtAlgorithm::ES256 | JwtAlgorithm::ES384 | JwtAlgorithm::ES512 => {
                DecodingKey::from_ec_pem(config.signing_key.expose_secret().as_bytes())?
            }
            JwtAlgorithm::EdDSA => {
                DecodingKey::from_ed_pem(config.signing_key.expose_secret().as_bytes())?
            }
        };

        let mut validation = Validation::new(match config.algorithm {
            JwtAlgorithm::HS256 => JwtAlg::HS256,
            JwtAlgorithm::HS384 => JwtAlg::HS384,
            JwtAlgorithm::HS512 => JwtAlg::HS512,
            JwtAlgorithm::RS256 => JwtAlg::RS256,
            JwtAlgorithm::RS384 => JwtAlg::RS384,
            JwtAlgorithm::RS512 => JwtAlg::RS512,
            JwtAlgorithm::ES256 => JwtAlg::ES256,
            JwtAlgorithm::ES384 => JwtAlg::ES384,
            JwtAlgorithm::ES512 => JwtAlg::ES512,
            JwtAlgorithm::EdDSA => JwtAlg::EdDSA,
        });

        validation.set_issuer(&[config.issuer.clone()]);
        validation.set_audience(&config.audience);
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.leeway = 60;
        validation.required_spec_claims = std::collections::HashSet::from(["exp", "iat", "iss", "sub", "aud", "jti"]);

        Ok(Self {
            config: config.clone(),
            decoding_key,
            validation,
        })
    }

    pub fn verify(&self, token: &str) -> Result<TokenData<JwtClaims>> {
        let mut validation = self.validation.clone();
        
        // If JWKS URI is configured, we'd need to fetch keys dynamically
        // For now, use the static key
        let data = decode::<JwtClaims>(token, &self.decoding_key, &validation)?;
        Ok(data)
    }

    pub fn verify_without_validation(&self, token: &str) -> Result<TokenData<JwtClaims>> {
        let mut validation = Validation::new(match self.config.algorithm {
            JwtAlgorithm::HS256 => JwtAlg::HS256,
            JwtAlgorithm::HS384 => JwtAlg::HS384,
            JwtAlgorithm::HS512 => JwtAlg::HS512,
            JwtAlgorithm::RS256 => JwtAlg::RS256,
            JwtAlgorithm::RS384 => JwtAlg::RS384,
            JwtAlgorithm::RS512 => JwtAlg::RS512,
            JwtAlgorithm::ES256 => JwtAlg::ES256,
            JwtAlgorithm::ES384 => JwtAlg::ES384,
            JwtAlgorithm::ES512 => JwtAlg::ES512,
            JwtAlgorithm::EdDSA => JwtAlg::EdDSA,
        });
        validation.validate_exp = false;
        validation.validate_nbf = false;
        validation.validate_aud = false;
        validation.validate_iss = false;

        let data = decode::<JwtClaims>(token, &self.decoding_key, &validation)?;
        Ok(data)
    }
}

/// JWT token generator
pub struct TokenGenerator {
    config: JwtConfig,
    encoding_key: EncodingKey,
    header: Header,
}

impl TokenGenerator {
    pub fn new(config: &JwtConfig) -> Result<Self> {
        let encoding_key = match config.algorithm {
            JwtAlgorithm::HS256 | JwtAlgorithm::HS384 | JwtAlgorithm::HS512 => {
                EncodingKey::from_secret(config.signing_key.expose_secret().as_bytes())
            }
            JwtAlgorithm::RS256 | JwtAlgorithm::RS384 | JwtAlgorithm::RS512 => {
                EncodingKey::from_rsa_pem(config.signing_key.expose_secret().as_bytes())?
            }
            JwtAlgorithm::ES256 | JwtAlgorithm::ES384 | JwtAlgorithm::ES512 => {
                EncodingKey::from_ec_pem(config.signing_key.expose_secret().as_bytes())?
            }
            JwtAlgorithm::EdDSA => {
                EncodingKey::from_ed_pem(config.signing_key.expose_secret().as_bytes())?
            }
        };

        let mut header = Header::new(match config.algorithm {
            JwtAlgorithm::HS256 => JwtAlg::HS256,
            JwtAlgorithm::HS384 => JwtAlg::HS384,
            JwtAlgorithm::HS512 => JwtAlg::HS512,
            JwtAlgorithm::RS256 => JwtAlg::RS256,
            JwtAlgorithm::RS384 => JwtAlg::RS384,
            JwtAlgorithm::RS512 => JwtAlg::RS512,
            JwtAlgorithm::ES256 => JwtAlg::ES256,
            JwtAlgorithm::ES384 => JwtAlg::ES384,
            JwtAlgorithm::ES512 => JwtAlg::ES512,
            JwtAlgorithm::EdDSA => JwtAlg::EdDSA,
        });

        if let Some(kid) = &config.key_id {
            header.kid = Some(kid.clone());
        }

        Ok(Self {
            config: config.clone(),
            encoding_key,
            header,
        })
    }

    pub fn generate_access_token(&self, claims: JwtClaims) -> Result<String> {
        encode(&self.header, &claims, &self.encoding_key).map_err(Into::into)
    }

    pub fn generate_refresh_token(&self, claims: JwtClaims) -> Result<String> {
        // Refresh tokens use the same signing but different claims
        encode(&self.header, &claims, &self.encoding_key).map_err(Into::into)
    }

    pub fn generate_id_token(&self, claims: JwtClaims) -> Result<String> {
        encode(&self.header, &claims, &self.encoding_key).map_err(Into::into)
    }
}

/// Password hashing using Argon2
pub struct PasswordHasher {
    config: Argon2Config,
}

impl PasswordHasher {
    pub fn new(config: Argon2Config) -> Self {
        Self { config }
    }

    pub fn hash(&self, password: &[u8]) -> Result<String> {
        let salt = Self::generate_salt(self.config.salt_length)?;
        let argon2 = self.build_argon2()?;
        
        let hash = argon2.hash_password(password, &salt)?
            .to_string();
        
        Ok(hash)
    }

    pub fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;
        
        let argon2 = self.build_argon2()?;
        Ok(argon2.verify_password(password, &parsed_hash).is_ok())
    }

    fn build_argon2(&self) -> Result<Argon2> {
        let params = Params::new(
            self.config.memory_cost,
            self.config.time_cost,
            self.config.parallelism,
            Some(self.config.hash_length),
        ).map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))?;

        let algorithm = match self.config.variant {
            Argon2Variant::Argon2d => Algorithm::Argon2d,
            Argon2Variant::Argon2i => Algorithm::Argon2i,
            Argon2Variant::Argon2id => Algorithm::Argon2id,
        };

        Ok(Argon2::new(algorithm, Version::V0x13, params))
    }

    fn generate_salt(length: usize) -> Result<argon2::password_hash::SaltString> {
        let mut salt = vec![0u8; length];
        OsRng.fill(&mut salt);
        argon2::password_hash::SaltString::encode_b64(&salt)
            .map_err(|e| crate::errors::ApexosError::new(ErrorCode::InternalError, e.to_string()))
    }
}

/// TOTP (Time-based One-Time Password) utilities
pub struct TotpManager;

impl TotpManager {
    pub fn generate_secret() -> String {
        base32::encode(base32::Alphabet::RFC4648 { padding: false }, &OsRng.gen::<[u8; 20]>())
    }

    pub fn generate_uri(
        issuer: &str,
        account: &str,
        secret: &str,
        digits: u32,
        period: u64,
        algorithm: TotpAlgorithm,
    ) -> String {
        let totp = TOTP::new(
            algorithm.into(),
            digits,
            period,
            1,
            secret.as_bytes().to_vec(),
        ).unwrap();
        
        totp.get_uri(issuer, account)
    }

    pub fn verify(
        secret: &str,
        code: &str,
        digits: u32,
        period: u64,
        algorithm: TotpAlgorithm,
        skew: u32,
    ) -> bool {
        let totp = TOTP::new(
            algorithm.into(),
            digits,
            period,
            skew,
            secret.as_bytes().to_vec(),
        ).unwrap();
        
        totp.check_current(code).unwrap_or(false)
    }
}

impl From<crate::config::TotpAlgorithm> for totp_rs::Algorithm {
    fn from(algo: crate::config::TotpAlgorithm) -> Self {
        match algo {
            crate::config::TotpAlgorithm::SHA1 => totp_rs::Algorithm::SHA1,
            crate::config::TotpAlgorithm::SHA256 => totp_rs::Algorithm::SHA256,
            crate::config::TotpAlgorithm::SHA512 => totp_rs::Algorithm::SHA512,
        }
    }
}

/// HKDF (HMAC-based Key Derivation Function)
pub struct Hkdf;

impl Hkdf {
    pub fn derive(
        algorithm: hmac::Algorithm,
        salt: &[u8],
        ikm: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>> {
        use ring::hkdf;
        
        let salt = hkdf::Salt::new(algorithm, salt);
        let prk = salt.extract(ikm);
        let mut okm = vec![0u8; output_length];
        prk.expand(&[info], algorithm, &mut okm)
            .map_err(|_| crate::errors::ApexosError::new(ErrorCode::InternalError, "HKDF expansion failed"))?;
        
        Ok(okm)
    }
}

/// X25519 key agreement (for E2E encryption)
pub struct KeyAgreement;

impl KeyAgreement {
    pub fn generate_keypair() -> (X25519Secret, X25519Public) {
        let secret = X25519Secret::random(&mut OsRng);
        let public = X25519Public::from(&secret);
        (secret, public)
    }

    pub fn agree(secret: &X25519Secret, public: &X25519Public) -> [u8; 32] {
        secret.diffie_hellman(public).to_bytes()
    }
}

/// Secure random utilities
pub struct SecureRandom;

impl SecureRandom {
    pub fn bytes(len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        OsRng.fill(&mut bytes);
        bytes
    }

    pub fn string(len: usize) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut result = String::with_capacity(len);
        for _ in 0..len {
            let idx = (OsRng.next_u64() % CHARS.len() as u64) as usize;
            result.push(CHARS[idx] as char);
        }
        result
    }

    pub fn alphanumeric(len: usize) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut result = String::with_capacity(len);
        for _ in 0..len {
            let idx = (OsRng.next_u64() % CHARS.len() as u64) as usize;
            result.push(CHARS[idx] as char);
        }
        result
    }

    pub fn numeric(len: usize) -> String {
        let mut result = String::with_capacity(len);
        for _ in 0..len {
            let digit = (OsRng.next_u64() % 10) as u8;
            result.push((b'0' + digit) as char);
        }
        result
    }

    pub fn uuid_v7() -> Uuid {
        Uuid::now_v7()
    }
}

/// Constant-time comparison
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    use subtle::ConstantTimeEq;
    a.ct_eq(b).into()
}

/// Secure memory clearing
pub fn zeroize_memory(data: &mut [u8]) {
    use zeroize::Zeroize;
    data.zeroize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption_roundtrip() {
        let key = EncryptionKey::generate(EncryptionAlgorithm::Aes256Gcm);
        let encryptor = Encryptor::new(EncryptionAlgorithm::Aes256Gcm, key);
        
        let plaintext = "Hello, World!";
        let encrypted = encryptor.encrypt_string(plaintext).unwrap();
        let decrypted = encryptor.decrypt_string(&encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn chacha_encryption_roundtrip() {
        let key = EncryptionKey::generate(EncryptionAlgorithm::ChaCha20Poly1305);
        let encryptor = Encryptor::new(EncryptionAlgorithm::ChaCha20Poly1305, key);
        
        let plaintext = "Hello, ChaCha!";
        let encrypted = encryptor.encrypt_string(plaintext).unwrap();
        let decrypted = encryptor.decrypt_string(&encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn ed25519_sign_verify() {
        let signer = Signer::new_ed25519();
        let verifier = Verifier::new_ed25519(signer.public_key());
        
        let message = b"Test message";
        let signature = signer.sign(message).unwrap();
        
        assert!(verifier.verify(message, &signature).unwrap());
        assert!(!verifier.verify(b"Different message", &signature).unwrap());
    }

    #[test]
    fn jwt_generation_verification() {
        let config = JwtConfig {
            issuer: "test".to_string(),
            audience: vec!["test".to_string()],
            access_token_ttl: std::time::Duration::from_secs(3600),
            refresh_token_ttl: std::time::Duration::from_secs(86400),
            id_token_ttl: std::time::Duration::from_secs(3600),
            signing_key: SecretString::new("test-secret-key-at-least-32-bytes-long".into()),
            verification_key: None,
            algorithm: JwtAlgorithm::HS256,
            key_id: None,
            jwks_uri: None,
            jwks_cache_ttl: std::time::Duration::from_secs(3600),
        };

        let generator = TokenGenerator::new(&config).unwrap();
        let verifier = TokenVerifier::new(&config).unwrap();

        let claims = JwtClaims::new("test", "user123", vec!["test".to_string()], std::time::Duration::from_secs(3600), vec!["read".to_string()]);
        
        let token = generator.generate_access_token(claims).unwrap();
        let decoded = verifier.verify(&token).unwrap();
        
        assert_eq!(decoded.claims.sub, "user123");
        assert!(!decoded.claims.is_expired());
    }

    #[test]
    fn password_hashing() {
        let config = Argon2Config {
            variant: Argon2Variant::Argon2id,
            version: 19,
            memory_cost: 65536,
            time_cost: 3,
            parallelism: 4,
            hash_length: 32,
            salt_length: 16,
        };

        let hasher = PasswordHasher::new(config);
        let password = b"TestPassword123!";
        
        let hash = hasher.hash(password).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"WrongPassword", &hash).unwrap());
    }

    #[test]
    fn totp_generation_verification() {
        let secret = TotpManager::generate_secret();
        let uri = TotpManager::generate_uri("TestApp", "user@example.com", &secret, 6, 30, TotpAlgorithm::SHA1);
        
        assert!(uri.starts_with("otpauth://totp/"));
        assert!(uri.contains("TestApp"));
        assert!(uri.contains("user@example.com"));
    }

    #[test]
    fn secure_random() {
        let s1 = SecureRandom::string(32);
        let s2 = SecureRandom::string(32);
        assert_eq!(s1.len(), 32);
        assert_ne!(s1, s2);
        
        let num = SecureRandom::numeric(6);
        assert_eq!(num.len(), 6);
        assert!(num.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn key_agreement() {
        let (secret1, public1) = KeyAgreement::generate_keypair();
        let (secret2, public2) = KeyAgreement::generate_keypair();
        
        let shared1 = KeyAgreement::agree(&secret1, &public2);
        let shared2 = KeyAgreement::agree(&secret2, &public1);
        
        assert_eq!(shared1, shared2);
    }

    #[test]
    fn hkdf() {
        let salt = b"salt";
        let ikm = b"input key material";
        let info = b"info";
        
        let derived = Hkdf::derive(hmac::HMAC_SHA256, salt, ikm, info, 32).unwrap();
        assert_eq!(derived.len(), 32);
        
        let derived2 = Hkdf::derive(hmac::HMAC_SHA256, salt, ikm, info, 32).unwrap();
        assert_eq!(derived, derived2);
    }
}