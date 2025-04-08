import { Home, ChevronRight } from "lucide-react";
import { Link } from "react-router-dom";

const Breadcrumbs = ({ current }: { current: string }) => {
    return (
        <nav className="w-full p-4">
            <div className="flex items-center space-x-2 text-gray-300">
                <Link to="/" className="hover:text-white flex items-center">
                    <Home className="w-5 h-5 mr-1" />
                </Link>
                <ChevronRight className="w-4 h-4" />
                <span className="text-gray-400">{current}</span>
            </div>
        </nav>
    );
};

export default Breadcrumbs;