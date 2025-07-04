import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises";

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://gateway.irys.xyz/Byn7hVVtnaYXa7675yWPSkgFqeQj775B8vLN3cgZZZHq";
        const metadata = {
            name: "SONA",
            symbol: "SoNa",
            description: "onchain agent",
            image: image,
            attributes: [
                {trait_type: 'Background', value: 'Yellow'}
           ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    }
                ]
            },
            creators: [
                {
                    address: signer.publicKey,
                    share: 100,
                    verified: true
                }
            ]
        };
         // Upload the metadata and get the URI
         const myUri: string = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
