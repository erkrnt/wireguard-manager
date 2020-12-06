struct Interface {
    ID: String,
    Address: String,
    ListenPort: u8,
    PostUp: String,
    PostDown: String,
    SaveConfig: bool,
}

struct Peer {
    ID: String,
    Address: String,
    InterfaceID: String,
    PrivateKey: String,
    PublicKey: String,
    UserID: String,
}

struct User {
    ID: String,
    Name: String,
}
