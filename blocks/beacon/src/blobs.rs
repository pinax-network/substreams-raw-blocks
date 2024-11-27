use common::utils::bytes_to_hex;

use crate::{
    pb::{pinax::beacon::v1::Blob as RawBlob, sf::beacon::r#type::v1::Blob},
    structs::BlockTimestamp,
    utils::encode_hex_2d_array,
};

pub fn collect_blobs(blobs: &Vec<Blob>, timestamp: &BlockTimestamp) -> Vec<RawBlob> {
    let mut vec = Vec::<RawBlob>::new();

    for b in blobs {
        vec.push(RawBlob {
            block_time: timestamp.time.to_string(),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: b.index,
            blob: bytes_to_hex(&b.blob),
            kzg_commitment: bytes_to_hex(&b.kzg_commitment),
            kzg_proof: bytes_to_hex(&b.kzg_proof),
            kzg_commitment_inclusion_proof: encode_hex_2d_array(&b.kzg_commitment_inclusion_proof),
        });
    }

    vec
}
