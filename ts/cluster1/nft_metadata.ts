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

        const image = await readFile("/home/user_nuel21/Q3_25_Builder_Onana/solana-starter/ts/cluster1/asset/generug.png");
        const genericFile = createGenericFile(image, "image/png");
        const metadata = {
            name: "SONA",
            symbol: "SoNa",
            description: "onchain agent",
            image: genericFile,
            attributes: [
                {trait_type: 'Background', value: 'Yellow'}
           ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://gateway.irys.xyz/AaXAGXomN8kgCkRE3WNtAMUcSBxTV585gdFZpwgGREiV"
                    }
                ]
            },
            creators: []
        };
         // Upload the metadata and get the URI
         const myUri: string = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
