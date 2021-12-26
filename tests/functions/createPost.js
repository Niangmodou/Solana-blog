const anchor = require("@project-serum/anchor")
const { SystemProgram } = anchor.web3

const createPost = async (program, provider, blogAccount, userAccount) => {
    const postAccount = anchor.web3.Keypair.generate()  
    const title = "Post Title"
    const content = "Post Content"

    await program.rpc.createPost(title, content, {
        accounts: {
            blogAccount: blogAccount.publicKey,
            authority: provider.wallet.publicKey,
            userAccount: userAccount.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [postAccount],
    })

    const post = await program.account.postState.fetch(postAccount.publicKey)

    return { post, postAccount, title, content }
}

module.exports = {
    createPost
}