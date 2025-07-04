import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../turbin3-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);
const metadataUri = "https://gateway.irys.xyz/3NgzhV4B9e1aM9NqhrCGumr1MVvCAFgdg3wvNS4hUJJH";
(async () => {
     let tx = createNft(umi, {
        mint,
        authority: myKeypairSigner,
        name: "SONA",
        uri: metadataUri,
        sellerFeeBasisPoints: percentAmount(5, 2), // 5%
        symbol: "NFT",
        isMutable: true,
        creators: [
            { address: myKeypairSigner.publicKey, verified: true, share: 100 }
        ]
    });

    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);

    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();