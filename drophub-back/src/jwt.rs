use base64::{engine::general_purpose::URL_SAFE as B64_ENGINE, Engine};
use drophub::{ClientId, JwtEncoded, RoomError, RoomId};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Jwt {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    span: tracing::Span,
}

impl Jwt {
    pub fn new(access_token: AccessToken, refresh_token: RefreshToken) -> Self {
        let span = Self::create_span(&access_token);
        Self {
            access_token,
            refresh_token,
            span,
        }
    }

    pub fn encode(&self, secret: &str) -> Result<JwtEncoded, JwtError> {
        Ok(JwtEncoded {
            access_token: self.access_token.encode(secret)?,
            refresh_token: self.refresh_token.encode()?,
        })
    }

    pub fn decode(enc: &JwtEncoded, secret: &str) -> Result<Self, JwtError> {
        let access_token = AccessToken::decode(secret, &enc.access_token)?;
        let span = Self::create_span(&access_token);
        Ok(Self {
            access_token,
            refresh_token: RefreshToken::decode(&enc.refresh_token)?,
            span,
        })
    }

    pub fn span(&self) -> &tracing::Span {
        &self.span
    }

    fn create_span(access_token: &AccessToken) -> tracing::Span {
        tracing::info_span!(parent: tracing::Span::current(),
            "jwt",
            "room.id" = ?access_token.room_id,
            "client.id" = ?access_token.client_id,
            "client.role" = ?access_token.role,
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    #[error(transparent)]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),
}

impl From<JwtError> for RoomError {
    fn from(f: JwtError) -> Self {
        RoomError::Other(f.into())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientRole {
    Host,
    Guest,
}

/// Token for accessing resources.
/// After expired needs to refresh access token via refresh token.
///
/// Can be encoded to JWT format `"header.payload.signature"`.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    pub client_id: ClientId,
    pub room_id: RoomId,
    pub role: ClientRole,
    #[serde(with = "jwt_numeric_date")]
    pub exp: Option<OffsetDateTime>,
}

impl AccessToken {
    /// Encodes token to JWT format.
    pub fn encode(&self, secret: &str) -> Result<String, JwtError> {
        let tok = jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        tracing::debug!("encoded access token: {:?}", tok);
        Ok(tok)
    }

    /// Decodes token from JWT format.
    pub fn decode(secret: &str, token: &str) -> Result<Self, JwtError> {
        let validation = {
            let mut v = Validation::default();
            v.set_required_spec_claims::<&str>(&[]);
            v
        };

        let tok = jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map(|token_data| token_data.claims)?;

        tracing::debug!("decoded access token: {:?}", tok);
        Ok(tok)
    }
}

/// Token to refreshing access token.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: Uuid,
    #[serde(with = "jwt_numeric_date")]
    pub exp: Option<OffsetDateTime>,
}

impl RefreshToken {
    /// Encodes token into base64 encoded json.
    pub fn encode(&self) -> Result<String, JwtError> {
        let tok_json = serde_json::to_string(self)?;
        tracing::debug!("encoded refresh token (json): {:?}", tok_json);

        let tok_b64 = B64_ENGINE.encode(tok_json);
        tracing::debug!("encoded refresh token (base64): {:?}", tok_b64);

        Ok(tok_b64)
    }

    /// Decodes token from base64 encoded json.
    pub fn decode(token: &str) -> Result<Self, JwtError> {
        let tok_b64 = B64_ENGINE.decode(token)?;
        let tok = serde_json::from_slice::<Self>(&tok_b64)?;

        tracing::debug!("decoded refresh token: {:?}", tok);
        Ok(tok)
    }
}

mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            None => serializer.serialize_none(),
            Some(date) => {
                let timestamp = date.unix_timestamp();
                serializer.serialize_i64(timestamp)
            }
        }
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let Some(ts ) = Option::<i64>::deserialize(deserializer)? else { return Ok(None) };
        let dt = OffsetDateTime::from_unix_timestamp(ts)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))?;

        Ok(Some(dt))
    }
}
