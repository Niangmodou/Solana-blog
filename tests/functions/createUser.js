const achor = require("@project-serum/anchor")
const {SystemProgram } = anchor.web3

const createUser = async (program, provider) => {
    const userAccount = anchor.web3.Keypair.generate()

    const name = "User Name"
    const avatar = "https://img.link"

    await program.rpc.signupUser(name, avatar, {
        accounts: {
            authority: provider.wallet.publicKey,
            userAccount: userAccount.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [userAccount],
    })

    const user = await program.account.userState.fetch(userAccount.publicKey)

    return { user, userAccount, name, avatar }
}

module.exports = {
    createUser,
}