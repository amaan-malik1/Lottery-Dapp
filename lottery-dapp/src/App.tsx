import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react"
import { WalletDisconnectButton, WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui";

// Default styles that can be overridden by your app
import '@solana/wallet-adapter-react-ui/styles.css';
import Navbar from "./components/Navbar";

function App() {
  const RPC = import.meta.env.VITE_RPC_URL;

  // const {authUser, isLoading} = useAuthUser();

  return (
    <div>
      <ConnectionProvider endpoint={RPC}>
        <WalletProvider wallets={[]} autoConnect>
          <WalletModalProvider>
            {/* navbar */}
            <div className="w-full flex justify-between items-center px-4 py-2 bg-slate-600">
              <Navbar />
              {/* connect and disconnect button */}
              <div className="flex items-center gap-2">
                <WalletMultiButton />
                <WalletDisconnectButton />
              </div>
            </div>
            {/* other componenets here */}
            <div></div>
          </WalletModalProvider>

        </WalletProvider>

      </ConnectionProvider>

    </div>
  )
}

export default App
