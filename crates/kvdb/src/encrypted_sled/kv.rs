// Copyright (C) 2023 Entropy Cryptography Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Wrap [sled] with [chacha20poly1305] encryption. An password-based key derivation function is
//! used as a [XChaCha20Poly1305](chacha20poly1305::XChaCha20Poly1305) cipher key to create an
//! [EncryptedDb]. A new random [chacha20poly1305::XNonce] is created every time a new value needs
//! to be inserted, forming a [EncryptedRecord]:<encrypted value, nonce>. The nonce is later
//! used to decrypt and retrieve the originally inserted value.

use chacha20poly1305::{
    self,
    aead::{AeadInPlace, NewAead},
    XChaCha20Poly1305,
};
use rand::RngCore;
use sled::IVec;
use zeroize::Zeroize;

use super::{
    constants::*,
    record::EncryptedRecord,
    result::{EncryptedDbError::*, EncryptedDbResult},
};

/// A [sled] kv store with [XChaCha20Poly1305] value encryption.
pub struct EncryptedDb {
    kv: sled::Db,
    cipher: XChaCha20Poly1305,
}

impl EncryptedDb {
    /// create a new [EncryptedDb] that wraps sled::open(db_name).
    /// Creates an XChaCha20 stream cipher from a password-based-key-derivation-function and
    /// verifies that the password is valid.
    /// See [super::Password] for more info on pdkdf.
    pub fn open<P>(db_name: P, mut key: [u8; 32]) -> EncryptedDbResult<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let kv = sled::open(db_name).map_err(CorruptedKv)?;

        let cipher = XChaCha20Poly1305::new(&key.into());
        key.zeroize();

        let encrypted_db = EncryptedDb { kv, cipher };

        // verify that [super::Password] is correct
        if encrypted_db.kv.was_recovered() {
            // existing kv: can we decrypt the verification value?
            encrypted_db.get(PASSWORD_VERIFICATION_KEY).map_err(|_| WrongPassword)?;
        } else {
            // new kv: encrypt the verification value
            encrypted_db.insert(PASSWORD_VERIFICATION_KEY, PASSWORD_VERIFICATION_VALUE)?;
        }

        Ok(encrypted_db)
    }

    /// get a new random nonce to use for value encryption using [rand::thread_rng]
    fn generate_nonce() -> chacha20poly1305::XNonce {
        let mut bytes = chacha20poly1305::XNonce::default();
        rand::thread_rng().fill_bytes(bytes.as_mut_slice());
        bytes
    }

    /// create a new [EncryptedRecord] containing an encrypted value and a newly derived random
    /// nonce
    fn encrypt<V>(&self, value: V) -> EncryptedDbResult<EncryptedRecord>
    where
        V: Into<IVec>,
    {
        let nonce = Self::generate_nonce();

        let mut value = value.into().to_vec();

        // encrypt value
        self.cipher
            .encrypt_in_place(&nonce, b"", &mut value)
            .map_err(|e| Encryption(e.to_string()))?;

        // return record
        Ok(EncryptedRecord::new(value, nonce))
    }

    /// derive a decrypted value from a [EncryptedRecord] containing an encrypted value and a random
    /// nonce
    fn decrypt_record_value(&self, record: EncryptedRecord) -> EncryptedDbResult<IVec> {
        let (mut value, nonce) = record.into();

        // decrypt value
        self.cipher
            .decrypt_in_place(&nonce, b"", &mut value)
            .map_err(|e| Decryption(e.to_string()))?;

        // return decrypted value
        Ok(value.into())
    }

    /// derive a decrypted value from [EncryptedRecord] bytes
    fn decrypt(&self, record_bytes: Option<IVec>) -> EncryptedDbResult<Option<IVec>> {
        let res = match record_bytes {
            Some(record_bytes) => {
                let record = EncryptedRecord::from_bytes(&record_bytes)?;
                let decrypted_value_bytes = self.decrypt_record_value(record)?;
                Some(decrypted_value_bytes)
            },
            None => None,
        };
        Ok(res)
    }

    /// Insert a key to a new encrypted value, returning and decrypting the last value if it was
    /// set.
    pub fn insert<K, V>(&self, key: K, value: V) -> EncryptedDbResult<Option<IVec>>
    where
        K: AsRef<[u8]>,
        V: Into<IVec>,
    {
        let record = self.encrypt(value)?;
        let prev_record_bytes_opt = self.kv.insert(&key, record.to_bytes()?)?;
        self.decrypt(prev_record_bytes_opt)
    }

    /// Retrieve and decrypt a value from the `Tree` if it exists.
    pub fn get<K>(&self, key: K) -> EncryptedDbResult<Option<IVec>>
    where
        K: AsRef<[u8]>,
    {
        let bytes_opt = self.kv.get(&key)?;
        self.decrypt(bytes_opt)
    }

    /// Returns `true` if the `Tree` contains a value for the specified key.
    pub fn contains_key<K>(&self, key: K) -> EncryptedDbResult<bool>
    where
        K: AsRef<[u8]>,
    {
        Ok(self.kv.contains_key(&key)?)
    }

    /// Delete a value, decrypting and returning the old value if it existed.
    pub fn remove<K>(&self, key: K) -> EncryptedDbResult<Option<IVec>>
    where
        K: AsRef<[u8]>,
    {
        let prev_val = self.kv.remove(&key)?;
        self.decrypt(prev_val)
    }

    /// Returns true if the database was recovered from a previous process.
    pub fn was_recovered(&self) -> bool {
        self.kv.was_recovered()
    }
}
