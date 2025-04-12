import React from "react";
import { Link } from "react-router-dom";

interface Props {
    
}

const Completed: React.FC<Props> = ({  }) => {
    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
            <h1 className="text-4xl font-bold mb-4">Session Completed!</h1>
            <p className="text-lg mb-8">Congratulations on completing the session!</p>
            <div className="flex space-x-4">
                <Link
                    to="/"
                    className="bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded"
                >
                    Go to Home
                </Link>
            </div>
        </div>
    );
};

export default Completed;