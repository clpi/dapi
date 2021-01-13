use serde::{Serialize, Deserialize};
//use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use jsonwebtoken::errors::Result as JWTResult;
use crate::models::User;
use chrono::{Utc, Duration};
use argonautica::{Hasher, Verifier, 
    config::Variant,
    input::{Salt, SecretKey}
};

#[derive(Deserialize, Serialize, Clone)]
pub struct UserSession {
    pub user: User,
    pub claims: Claims,
    pub privel: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: i32,
    exp: i32, 
}

impl Claims {
    pub fn new(uid: i32) -> Claims {
        let exp = (Utc::now() + Duration::weeks(2)).timestamp() as i32;
        Self { sub: uid, exp }
    }
}

pub async fn hash_pwd(key: &String, pwd: &String) -> String {
    let secret_key: SecretKey<'static> = 
        SecretKey::from_base64_encoded(key).unwrap();
    let mut hasher = Hasher::default();
    hasher.configure_secret_key_clearing(false)
        .configure_threads(2)
        .configure_variant(Variant::Argon2id)
        .with_salt(Salt::random(16))
        .with_secret_key(&secret_key)
        .with_password(pwd)
        .hash().unwrap()
}

pub async fn verify_pwd(key: &String, pwd: &String, db_pwd: &String) -> bool {
    let secret_key: SecretKey<'static> = 
        SecretKey::from_base64_encoded(key).unwrap();
    let mut verifier = Verifier::default();
    verifier.with_secret_key(&secret_key)
        .with_hash(&db_pwd)
        .with_password(pwd)
        .verify().unwrap()
}

pub async fn get_secret_key() -> Result<String, std::io::Error>  {
    let key = dotenv::var("SECRET_KEY")
        .expect("SECRET_KEY NOT SET");
    Ok(key)
}


//pub fn encode_jwt(secret: &String, user: &User) -> Result<String, String> {
    //match encode(
        //&Header::default(),
        //&Claims::new(user.id.unwrap()),
        //&EncodingKey::from_secret(secret.as_ref()),
    //) {
        //Ok(jwt) => Ok(jwt),
        //Err(_) => Err(String::from("Couldn't encode")),
    //}
//}

//pub fn decode_jwt(secret: &String, token: &String) -> JWTResult<Claims> {
    //match decode::<Claims>(
        //token.trim_start_matches("MEMT "),
        //&DecodingKey::from_secret(secret.as_ref()),
        //&Validation::default(),
    //) {
        //Ok(jwt) => Ok(jwt.claims),
        //Err(err) => Err(err),
    //}
//}

pub async fn get_jwt_secret() -> Result<String, std::io::Error> {
    Ok(dotenv::var("JWT_SECRET")
        .expect("JWT SECRET NOT SET"))
}
