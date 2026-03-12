import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react"
import { WalletDisconnectButton, WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui";

function App() {
  const RPC = import.meta.env.VITE_RPC_URL;

  // const {authUser, isLoading} = useAuthUser();

  return (
    <div>
      <ConnectionProvider endpoint={RPC}>
        <WalletProvider wallets={[]} autoConnect>
          <WalletModalProvider>
            {/* connect and disconnect button */}
            <div className="flex justify-between items-center ">
              <div className="flex justify-evenly gap-4 items-center w-full p-4">
                <WalletMultiButton />
                <WalletDisconnectButton />
              </div>
              {/* other componenets here */}
              <div></div>

            </div>
          </WalletModalProvider>

        </WalletProvider>

      </ConnectionProvider>

    </div>
  )
}

export default App
