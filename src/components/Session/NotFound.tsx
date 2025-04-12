import { Link } from "react-router-dom";
import { AlertCircle } from "lucide-react";

const NotFound: React.FC = () => {
    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
            <AlertCircle className="w-16 h-16 text-red-500 mb-4" />
            <h1 className="text-4xl font-bold mb-4">Session Not Found</h1>
            <p className="text-lg mb-8">The session you are looking for does not exist or is no longer active.</p>
            <Link
                to="/"
                className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            >
                Go to Home
            </Link>
        </div>
    );
};

export default NotFound;