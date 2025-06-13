use rand::{rngs::OsRng, TryRngCore};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime};
use hex;

// Human readable alphabet (a-z, 0-9 without l, o, 0, 1 to avoid confusion)
const READABLE_ALPHABET: &[u8] = b"abcdefghijkmnpqrstuvwxyz23456789";

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Session {
    id: String,
    secret_hash: String,
    created_at: i64,
    user: u32
}

fn gen_secure_random_str() -> Option<String> {
    let mut rand_bytes = [0u8;32];
    OsRng.try_fill_bytes(&mut rand_bytes).ok()?;
    let mut result = String::new();
    for rand in rand_bytes {
        let i = (rand >> 3) as usize;
        result.push(READABLE_ALPHABET[i] as char);
    }
    return Some(result);
}

pub async fn create_session(pool: &SqlitePool, user_id: u32) -> Result<Option<(Session, String)>, sqlx::Error> {
    let now = OffsetDateTime::now_utc().unix_timestamp(); 
    let (id, secret) = match (gen_secure_random_str(), gen_secure_random_str()) {
        (Some(id), Some(secret)) => (id, secret),
        _ => return Ok(None)
    };
    let secret_hash = hex::encode(Sha256::digest(secret.clone()));

    let token = id.clone() + "." + &secret;

    let session = Session {
        id: id.clone(), secret_hash: secret_hash.clone(), created_at: now, user: user_id
    };
    
    sqlx::query("
        INSERT INTO Session (id, secret_hash, created_at, user)
        VALUES (?, ?, ?, ?)").bind(id).bind(secret_hash).bind(now).bind(user_id)
        .execute(pool).await?;
    
    return Ok(Some((session, token)));
}

pub async fn validate_session(pool: &SqlitePool, token: String) -> Result<Option<Session>, sqlx::Error> {
    let (id, secret) = match token.splitn(2, '.').collect::<Vec<_>>().as_slice() {
        [id, secret] => (id.to_string(), secret.to_string()),
        _ => return Ok(None)
    };

    let session = get_session(pool, id).await?;

    if let Some(session) = session {
        let token_secret_hash = Sha256::digest(secret).to_vec();
        if let Ok(db_secret_hash) = hex::decode(session.secret_hash.clone()) {
            if eq_hashes(token_secret_hash, db_secret_hash) {
                return Ok(Some(session));
            }
        }
    }

    Ok(None)
}

fn eq_hashes(hash1: Vec<u8>, hash2: Vec<u8>) -> bool {
    if hash1.len() != hash2.len() {
        return false;
    }
    for i in 0..hash1.len() {
        if hash1[i] != hash2[i] {
            return false;
        }
    }
    return true;
}


async fn get_session(pool: &SqlitePool, session_id: String) -> Result<Option<Session>, sqlx::Error> {
    let now = OffsetDateTime::now_utc().unix_timestamp(); 
    
    let session: Option<Session> = sqlx::query_as("
        SELECT id, secret_hash, created_at, user
        FROM Session
        WHERE id = ?").bind(&session_id).fetch_optional(pool).await?;
    
    let Some(session) = session else {
        return Ok(None);
    };

    if now - session.created_at < Duration::days(7).whole_seconds() {
        return Ok(Some(session));
    } else {
        delete_session(pool, session_id).await?;
        return Ok(None);
    }
}

async fn delete_session(pool: &SqlitePool, session_id: String) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM Session WHERE id = ?").bind(session_id).execute(pool).await?;
    Ok(())
}
