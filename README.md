# spore-encode

This is a cli tool for encoding files into molecule binary follows the Spore Protocol.

## Usage

This tool has 3 modes: `spore`, `cluster` and `type-id`

### `spore`
This is for encoding files into spore binary.
Run `cargo run -- spore`, input your `content-type` (For example, "image/png"), and file path, define your cluster id if you want it be inside a specified cluster.
Finally, input the output path.

### `cluster`
This is for encoding files into cluster binary.
Run `cargo run -- cluster`, input your cluster name, description, and then select the output path

### `type-id`
This is for calculate type-id for spore/cluster.
Run `cargo run -- type-id`.
First input the first input cell's hash(for example, `0x22b5cad9a6eed5745bcbfd68435f3a0514f2da94e5c53462c813c92386cd2320`), then input the output index of the input cell in the original tx,
finally input the output index of your spore/cluster.
You will get a 0x started type-id, you can use this as the Type Script Args while handful minting spore/cluster
