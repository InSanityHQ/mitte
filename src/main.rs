mod error;
mod agent;

pub use agent::*;

mod debug;

fn main() {
    let mut agent = Agent::new("0.0.0.0:9522", "chickens").unwrap();
    println!("{:?}", agent.profile.serialize());
    
    let serialized = [1, 0, 0, 0, 0, 49, 37, 64, 0, 0, 0, 0, 0, 0, 0, 35, 175, 48, 113, 245, 1, 53, 1, 199, 90, 77, 113, 218, 129, 174, 117, 143, 122, 188, 209, 80, 190, 62, 126, 2, 189, 204, 18, 140, 81, 190, 39, 115, 43, 222, 209, 179, 186, 121, 20, 191, 51, 97, 5, 11, 212, 60, 208, 91, 28, 21, 177, 65, 191, 175, 200, 240, 198, 50, 153, 117, 165, 218, 151, 254, 67, 112, 49, 103, 166, 253, 87, 52, 246, 102, 26, 169, 37, 98, 238, 152, 34, 63, 15, 92, 58, 71, 186, 6, 107, 75, 220, 19, 27, 21, 84, 131, 168, 116, 65, 11, 128, 73, 192, 145, 30, 62, 80, 194, 240, 111, 150, 242, 55, 124, 225, 88, 178, 0, 235, 11, 158, 227, 114, 150, 223, 249, 171, 95, 202, 206, 156, 0, 13, 244, 102, 161, 83, 32, 71, 181, 252, 127, 0, 204, 11, 14, 66, 48, 222, 222, 99, 78, 147, 164, 170, 85, 6, 82, 35, 10, 222, 152, 235, 65, 136, 221, 6, 35, 3, 118, 53, 71, 230, 119, 218, 23, 120, 38, 29, 65, 223, 7, 81, 86, 36, 6, 44, 182, 137, 41, 33, 158, 95, 110, 57, 190, 0, 172, 216, 120, 221, 132, 2, 150, 162, 141, 229, 143, 22, 229, 195, 107, 229, 104, 225, 37, 141, 212, 186, 253, 149, 224, 235, 25, 0, 142, 143, 6, 103, 179, 53, 231, 17, 243, 137, 43, 37, 218, 111, 213, 245, 252, 189, 68, 52, 78, 80, 234, 192, 237, 20, 8, 146, 127, 199, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 8, 0, 0, 0, 0, 0, 0, 0, 99, 104, 105, 99, 107, 101, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let desc = AgentDescription::deserialize(&serialized);
    agent.handshake(&desc).unwrap();

                // we listen for practically forever
    // agent.listen(10000).unwrap();
    println!("{:?}", agent)


    // let serialized = agent.profile.serialize();


    // let priv_key:RsaPrivateKey = bincode::deserialize(&debug::_KEY_PRIV).unwrap();
    // let pub_key:RsaPublicKey = bincode::deserialize(&debug::_KEY_PUB).unwrap();
     
    // let desc = AgentDescription::new("0.0.0.0:8301", "TestAgent")
    //                             .expect("error! name is probably too long");

    // let ag = Agent::new("0.0.0.0:8381", "TestAgent");
    // println!("{:?}", ag);
}

