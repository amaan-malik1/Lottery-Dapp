import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react"
import { WalletDisconnectButton, WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui";

function App() {
  const RPC = import.meta.env.VITE_RPC_URL;

  const {authUser, isLoading} = useAuthUser();

  return (
    <div>
      <ConnectionProvider endpoint={RPC}>
        <WalletProvider wallets={[]} autoConnect>
          <WalletModalProvider>
            {/* connect and disconnect button */}
            <div className="flex justify-between items-center ">
              <WalletMultiButton />
              <WalletDisconnectButton />
              {/* other componenets here */}
              <div>

              </div>

            </div>
          </WalletModalProvider>

        </WalletProvider>

      </ConnectionProvider>

    </div>
  )
}

export default App
