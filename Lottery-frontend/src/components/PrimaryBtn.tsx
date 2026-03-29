const PrimaryBtn = ({ onClick, value }: {
    onClick: () => void;
    value: string;
}) => {
    return (
        <button
            className=" px-6 py-2 rounded-2xl
                bg-white/10 backdrop-blur-md
                border border-white/20
                text-white font-medium
                 bg-linear-to-r from-black to-black/90
                shadow-lg shadow-black/20
                
                transition-all duration-300 ease-in-out
                
                hover:bg-white/20
                hover:shadow-xl hover:shadow-black/30
                hover:scale-[1.03]
                
                active:scale-[0.97]"
            onClick={onClick}
        >
            {value}
        </button>
    )
}

export default PrimaryBtn