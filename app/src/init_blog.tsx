import { Program, Provider } from "@project-serum/anchor"
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react"
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js"
import idl from "./idl.json"

const PROGRAM_KEY = new PublicKey(idl.metadata.address)
// const BLOG_KEY = new PublicKey("")

const initBlog = async (walletKey, provider) => {
    const program = new Program(idl, PROGRAM_KEY, provider)
    const blogAccount = Keypair.generate()
    const genesisPostAccount = Keypair.generate()

    await program.rpc.initBlog({
        accounts: {
            authority: walletKey,
            systemProgram: SystemProgram.programId,
            blogAccount: blogAccount.publicKey,
            genesisPostAccount: genesisPostAccount.publicKey
        },
        signers: [blogAccount, genesisPostAccount],
    })

    console.log("Blog Pubkey: ", blogAccount.publicKey.toString())
}

const { connection } = useConnection()
const wallet = useAnchorWallet()

const _initBlog = () => {
    const provider = new Provider(connection, wallet, {})
    initBlog(provider.wallet.publicKey, provider)
}

const init_blog = () => {
    return (
        <button onClick={_initBlog}>
            Init Blog
        </button>
    )
}