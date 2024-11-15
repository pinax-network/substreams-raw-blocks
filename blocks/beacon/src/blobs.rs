use common::utils::bytes_to_hex;

use crate::{
    pb::{beacon::Blob as RawBlob, sf::beacon::r#type::v1::Blob},
    structs::BlockTimestamp,
    utils::encode_2d_array_to_csv_string,
};

pub fn collect_blobs(blobs: &Vec<Blob>, timestamp: &BlockTimestamp) -> Vec<RawBlob> {
    let mut vec = Vec::<RawBlob>::new();

    for b in blobs {
        vec.push(RawBlob {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: b.index,
            blob: bytes_to_hex(&b.blob),
            kzg_commitment: bytes_to_hex(&b.kzg_commitment),
            kzg_proof: bytes_to_hex(&b.kzg_proof),
            // TODO: use encode_hex_2d_array once Array(Text) is supported
            kzg_commitment_inclusion_proof: encode_2d_array_to_csv_string(&b.kzg_commitment_inclusion_proof),
        });
    }

    vec
}
