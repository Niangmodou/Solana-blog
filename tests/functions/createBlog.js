const anchor = require("@project-serum/anchor")

const { SystemProgram } = anchor.web3;

const createBlog = async (program, provider) => {
    // Random KeyPair generation
    const blogAccount = anchor.web3.Keypair.generate()
    const genesisPostAccount = anchor.web3.Keypair.generate()

    await program.rpc.initBlog({
        accounts: {
            authority: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
            blogAccount: initBlogAccount.publicKey,
            genesisPostAccount: genesisPostAccount.publicKey,
        },
        signers: [initBlogAccount, genesisPostAccount],
    })

    const blog = await program.account.blogState.fetch(initBlogAccount.publicKey)

    return { blog, blogAccount, genesisPostAccount }
}

module.export = {
    createBlog,
}