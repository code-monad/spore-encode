use std::any::Any;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{Read, stdin, stdout, Write};
use std::path::PathBuf;
use std::str::FromStr;
use clap::{arg, command, value_parser, ArgAction, Command};
use std::string::String;
use ckb_types::{core, h256, prelude::*, H256};
use clap::ValueHint::FilePath;
use spore_types::generated::spore_types::{Bytes32, ClusterData, ClusterDataBuilder};
use molecule::prelude::*;
use ckb_types::bytes::Bytes;
use blake2b_rs::Blake2bBuilder;
use ckb_types::core::TransactionView;
use ckb_types::packed::{Byte32, Byte32Builder, CellInput, CellInputBuilder, OutPoint, Uint32, Uint64};


fn main() {
    let matches = command!()
        .subcommand(
            Command::new("cluster")
                .about("create a new cluster binary")
        )
        .subcommand(
            Command::new("spore")
                .about("create a new spore binary from file")
        )
        .subcommand(
            Command::new("type-id")
                .about("Helps you to calculate your type_id")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("spore", sub_m)) => {
            // Create spore from file
            print!("content-type of nft: ");
            let _ = stdout().flush();
            let mut content_type = String::new();
            stdin().read_line(&mut content_type).expect("Error while reading content-type!");
            let content_type = content_type.trim_end();
            spore_utils::MIME::parse(content_type.as_bytes().into()).expect("Not a valid content-type!");
            print!("file path of nft: ");
            let _ = stdout().flush();
            let mut f_path = String::new();
            stdin().read_line(&mut f_path).expect("Error while read path string!");
            let f_path = f_path.trim_end();
            let mut f = File::open(&f_path).expect("file not found!");
            let mut buffer = vec![0; f.metadata().expect("Unable to read metadata").len() as usize];
            f.read(&mut buffer).expect("buffer overflow");


            print!("cluster id of nft: ");
            let _ = stdout().flush();
            let mut cluster_id = String::new();
            stdin().read_line(&mut cluster_id).expect("Error while reading cluster_id!");
            let cluster_id = Some(hex::decode(cluster_id.trim_end().trim_start_matches("0x")).expect("Error cluster id!"));
            let data = spore_types::NativeNFTData {
                content_type: content_type.to_string(),
                content: buffer.clone(),
                cluster: cluster_id
            };

            println!("Please confirm the information:");
            println!("Spore content-type: {}", content_type);
            println!("Spore data_hash: {}", hex::encode(buffer.as_slice()));
            println!("Okay?(Y/n)");
            let mut op = String::new();
            stdin().read_line(&mut op).unwrap();
            let op = op.trim_end();
            match op.to_ascii_lowercase().as_str() {
                "n" => {
                    println!("Cancelled! Exit...");
                    std::process::exit(0);
                }
                _ => {
                    println!("Encoding...");
                    println!("Enter data save path: ");
                    let mut file_path = String::new();
                    stdin().read_line(&mut file_path).expect("Not a valid string");
                    let file_path = file_path.trim_end();
                    fs::create_dir_all(PathBuf::from(file_path.clone()).parent().expect("Failed to get prefix")).expect("Failed to create parent dir");
                    let mut output = File::create(file_path.clone()).expect(format!("Failed to create {}, please check!", file_path).as_str());
                    output.write_all(spore_types::generated::spore_types::SporeData::from(data).as_slice()).expect("Failed to write file!");
                }
            }


        },
        Some(("cluster", sub_m)) => {
            // Create cluster
            print!("Name of cluster: ");
            let _ = stdout().flush();
            let mut name = String::new();
            stdin().read_line(&mut name).expect("Error while reading name!");
            let name = name.trim_end();
            if name.is_empty() {
                println!("Empty name!");
                std::process::exit(-1);
            }
            println!("Enter description: (Multi line supported, use ^D to finish)");
            let mut description = Vec::new();
            stdin().read_to_end(&mut description).expect("Error while reading description!");
            let description = String::from_utf8(description).expect("not a valid utf-8 string!");
            let description = description.trim_end();

            println!("Please confirm the information:");
            println!("Cluster name: {}", name);
            println!("Cluster description: {}", description);
            println!("Okay?(Y/n)");
            let mut op = String::new();
            stdin().read_line(&mut op).expect("Error while reading description!");
            let op = op.trim_end();
            match op.to_ascii_lowercase().as_str() {
                "n" => {
                    println!("Cancelled! Exit...");
                    std::process::exit(0);
                }
                _ => {
                    println!("Encoding...");
                    let data = ClusterData::new_builder().
                        name(name.as_bytes().into()).
                        description(description.as_bytes().into()).build();
                    let code_hash = hex::encode(data.as_slice());
                    println!("Data hash: 0x{}", code_hash);
                    println!("Enter data save path: ");
                    let mut file_path = String::new();
                    stdin().read_line(&mut file_path).expect("Not a valid string");
                    let file_path = file_path.trim_end();
                    std::fs::create_dir_all(PathBuf::from(file_path.clone()).parent().expect("Failed to get prefix")).expect("Failed to create parent dir");
                    let mut output = File::create(file_path.clone()).expect(format!("Failed to create {}, please check!", file_path).as_str());
                    output.write_all(data.as_slice()).expect("Failed to write file!");
                }
            }
        }
        Some(("type-id", sub_m)) => {
            print!("tx hash: ");
            let _ = stdout().flush();
            let mut tx_hash = String::new();
            stdin().read_line(&mut tx_hash).expect("Failed to read tx hash!");

            let tx_hash = hex::decode(
                tx_hash.trim_end().trim_start_matches("0x")
            ).expect("Failed to parse tx hash string!");
            print!("index of your previous_output: 0x");
            let _ = stdout().flush();
            let mut input_index = String::new();
            stdin().read_line(&mut input_index).expect("Failed to read tx hash!");

            let input_index = input_index.trim_end().parse::<u32>().expect(format!("Invalid input index {}!", input_index).as_str());
            let packed_data = CellInput::new_builder()
                .since(Uint64::default())
                .previous_output(
                OutPoint::new_builder()
                    .tx_hash(Byte32::from_slice(tx_hash.as_slice()).expect("parse tx hash"))
                    .index(input_index.pack())
                    .build()
            )
                .build();

            print!("index of your output: 0x");
            let _ = stdout().flush();
            let mut output_index = String::new();
            stdin().read_line(&mut output_index).expect("Failed to read tx hash!");

            let output_index = output_index.trim_end().parse::<usize>().expect(format!("Invalid output index {}!", output_index).as_str());


            println!("Input: {:?}, index: {:?}", packed_data.as_slice(), output_index);

            let mut blake2b = Blake2bBuilder::new(32)
                .personal(b"ckb-default-hash")
                .build();
            blake2b.update(packed_data.as_slice());
            blake2b.update(&(output_index as u64).to_le_bytes());
            let mut type_id = [0; 32];
            blake2b.finalize(&mut type_id);

            println!("Your type id is: 0x{}", hex::encode(type_id));


        }
        _ => {
            println!("You must specify an operation!");
            std::process::exit(-1);
        }
    }
}
