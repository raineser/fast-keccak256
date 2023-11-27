use fast_keccak256::keccak256;
fn main () {
    
    let message = b"hello world"; 

    let output = keccak256(message);

    println!("Hash of hello world {:?}", hex::encode(output));

    let message = b""; 

    let output = keccak256(message);

    println!("Hash of empty string {:?}", hex::encode(output));

    let message:[u8;137] = [97;137]; //137 bytes to testing pad

    let output = keccak256(&message);

    println!("Hash of a*137 {:?}", hex::encode(output));

}