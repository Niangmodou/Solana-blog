import { WalletName } from "@solana/wallet-adapter-wallets"
import { useWallet } from "@solana/wallet-adapter-react"
import init_blog from "./init_blog"

const Home = () => {
    const { select } = useWallet()

    const onConnect = () => {
        select()
    }

    return (
        <div>
            <init_blog />
            <button onClick={onConnect}>Connect With Phantom</button>
        </div>
    )
}

export default Home
