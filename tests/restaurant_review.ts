import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RestaurantReview } from "../target/types/restaurant_review";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { expect } from "chai";

describe("restaurant_review", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.restaurantReview as Program<RestaurantReview>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  it("add review", async () => {
    // Add your test here.
    const review = {
        title: "add",
        rating: 5,
        description: "add a new review"
    };
    const user = provider.wallet.publicKey;
    const [reviewPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("review"), user.toBuffer()],
        program.programId,
    );
    const tx = await program.methods
        .addReview(review)
        .accounts({
            signer: user,
            reviewContent: reviewPda,
            SystemProgram: SystemProgram.programId,
        })
        .rpc();
    console.log("Your transaction signature", tx);

    const reviewAccount = await program.account.reviewContent.fetch(reviewPda);

    expect(reviewAccount.title).to.equal(review.title);
    expect(reviewAccount.rating).to.equal(review.rating);
    expect(reviewAccount.description).to.equal(review.description);
  });

  it("update review", async () => {
    const review = {
        title: "update",
        rating: 3,
        description: "update a review",
    };

    const user = provider.wallet.publicKey;
    const [reviewPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("review"), user.toBuffer()],
        program.programId,
    );
    const tx = await program.methods
        .updateReview(review)
        .accounts({
            signer: user,
            reviewContent: reviewPda,
        })
        .rpc();
    console.log("Your transaction signature", tx);

    const reviewAccount = await program.account.reviewContent.fetch(reviewPda);

    expect(reviewAccount.title).to.equal(review.title);
    expect(reviewAccount.rating).to.equal(review.rating);
    expect(reviewAccount.description).to.equal(review.description);
  });

  it("delete review", async () => {
    const user = provider.wallet.publicKey;
    const [reviewPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("review"), user.toBuffer()],
        program.programId,
    );
    const tx = await program.methods
        .deleteReview()
        .accounts({
            signer: user,
            reviewContent: reviewPda,
        })
        .rpc();
    console.log("Your transaction signature", tx);
  });
});
