
interface Props {
    
}

const Background: React.FC<Props> = () => {

    return <div className="absolute w-full h-full -z-10 bg-black">
        <img src="./landscape.webp" alt="Background" className="w-full h-full opacity-20" />
    </div>
};

export default Background;